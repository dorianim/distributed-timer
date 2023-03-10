<script lang="ts">
	import { ProgressRadial, SlideToggle } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import { API_URL } from '../../../stores';
	import type { Timer, TimerUpdateRequest } from '../../../types/timer';
	import type { PageData } from './$types';
	import TimerForm from './TimerForm.svelte';
	import Fa from 'svelte-fa';
	import { faClose, faCircleCheck, faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';

	export let data: PageData;
	let { timerData } = data;
	let submitResult: Promise<Timer | void> = Promise.resolve();

	$: {
		if (browser && !localStorage.getItem('token')) goto('/manage/login');
	}

	const onSubmit = async (newTimerData: TimerUpdateRequest) => {
		submitResult = fetch(`${get(API_URL)}/timer/${data.params.id}`, {
			method: 'PUT',
			body: JSON.stringify(newTimerData),
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${localStorage.getItem('token')}`
			}
		})
			.then((res) => {
				if (!res.ok) {
					throw new Error(res.statusText);
				}
				return res.json();
			})
			.then((timer: Timer) => {
				timerData = timer;
				return timer;
			});
	};
</script>

<h2 class="text-center">Manage timer <strong>{timerData.id}</strong></h2>

<div class=" m-auto w-full items-center">
	{#await submitResult}
		<div class="flex items-center justify-center">
			<ProgressRadial class="w-10" />
		</div>
	{:then result}
		{#if result}
			<aside class="alert variant-ghost-success">
				<Fa icon={faCircleCheck} class="text-2xl" />
				<h3 class="alert-message">Success</h3>
				<button
					class="btn-icon"
					on:click={() => {
						submitResult = Promise.resolve();
					}}><Fa icon={faClose} /></button
				>
			</aside>
		{/if}
		<TimerForm {timerData} {onSubmit} />
	{:catch error}
		<div class="alert variant-ghost-error">
			<Fa icon={faCircleExclamation} class="text-2xl" />
			<div class="alert-message">
				<h3>Error</h3>
				<p>{error}</p>
			</div>
			<button
				class="btn-icon"
				on:click={() => {
					submitResult = Promise.resolve();
				}}><Fa icon={faClose} /></button
			>
		</div>
		<TimerForm {timerData} {onSubmit} />
	{/await}
</div>

<div class="m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a>,
	<a href="/manage/login">manage a diffrent existing timer</a> or
	<a href="/manage/create">create a new timer</a>
</div>

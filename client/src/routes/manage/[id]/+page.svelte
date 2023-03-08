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
	let submitResult: Promise<Timer> | undefined;

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

<div class="p-4 m-auto items-center">
	{#await submitResult}
		<div class="flex items-center justify-center">
			<ProgressRadial class="w-10" />
		</div>
	{:then result}
		{#if result}
			<aside class="alert variant-ghost-success mb-4 p-2 pl-4">
				<Fa icon={faCircleCheck} class="text-2xl" />
				<h3 class="alert-message">Success</h3>
				<button
					class="btn-icon"
					on:click={() => {
						submitResult = undefined;
					}}><Fa icon={faClose} /></button
				>
			</aside>
		{/if}
		<TimerForm {timerData} {onSubmit} />
	{:catch error}
		<aside class="alert variant-ghost-error mb-4 p-2 pl-4">
			<Fa icon={faCircleExclamation} class="text-2xl" />
			<h3 class="alert-message">Error: {error}</h3>
			<button
				class="btn-icon"
				on:click={() => {
					submitResult = undefined;
				}}><Fa icon={faClose} /></button
			>
		</aside>
		<TimerForm {timerData} {onSubmit} />
	{/await}
</div>

<div class="m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a> or
	<a href="/manage/login">manage an existing timer</a>
</div>

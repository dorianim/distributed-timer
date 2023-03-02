<script lang="ts">
	import { ProgressRadial, SlideToggle } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import { API_URL } from '../../../stores';
	import type { Segment } from '../../../types/segment';
	import type { Timer, TimerCreationRequest } from '../../../types/timer';
	import type { PageData } from './$types';
	import TimerForm from './TimerForm.svelte';
	import Fa from 'svelte-fa';
	import { faClose, faCircleCheck, faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import type TimerFormData from './TimerForm.svelte';

	export let data: PageData;
	let { timerData } = data;
	let submitResult: Promise<Response> | undefined;
	let useCurrentTime: boolean = true;
	let start_at = new Date(timerData.start_at).toISOString().slice(0, 16);

	$: {
		console.log(timerData);
	}

	$: {
		if (useCurrentTime) start_at = new Date().toISOString().slice(0, 16);
	}

	const onSubmit = async (newTimerData: Timer) => {
		timerData = newTimerData;

		submitResult = fetch(`${get(API_URL)}/api/timer/${data.params.id}`, {
			method: 'PUT',
			body: JSON.stringify({ password: 'test', ...newTimerData }),
			headers: {
				'Content-Type': 'application/json'
			}
		}).then((res) => {
			if (res.status === 200) {
				return res;
			} else {
				throw new Error('Something went wrong');
			}
		});
	};
</script>

<h2 class="text-center">Manage timer <strong>{timerData.id}</strong> ({timerData.name})</h2>

<div class="p-4 md:w-[80%] m-auto items-center">
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

<div class="md:w-[50%] m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a> or
	<a href="/manage/login">manage an existing timer</a>
</div>

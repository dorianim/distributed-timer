<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import { API_URL } from '../../../stores';
	import type { Timer, TimerCreationRequest, TimerCreationResponse } from '../../../types/timer';
	import type { PageData } from './$types';
	import CreateTimerForm from './CreateTimerForm.svelte';
	import Fa from 'svelte-fa';
	import { faClose, faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { goto } from '$app/navigation';
	import { createTimer } from 'utils/api';

	export let data: PageData;
	const { fetch } = data;
	let submitResult: Promise<TimerCreationResponse | void> = Promise.resolve();

	const onSubmit = (timerData: TimerCreationRequest) => {
		submitResult = createTimer(timerData, fetch).then((data) => {
			console.log(data);
			localStorage.setItem('token', data.token);
			goto(`/manage/${data.timer.id}`);
			return data;
		});
	};
</script>

<h2 class="text-center">Create timer</h2>

<div class="w-full md:w-[70%] lg:w-[50%] m-auto items-center">
	{#await submitResult}
		<div class="flex items-center justify-center">
			<ProgressRadial class="w-10" />
		</div>
	{:then result}
		{#if result}
			<div class="flex items-center justify-center">
				<ProgressRadial class="w-10" />
			</div>
		{:else}
			<CreateTimerForm {onSubmit} />
		{/if}
	{:catch error}
		<aside class="alert variant-ghost-error">
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
		</aside>
		<CreateTimerForm {onSubmit} />
	{/await}
</div>

<div class="md:w-[50%] m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a> or
	<a href="/manage/login">manage an existing timer</a>
</div>

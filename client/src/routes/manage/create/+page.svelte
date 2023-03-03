<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import { API_URL } from '../../../stores';
	import type { TimerCreationRequest } from '../../../types/timer';
	import type { PageData } from './$types';
	import CreateTimerForm from './CreateTimerForm.svelte';
	import Fa from 'svelte-fa';
	import { faClose, faCircleCheck, faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import { goto } from '$app/navigation';

	export let data: PageData;
	const { fetch } = data;
	let submitResult: Promise<string> | undefined;

	const onSubmit = (timerData: TimerCreationRequest) => {
		submitResult = fetch(`${get(API_URL)}/api/timer`, {
			method: 'POST',
			body: JSON.stringify(timerData),
			headers: {
				'Content-Type': 'application/json'
			}
		})
			.then((res) => res.json())
			.then((data) => {
				console.log(data);
				return data.id;
			});
	};
</script>

<h2 class="text-center">Create timer</h2>

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
					class="btn variant-filled-success"
					on:click={() => {
						goto(`/manage/${result}`);
					}}>Manage timer</button
				>
			</aside>
		{:else}
			<CreateTimerForm {onSubmit} />
		{/if}
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
		<CreateTimerForm {onSubmit} />
	{/await}
</div>

<div class="md:w-[50%] m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a> or
	<a href="/manage/login">manage an existing timer</a>
</div>
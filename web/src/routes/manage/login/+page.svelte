<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import type { PageData } from './$types';
	import Fa from 'svelte-fa';
	import { faClose, faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import LoginForm from './LoginForm.svelte';
	import { goto } from '$app/navigation';
	import { loginTimer } from 'utils/api';

	export let data: PageData;
	const { fetch } = data;
	let submitResult: Promise<string | void> = Promise.resolve();

	const onSubmit = async (id: string, password: string) => {
		submitResult = loginTimer(id, password, fetch).then((data) => {
			localStorage.setItem('token', data.token);
			goto(`/manage/${id}`);
			return data.token;
		});
	};
</script>

<h2 class="text-center">Manage existing timer</h2>

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
			<LoginForm {onSubmit} />
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

		<LoginForm {onSubmit} />
	{/await}
</div>

<div class="md:w-[50%] m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a> or
	<a href="/manage/create">create a new timer</a>
</div>

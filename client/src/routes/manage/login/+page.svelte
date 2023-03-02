<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import type { PageData } from './$types';
	import Fa from 'svelte-fa';
	import { faClose, faCircleExclamation } from '@fortawesome/free-solid-svg-icons';
	import LoginForm from './LoginForm.svelte';

	export let data: PageData;
	const { fetch } = data;
	let submitResult: Promise<string> | undefined;

	const onSubmit = async (name: string, password: string) => {
		submitResult = fetch('/api/token', {
			method: 'POST',
			body: JSON.stringify({
				name,
				password
			})
		})
			.then((res) => res.json())
			.then((data) => {
				console.log(data);
				return data.id;
			});

		await submitResult;
	};
</script>

<h2 class="text-center">Manage existing timer</h2>

<div class="p-4 md:w-[80%] m-auto items-center">
	{#await submitResult}
		<div class="flex items-center justify-center">
			<ProgressRadial class="w-10" />
		</div>
	{:then result}
		<LoginForm {onSubmit} />
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

		<LoginForm {onSubmit} />
	{/await}
</div>

<div class="md:w-[50%] m-auto text-center">
	Alternatively, you can <a href="/">view an existing timer</a> or
	<a href="/manage/create">create a new timer</a>
</div>

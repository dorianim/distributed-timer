<script lang="ts">
	import { get } from 'svelte/store';
	import { API_URL } from '../../../stores';
	import type { TimerCreationRequest } from '../../../types/timer';
	import type { PageData } from './$types';

	export let data: PageData;
	let result: Promise<string | void>;
	const { fetch } = data;

	const createTimer = (e: Event) => {
		e.preventDefault();
		if (!e.target) return Promise.reject('');
		const form = new FormData(e.target as HTMLFormElement);

		const timer: TimerCreationRequest = {
			name: form.get('name') as string,
			password: form.get('password') as string,
			repeat: true,
			start_at: new Date().getTime(),
			segments: [
				{
					label: 'Change',
					time: 15000,
					sound: false
				},
				{
					label: 'Boulder',
					time: 240000,
					sound: true
				}
			]
		};

		result = fetch(`${get(API_URL)}/api/timer`, {
			method: 'POST',
			body: JSON.stringify(timer),
			headers: {
				'Content-Type': 'application/json'
			}
		})
			.then((res) => res.json())
			.then((data) => {
				console.log(data);
			});
	};
</script>

<div class="m-auto items-center grid gap-10 pt-10 p-4">
	<h2 class="text-center">Create timer</h2>

	<div class="card p-4 md:w-[50%] m-auto">
		{#await result}
			<div>loading...</div>
		{:then result}
			<div>res: {result}</div>
		{:catch error}
			<div>err: {error}</div>
		{/await}
		<form class="grid gap-3" on:submit={createTimer}>
			<label class="label">
				<span>Enter a name for your timer:</span>
				<input
					class="input variant-form-material"
					type="text"
					placeholder="timer name"
					required
					pattern="[a-z0-9-_]*"
					name="name"
				/>
			</label>

			<label class="label">
				<span>Enter a password to access your timer later:</span>
				<input
					class="input variant-form-material"
					type="password"
					placeholder="timer password"
					name="password"
					required
				/>
			</label>

			<button class="btn variant-filled-primary">Submit</button>
		</form>
	</div>

	<div class="md:w-[50%] m-auto text-center">
		Alternatively, you can <a href="/">view an existing timer</a> or
		<a href="/manage/login">manage an existing timer</a>
	</div>
</div>

<script lang="ts">
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import { API_URL } from '../../../stores';
	import type { Segment } from '../../../types/segment';
	import type { TimerCreationRequest } from '../../../types/timer';
	import type { PageData } from './$types';

	export let data: PageData;
	let result: Promise<string | void>;
	let useCurrentTime: boolean = true;
	let sequence: Segment[] = [];

	$: {
		console.log(sequence);
	}

	const updateTimer = async (e: Event) => {
		const timer: TimerCreationRequest = {
			name: 'test',
			password: 'test',
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

		result = fetch(`${get(API_URL)}/api/timer/${data.params.id}`, {
			method: 'PUT',
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
	<h2 class="text-center">Manage timer</h2>

	<div class="card p-4 md:w-[50%] m-auto">
		{#await result}
			<div>loading...</div>
		{:catch error}
			<div>err: {error}</div>
		{/await}

		<form class="grid gap-3" on:submit={updateTimer}>
			<label class="label">
				<strong>When to start the timer:</strong>
				<input
					class="input variant-form-material"
					type="datetime-local"
					placeholder="timer starttime"
					name="start_at"
					disabled={useCurrentTime}
					value={useCurrentTime ? new Date().toISOString().slice(0, 16) : ''}
					required
				/>
				<label class="flex items-center space-x-2"
					><input class="checkbox" type="checkbox" required bind:checked={useCurrentTime} />
					<p>Use current time</p></label
				>
			</label>

			<SlideToggle name="slider-example">repeat</SlideToggle>

			<strong>Timer sequence:</strong>

			{#each sequence as segment, i}
				<div class="card p-4 grid gap-3">
					<strong>{segment.label}</strong>
					<label class="label">
						<span>Time in milliseconds:</span>
						<input
							class="input variant-form-material"
							type="number"
							placeholder="time in milliseconds"
							name="time"
							bind:value={segment.time}
							required
						/>
					</label>
					<SlideToggle name="slider-example" bind:checked={segment.sound}>enable sound</SlideToggle>
				</div>
			{/each}

			<button
				type="button"
				class="btn variant-filled-secondary"
				on:click={() => {
					console.log('add segment');
					sequence = [
						...sequence,
						{
							label: `Segment ${sequence.length + 1}`,
							time: 15000,
							sound: false
						}
					];
				}}>Add segment</button
			>

			<button class="btn variant-filled-primary">Submit</button>
		</form>
	</div>

	<div class="md:w-[50%] m-auto text-center">
		Alternatively, you can <a href="/">view an existing timer</a> or
		<a href="/manage/login">manage an existing timer</a>
	</div>
</div>

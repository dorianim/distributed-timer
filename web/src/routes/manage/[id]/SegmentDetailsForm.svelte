<script lang="ts">
	import { faRemove } from '@fortawesome/free-solid-svg-icons';
	import { modalStore, SlideToggle } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import type { Segment } from '../../../types/segment';
	import { getTimerText } from '../../../utils/timer';
	import TimeInputField from './TimeInputField.svelte';

	export let segment: Segment;

	const onSubmit = () => {
		if ($modalStore[0].response) $modalStore[0].response(segment);
		modalStore.close();
	};
</script>

<div class="card p-4 w-modal shadow-xl space-y-4">
	<header class="text-2xl font-bold">{segment.label}</header>
	<p>
		This timer will count from {getTimerText(segment.time + segment.count_to - 1)} to {getTimerText(
			segment.count_to
		)}
	</p>
	<form class="modal-form space-y-4 rounded-container-token">
		<label class="label">
			Label:
			<input
				class="input variant-form-material"
				type="text"
				placeholder="label"
				bind:value={segment.label}
				required
			/>
		</label>

		<TimeInputField bind:time={segment.time} label="Time (mm:ss):" />

		<TimeInputField bind:time={segment.count_to} label="Count to (s):" />

		<SlideToggle active="bg-primary-500" name="sound" size="sm" bind:checked={segment.sound}
			>enable sound</SlideToggle
		>

		<label class="label">
			<div class="flex items-center">
				<input class="input" type="color" bind:value={segment.color} />
				<span class="pl-3">background-color</span>
				{#if segment.color}
					<button
						type="button"
						class="btn-icon p-0"
						on:click={() => {
							segment.color = undefined;
						}}><Fa icon={faRemove} /></button
					>
				{/if}
			</div>
		</label>
	</form>

	<button class="btn variant-filled" on:click={onSubmit}>Ok</button>
</div>

<script lang="ts">
	import { faRemove } from '@fortawesome/free-solid-svg-icons';
	import { modalStore, SlideToggle } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import type { Segment } from '../../../types/segment';
	import { getTimerText } from '../../../utils/timer';
	import TimeInputField from './TimeInputField.svelte';
	import SegmentInfoBox from './SegmentInfoBox.svelte';

	export let segment: Segment;

	const onSubmit = () => {
		if ($modalStore[0].response) $modalStore[0].response(segment);
		modalStore.close();
	};
</script>

<div class="card w-modal shadow-xl space-y-4 p-[2px]">
	<SegmentInfoBox {segment} class="rounded-t-md rounded-b-2xl justify-center" />
	<form
		class="modal-form p-4 space-y-4 rounded-container-token"
		on:submit|preventDefault={() => {}}
	>
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

		<button class="btn variant-filled" on:click={onSubmit}>Ok</button>
	</form>
</div>

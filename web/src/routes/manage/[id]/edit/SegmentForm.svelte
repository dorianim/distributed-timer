<script lang="ts">
	import { faRemove } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
	import type { Segment } from 'types/segment';
	import TimeInputField from './TimeInputField.svelte';
	import { detectSoundPreset, soundPresets } from 'utils/sounds';

	export let segment: Segment;
	let clazz: string = '';
	export { clazz as class };

	let soundPreset = detectSoundPreset(segment.sounds);

	const updateSegmentSounds = (preset: string | null) => {
		if (!preset) {
			segment.sounds = [];
			return;
		}
		segment.sounds = soundPresets[preset];
	};

	$: updateSegmentSounds(soundPreset);
</script>

<div class="p-[2px] {clazz}">
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

		<div class="flex flex-row flex-wrap gap-4 w-full">
			<TimeInputField class="flex-1" bind:time={segment.time} label="Time:" />

			<TimeInputField class="flex-1" bind:time={segment.count_to} label="Count to:" />
		</div>

		<label class="">
			Sounds to play:
			<select class="select" bind:value={soundPreset}>
				<option value={null}>No sounds</option>
				<option value="beepOneMinute_countdownFiveSeconds"
					>Beep at one minute and countdown at five seconds</option
				>
				<option value="beepFourMinutesOneMinute_countdownFiveSeconds"
					>Beep at four minutes and one minute and countdown at five seconds</option
				>
			</select>
		</label>

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
</div>

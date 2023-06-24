<script lang="ts">
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import {
		faTrash,
		faAdd,
		faSave,
		faPauseCircle,
		faArrowUp,
		faArrowDown,
		faEdit,
		faCheck,
		faArrowLeft
	} from '@fortawesome/free-solid-svg-icons';
	import type { Timer } from 'types/timer';
	import type { Segment } from 'types/segment';
	import SegmentForm from './SegmentForm.svelte';
	import HelpPopup from 'components/HelpPopup.svelte';
	import SegmentInfoBox from './SegmentInfoBox.svelte';
	import { slide } from 'svelte/transition';
	import { calculateNewStartAt, type TimerAction } from '../helpers';

	interface TimerFormData {
		start_at_date: string;
		start_at_time: string;
		repeat: boolean;
		segments: Segment[];
		display_options: {
			clock: boolean;
			run_before_started: boolean;
		};
	}

	export let timerData: Timer;
	export let onSubmit: (timerData: Timer) => void;

	let formData: TimerFormData;
	let editingSegment: number | undefined = undefined;

	const handleSubmit = (e: Event) => {
		e.preventDefault();
	};

	const handleButtonSubmit = (action: TimerAction) => {
		console.log('button submit', action);
		return () => {
			onSubmit(formDataToTimerData(formData, action));
		};
	};

	const timerDataToFormData = (timerData: Timer): TimerFormData => {
		return {
			start_at_date: new Date(timerData.start_at).toISOString().split('T')[0],
			start_at_time: new Date(timerData.start_at).toLocaleTimeString(undefined, {
				hour12: false,
				hour: '2-digit',
				minute: '2-digit',
				second: '2-digit'
			}),
			repeat: timerData.repeat,
			segments: timerData.segments,
			display_options: {
				clock: timerData.display_options.clock,
				run_before_started: timerData.display_options.pre_start_behaviour === 'RunNormally'
			}
		};
	};

	/**
	 * This function calculates the new start_at
	 * It behaves differently depending on the action:
	 * - save: if the segments haven't changed, the start_at time is not changed
	 *         if the segments have changed, the start_at time is adjusted to make sure the timer is in the same state as before
	 * - restart: the start_at time is set to the current time
	 * - stop: the start_at time is not changed
	 * - resume: the start_at time is adjusted to make sure the timer is in the same state as before
	 */
	const calculateStartAt = (
		formData: TimerFormData,
		formSegments: Segment[],
		action: TimerAction
	): number => {
		if (action === 'restart') {
			return new Date().getTime();
		}

		const formDataStartedAt = new Date(
			`${formData.start_at_date} ${formData.start_at_time}`
		).getTime();
		if (formDataStartedAt > new Date().getTime()) {
			return formDataStartedAt;
		}

		return calculateNewStartAt(formSegments, timerData, action);
	};

	const formDataToTimerData = (formData: TimerFormData, action: TimerAction): Timer => {
		const start_at = calculateStartAt(formData, formData.segments, action);
		const stop_at = action === 'stop' ? new Date().getTime() : undefined;

		return {
			id: timerData.id,
			start_at: start_at,
			stop_at: stop_at,
			repeat: formData.repeat,
			segments: formData.segments,
			display_options: {
				clock: formData.display_options.clock,
				pre_start_behaviour: formData.display_options.run_before_started
					? 'RunNormally'
					: 'ShowZero'
			}
		};
	};

	const editSegment = (i: number) => {
		if (editingSegment === i) {
			editingSegment = undefined;
		} else {
			editingSegment = i;
		}
	};

	formData = timerDataToFormData(timerData);
</script>

<a href="/manage/{timerData.id}" class="btn variant-glass-primary mb-3"
	><Fa icon={faArrowLeft} />&nbsp; back to overview</a
>
<form class="w-full grid gap-3" on:submit={handleSubmit}>
	{#if timerData.stop_at}
		<aside class="alert variant-ghost-warning">
			<Fa icon={faPauseCircle} class="text-2xl" />
			<h3 class="alert-message">
				The timer is currently paused (since {new Date(timerData.stop_at).toLocaleString()})
			</h3>
		</aside>
	{/if}

	<strong>Timer sequence:</strong>

	{#each formData.segments as segment, i}
		<div class="card w-full grid sm:grid-cols-2 md:grid-cols-3 items-center p-[2px]">
			<SegmentInfoBox
				{segment}
				class="rounded-t-md rounded-b-2xl md:rounded-l-md md:rounded-r-2xl justify-center md:justify-start"
			/>

			<div class="p-2 flex items-center justify-around col-span-2 md:col-span-1">
				<button
					type="button"
					class="btn {editingSegment === i ? 'variant-filled-success' : 'variant-filled-secondary'}"
					on:click={() => editSegment(i)}
				>
					{#if editingSegment === i}
						<span><Fa icon={faCheck} /></span><span>Done</span>
					{:else}
						<span><Fa icon={faEdit} /></span><span>Edit</span>
					{/if}
				</button>

				<button
					type="button"
					class="btn-icon variant-ringed-tertiary"
					disabled={i === 0}
					on:click={() => {
						const segment = formData.segments[i];
						formData.segments[i] = formData.segments[i - 1];
						formData.segments[i - 1] = segment;
						formData.segments = [...formData.segments];
					}}><Fa icon={faArrowUp} /></button
				>

				<button
					type="button"
					class="btn-icon variant-ringed-tertiary"
					disabled={i === formData.segments.length - 1}
					on:click={() => {
						const segment = formData.segments[i];
						formData.segments[i] = formData.segments[i + 1];
						formData.segments[i + 1] = segment;
						formData.segments = [...formData.segments];
					}}><Fa icon={faArrowDown} /></button
				>

				<button
					type="button"
					class="btn-icon variant-filled-error"
					disabled={formData.segments.length === 1}
					on:click={() => {
						console.log('remove segment');
						formData.segments = formData.segments.filter((_, index) => index !== i);
					}}><Fa icon={faTrash} /></button
				>
			</div>

			{#if editingSegment === i}
				<div class="md:col-span-3 col-span-2 overflow-hidden" transition:slide|local>
					<SegmentForm bind:segment class="w-full space-y-4" />
				</div>
			{/if}
		</div>
	{/each}

	<button
		type="button"
		class="btn variant-glass-success p-2"
		on:click={() => {
			console.log('add segment');
			formData.segments.push({
				label: `Segment ${formData.segments.length + 1}`,
				time: 30 * 1000,
				sound: false,
				count_to: 0
			});
			formData.segments = [...formData.segments];
		}}><span><Fa icon={faAdd} /></span> <span>New segment</span></button
	>

	<SlideToggle active="bg-primary-500" name="repeat" size="sm" bind:checked={formData.repeat}>
		repeat
	</SlideToggle>

	<strong>Display options:</strong>
	<SlideToggle
		active="bg-primary-500"
		name="repeat"
		size="sm"
		bind:checked={formData.display_options.clock}
	>
		<div class="grid grid-cols-2 gap-2 items-center">
			show clock
			<HelpPopup>
				Shows a clock at the bottom of the timer.<br />Can be overridden on one timer by adding
				?clock=false to the URL.
			</HelpPopup>
		</div>
	</SlideToggle>

	<SlideToggle
		active="bg-primary-500"
		name="repeat"
		size="sm"
		bind:checked={formData.display_options.run_before_started}
	>
		<div class="grid grid-cols-2 gap-2 items-center">
			run before start time
			<HelpPopup>
				Makes the timer run normall before the start time.<br />Can be useful if ca competition
				should start at 9:00 but you want some test runs before.
			</HelpPopup>
		</div>
	</SlideToggle>

	<strong>When to start the timer:</strong>

	<div class="grid grid-cols-2 gap-3">
		<input
			class="input variant-form-material col-span-2 md:col-span-1"
			type="date"
			placeholder="timer starttime"
			bind:value={formData.start_at_date}
			required
		/>
		<input
			class="input variant-form-material col-span-2 md:col-span-1"
			type="time"
			placeholder="timer starttime"
			step="1"
			bind:value={formData.start_at_time}
			required
		/>
		<div class="col-span-2 flex items-center gap-2">
			<p>This field will only be used if the selected time is in the future.</p>
			<HelpPopup>
				... otherwise, the field will be adjusted to preserve the current state of the timer or
				restart it.
			</HelpPopup>
		</div>
	</div>

	<button class="btn variant-filled-secondary" on:click={handleButtonSubmit('save')}>
		<span><Fa icon={faSave} /></span><span>Save</span>
	</button>
</form>
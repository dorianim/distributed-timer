<script lang="ts">
	import {
		modalStore,
		popup,
		SlideToggle,
		type ModalComponent,
		type ModalSettings
	} from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import {
		faTrash,
		faAdd,
		faRemove,
		faGear,
		faQuestionCircle,
		faRefresh,
		faSave,
		faPause,
		faPlay,
		faPauseCircle,
		faVolumeUp,
		faVolumeMute
	} from '@fortawesome/free-solid-svg-icons';
	import type { Timer } from '../../../types/timer';
	import { calculateTimeInCurrentRound, calculateTimeInCurrentSegment } from '../../../utils/timer';
	import type { Segment } from '../../../types/segment';
	import SegmentDetailsForm from './SegmentDetailsForm.svelte';
	import TimeInputField from './TimeInputField.svelte';
	import HelpPopup from '../../../components/HelpPopup.svelte';

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

	type TimerAction = 'save' | 'restart' | 'stop' | 'resume';

	export let timerData: Timer;
	export let onSubmit: (timerData: Timer) => void;

	let formData: TimerFormData;

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

		const timeInAnySegmentChanged =
			formSegments.length == timerData.segments.length &&
			formSegments.some((segment, index) => {
				return segment.time !== timerData.segments[index].time;
			});

		if (action === 'save' && !timeInAnySegmentChanged) {
			return timerData.start_at;
		}

		let currentTime = new Date().getTime();
		if (action === 'resume') {
			currentTime = timerData.stop_at!;
		}

		const { timeInCurrentRound } = calculateTimeInCurrentRound(timerData, currentTime);
		const { timeInCurrentSegment, currentSegment } = calculateTimeInCurrentSegment(
			timeInCurrentRound,
			timerData.segments
		);
		const currentSegmentIndex = timerData.segments.indexOf(currentSegment);

		// find new segment with the same label
		let newSegment = formSegments.find((segment) => segment.label === currentSegment.label);
		if (!newSegment && formSegments.length > currentSegmentIndex) {
			newSegment = formSegments[currentSegmentIndex];
		} else if (!newSegment) {
			return new Date().getTime();
		}

		// calculate time before new segment
		const newSegmentIndex = formSegments.indexOf(newSegment);
		let timeBeforeNewSegment = 0;
		for (let i = 0; i < newSegmentIndex; i++) {
			timeBeforeNewSegment += formSegments[i].time;
		}

		// calculate time in new segment
		const timeInNewSegment =
			timeInCurrentSegment > newSegment.time ? 0 : newSegment.time - timeInCurrentSegment;

		// calculate new start_at
		return new Date().getTime() - timeBeforeNewSegment - timeInNewSegment;
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

	const openSegmentDetailsModal = (i: number) => {
		const modalComponent: ModalComponent = {
			ref: SegmentDetailsForm,
			props: { segment: { ...formData.segments[i] } }
		};

		const d: ModalSettings = {
			type: 'component',
			component: modalComponent,
			response: (segment: Segment | false) => {
				if (segment !== false) {
					formData.segments[i] = segment;
				}
			}
		};

		modalStore.trigger(d);
	};

	formData = timerDataToFormData(timerData);
	$: console.log(formData);
</script>

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
		<div class="card p-4 w-full grid gap-3 sm:grid-cols-2 lg:grid-cols-3 items-center">
			<input
				class="input variant-form-material"
				type="text"
				placeholder="label"
				bind:value={segment.label}
				required
			/>
			<TimeInputField bind:time={segment.time} />

			<div class="flex items-center justify-around sm:col-span-2 lg:col-span-1">
				<button
					type="button"
					class="btn-icon variant-filled-secondary"
					disabled={formData.segments.length === 1}
					on:click={() => openSegmentDetailsModal(i)}><Fa icon={faGear} /></button
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

				<button
					type="button"
					class="btn-icon variant-filled-success"
					on:click={() => {
						console.log('add segment');
						formData.segments.splice(i + 1, 0, {
							label: `Segment ${formData.segments.length + 1}`,
							time: 30 * 1000,
							sound: false,
							count_to: 0
						});
						formData.segments = [...formData.segments];
					}}><Fa icon={faAdd} /></button
				>
			</div>
		</div>
	{/each}

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

	<strong>Actions:</strong>

	<button class="btn variant-filled-secondary" on:click={handleButtonSubmit('save')}>
		<span><Fa icon={faSave} /></span><span>Save</span>
	</button>

	<div class="grid grid-cols-2 gap-3">
		<button
			class="btn variant-filled-primary col-span-2 md:col-span-1"
			on:click={handleButtonSubmit('restart')}
		>
			<span><Fa icon={faRefresh} /></span><span>Save and restart</span>
		</button>

		{#if timerData.stop_at}
			<button
				class="btn variant-filled-tertiary col-span-2 md:col-span-1"
				on:click={handleButtonSubmit('resume')}
			>
				<span><Fa icon={faPlay} /></span><span>Save and resume</span>
			</button>
		{:else}
			<button
				class="btn variant-filled-primary col-span-2 md:col-span-1"
				on:click={handleButtonSubmit('stop')}
			>
				<span><Fa icon={faPause} /></span><span>Save and pause</span>
			</button>
		{/if}
	</div>
</form>

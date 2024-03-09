<script lang="ts">
	import { RangeSlider, SlideToggle } from '@skeletonlabs/skeleton';
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
	import type { DisplayOptions } from 'types/displayOptions';
	import type { TimerMetadata } from 'types/timerMetadata';
	interface TimerFormData {
		start_at_date: string;
		start_at_time: string;
		repeat: boolean;
		segments: Segment[];
		display_options: DisplayOptions;
		metadata: TimerMetadata;
	}

	export let timerData: Timer;
	export let onSubmit: (timerData: Timer) => void;

	let formData: TimerFormData;
	let editingSegment: number | undefined = undefined;

	const handleSubmit = () => {
		onSubmit(formDataToTimerData(formData));
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
			display_options: timerData.display_options,
			metadata: timerData.metadata
		};
	};

	const formDataToTimerData = (formData: TimerFormData): Timer => {
		return {
			...timerData,
			start_at: new Date(`${formData.start_at_date} ${formData.start_at_time}`).getTime(),
			repeat: formData.repeat,
			segments: formData.segments
		};
	};

	const editSegment = (i: number) => {
		if (editingSegment === i) {
			editingSegment = undefined;
		} else {
			editingSegment = i;
		}
	};

	const updateFormData = (timerData: Timer) => {
		formData = timerDataToFormData(timerData);
	};

	const updateTimerData = (formData: TimerFormData) => {
		timerData = formDataToTimerData(formData);
	};

	$: updateFormData(timerData);
	$: updateTimerData(formData);

	const precisionLabel = ['none', 'low', 'medium', 'high', 'very high'];
</script>

<a href="/manage/{timerData.id}" class="btn variant-glass-primary mb-3"
	><Fa icon={faArrowLeft} />&nbsp; back to overview</a
>
<form class="w-full grid gap-2" on:submit|preventDefault>
	{#if timerData.stop_at}
		<aside class="alert variant-ghost-warning">
			<Fa icon={faPauseCircle} class="text-2xl" />
			<h3 class="alert-message">
				The timer is currently paused (since {new Date(timerData.stop_at).toLocaleString()})
			</h3>
		</aside>
	{/if}

	<strong class="pt-4 text-lg">Timer sequence:</strong>

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
				count_to: 0,
				sounds: []
			});
			formData.segments = [...formData.segments];
		}}><span><Fa icon={faAdd} /></span> <span>New segment</span></button
	>

	<div class="pl-2">
		<SlideToggle active="bg-primary-500" name="repeat" size="sm" bind:checked={formData.repeat}>
			repeat
		</SlideToggle>
	</div>

	<strong class="pt-4 text-lg">Timer options:</strong>

	<div class="pl-2">
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
	</div>

	<label class="pl-2 w-auto">
		<div class="flex flex-row gap-2 items-center pb-1">
			Restart precision
			<HelpPopup>
				Delays changes to the timer by the set amount of seconds to make sure all displays have
				received the update before anything changes. This is useful to make sure all displays change
				in perfect sync if you have a lot of displays and/or a slow internet connection.
			</HelpPopup>
		</div>
		<select class="select" bind:value={formData.metadata.delay_start_stop}>
			<option value={0}>None (0s)</option>
			<option value={1000}>Low (1s)</option>
			<option value={2000}>Medium (2s)</option>
			<option value={3000}>High (3s)</option>
			<option value={4000}>Very high (4s)</option>
		</select>
	</label>

	<label class="pl-2 w-auto">
		<div class="flex flex-row gap-2 items-center pb-1">
			Behaviour before start:
			<HelpPopup>
				What the timer should do before the start time.
				<ul>
					<li><strong>Show first segment:</strong> shows the beginning of the first segment</li>
					<li><strong>Show last segment:</strong> shows the end of the last segment</li>
					<li><strong>Run normally:</strong> makes the timer run normally</li>
				</ul>
			</HelpPopup>
		</div>
		<select class="select" bind:value={formData.display_options.pre_start_behaviour}>
			<option value="ShowFirstSegment">Show first segment</option>
			<option value="ShowLastSegment">Show last segment</option>
			<option value="RunNormally">Run normally</option>
		</select>
	</label>

	<div class="flex items-center gap-2 pt-4">
		<strong class="text-lg">When to start the timer:</strong>
		<HelpPopup>
			The time when the
			{#if formData.repeat}
				first iteration of the timer will begin
			{:else}
				the timer will be started
			{/if}. The start time will be modified if you restart, pause, resume or skip a segment.
		</HelpPopup>
	</div>

	<div class="grid grid-cols-2 gap-3 pl-2">
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
	</div>

	<p>
		<b class="text-[#F59D30]">Warning:</b> if you have modified any times in the sequence, the
		current time on the timer <b class="text-[#E01B24]">WILL CHANGE</b> as soon as you save!
	</p>

	<button class="btn variant-filled-secondary" on:click={handleSubmit}>
		<span><Fa icon={faSave} /></span><span>Save</span>
	</button>
</form>

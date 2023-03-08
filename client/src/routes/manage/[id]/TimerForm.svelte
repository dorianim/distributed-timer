<script lang="ts">
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import { faTrash, faAdd, faRemove } from '@fortawesome/free-solid-svg-icons';
	import type { Timer } from '../../../types/timer';

	interface SegmentFormData {
		label: string;
		time: string; // minutes:seconds
		sound: boolean;
		color?: string;
	}

	interface TimerFormData {
		when_to_start: 'now' | 'unchanged' | 'custom';
		start_at_date: string;
		start_at_time: string;
		repeat: boolean;
		segments: SegmentFormData[];
	}

	export let timerData: Timer;
	export let onSubmit: (timerData: Timer) => void;

	let formData: TimerFormData;

	const handleSubmit = (e: Event) => {
		e.preventDefault();
		console.log('submit');
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
			when_to_start: 'unchanged',
			segments: timerData.segments.map((segment) => {
				return {
					label: segment.label,
					time: `${Math.floor(segment.time / (1000 * 60))
						.toString()
						.padStart(2, '0')}:${(Math.floor(segment.time / 1000) % 60)
						.toString()
						.padStart(2, '0')}`,
					sound: segment.sound,
					color: segment.color
				};
			})
		};
	};

	const formDataToTimerData = (formData: TimerFormData): Timer => {
		const start_at =
			formData.when_to_start === 'now'
				? new Date().getTime()
				: new Date(`${formData.start_at_date} ${formData.start_at_time}`).getTime();

		return {
			id: timerData.id,
			start_at: start_at,
			repeat: formData.repeat,
			segments: formData.segments.map((segment) => {
				return {
					label: segment.label,
					time:
						parseInt(segment.time.split(':')[0]) * 60 * 1000 +
						parseInt(segment.time.split(':')[1]) * 1000,
					sound: segment.sound,
					color: segment.color
				};
			})
		};
	};

	const handleTimeFieldChange = (i: number) => {
		let time: string = formData.segments[i].time.replace(/[^0-9]/g, '');

		while (time.length < 4) {
			time = '0' + time;
		}

		time = time.substring(0, time.length - 2) + ':' + time.substring(time.length - 2);

		while (time[0] === '0' && time.length > 5) {
			time = time.substring(1);
		}

		formData.segments[i].time = time;
	};

	formData = timerDataToFormData(timerData);
	$: console.log(formData);
</script>

<form class="grid gap-3" on:submit={handleSubmit}>
	<strong>Timer sequence:</strong>

	{#each formData.segments as segment, i}
		<div class="card p-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-5 items-center">
			<input
				class="input variant-form-material"
				type="text"
				placeholder="label"
				bind:value={segment.label}
				required
			/>
			<input
				class="input variant-form-material"
				type="text"
				placeholder="time in milliseconds"
				pattern="^[0-9]+:[0-9][0-9]$"
				bind:value={segment.time}
				on:input={() => handleTimeFieldChange(i)}
				required
			/>

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

			<div class="flex items-center justify-around sm:col-span-2 lg:col-span-1">
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
							time: '0:30',
							sound: false
						});
						formData.segments = [...formData.segments];
					}}><Fa icon={faAdd} /></button
				>
			</div>
		</div>
	{/each}

	<SlideToggle active="bg-primary-500" name="repeat" size="sm" bind:checked={formData.repeat}
		>repeat</SlideToggle
	>
	<strong>When to start the timer:</strong>
	<fieldset>
		<label class="flex items-center space-x-2">
			<div class="space-y-2">
				<label class="flex items-center space-x-2">
					<input class="radio" type="radio" value="unchanged" bind:group={formData.when_to_start} />
					<p>Leave it unchanged</p>
				</label>
				<label class="flex items-center space-x-2">
					<input class="radio" type="radio" value="now" bind:group={formData.when_to_start} />
					<p>When clicking submit</p>
				</label>
				<label class="flex items-center space-x-2">
					<input class="radio" type="radio" value="custom" bind:group={formData.when_to_start} />
					<p>At a specific time</p>
				</label>
			</div>
		</label>
	</fieldset>

	{#if formData.when_to_start === 'custom'}
		<div class="grid grid-cols-2 gap-3">
			<input
				class="input variant-form-material"
				type="date"
				placeholder="timer starttime"
				bind:value={formData.start_at_date}
				required
			/>
			<input
				class="input variant-form-material"
				type="time"
				placeholder="timer starttime"
				step="1"
				bind:value={formData.start_at_time}
				required
			/>
		</div>
	{/if}

	<button class="btn variant-filled-primary">Submit</button>
</form>

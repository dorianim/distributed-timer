<script lang="ts">
	import Fa from 'svelte-fa';
	import type { PageData } from './$types';
	import {
		faChevronRight,
		faCircleCheck,
		faClose,
		faEdit,
		faForward,
		faPause,
		faPlay,
		faRefresh
	} from '@fortawesome/free-solid-svg-icons';
	import {
		updateTimer,
		calculateStartTimeAfterResume,
		calculateStartTimeAfterSkip
	} from './helpers';
	import type { Timer } from 'types/timer';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	export let data: PageData;
	let { timerData } = data;
	let submitResult: Promise<Timer | void> = Promise.resolve();

	const _updateTimer = (start_at?: number, stop_at?: number) => {
		if (!start_at) {
			start_at = timerData.start_at;
		}

		const newTimer = {
			...timerData,
			start_at: start_at,
			stop_at: stop_at
		};

		submitResult = updateTimer(timerData.id, newTimer, data.fetch).then((timer: Timer) => {
			timerData = timer;
			return timer;
		});
	};

	const restartTimer = () => {
		_updateTimer(new Date().getTime() + timerData.metadata.delay_start_stop);
	};

	const stopTimer = () => {
		_updateTimer(undefined, new Date().getTime() + timerData.metadata.delay_start_stop);
	};

	const resumeTimer = () => {
		_updateTimer(calculateStartTimeAfterResume(timerData), undefined);
	};

	const skipCurrentSegment = () => {
		_updateTimer(calculateStartTimeAfterSkip(timerData));
	};

	$: timerData = data.timerData;
</script>

<h2 class="text-center">Manage timer <strong>{timerData.id}</strong></h2>

<div class="m-auto items-center w-full max-w-screen-lg">
	<iframe class=" mb-4 w-full aspect-video card" src={`/t/${timerData.id}`} title="timer preview" />

	{#await submitResult}
		<div class="flex items-center justify-center">
			<ProgressRadial class="w-10" />
		</div>
	{:then}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-3 pb-4">
			<button class="btn variant-filled-primary p-4" on:click={restartTimer}>
				<span><Fa icon={faRefresh} /></span><span>Restart</span>
			</button>

			<button class="btn variant-filled-primary p-4" on:click={skipCurrentSegment}>
				<span><Fa icon={faForward} /></span><span>Skip current segment</span>
			</button>

			{#if timerData.stop_at}
				<button class="btn variant-filled-tertiary p-4" on:click={resumeTimer}>
					<span><Fa icon={faPlay} /></span><span>Resume</span>
				</button>
			{:else}
				<button class="btn variant-filled-primary p-4" on:click={stopTimer}>
					<span><Fa icon={faPause} /></span><span>Pause</span>
				</button>
			{/if}
		</div>

		<a class="btn variant-filled-secondary w-full" href={`/manage/${timerData.id}/edit`}>
			<span><Fa icon={faEdit} /></span><span>Edit</span>
		</a>
	{/await}
</div>

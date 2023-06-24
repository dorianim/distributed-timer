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
	import { calculateNewStartAt, updateTimer, type TimerAction } from './helpers';
	import type { Timer } from 'types/timer';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	export let data: PageData;
	let { timerData } = data;
	let submitResult: Promise<Timer | void> = Promise.resolve();

	const handleButton = (action: TimerAction) => {
		console.log('button submit', action);
		const start_at = calculateNewStartAt(timerData.segments, timerData, action);
		const stop_at = action === 'stop' ? new Date().getTime() : undefined;
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

	$: timerData = data.timerData;
</script>

<h2 class="text-center">Manage timer <strong>{timerData.id}</strong></h2>

<div class="m-auto items-center w-full max-w-screen-lg">
	<iframe class=" mb-4 w-full aspect-video card" src={`/t/${timerData.id}`} title="timer preview" />

	{#await submitResult}
		<div class="flex items-center justify-center">
			<ProgressRadial class="w-10" />
		</div>
	{:then result}
		{#if result}
			<aside class="alert variant-ghost-success">
				<Fa icon={faCircleCheck} class="text-2xl" />
				<h3 class="alert-message">Success</h3>
				<button
					class="btn-icon"
					on:click={() => {
						submitResult = Promise.resolve();
					}}><Fa icon={faClose} /></button
				>
			</aside>
		{/if}

		<div class="grid grid-cols-1 md:grid-cols-3 gap-3 pb-4">
			<button class="btn variant-filled-primary p-4" on:click={() => handleButton('restart')}>
				<span><Fa icon={faRefresh} /></span><span>Restart</span>
			</button>

			<button class="btn variant-filled-primary p-4" on:click={() => handleButton('skip')}>
				<span><Fa icon={faForward} /></span><span>Skip current segment</span>
			</button>

			{#if timerData.stop_at}
				<button class="btn variant-filled-tertiary p-4" on:click={() => handleButton('resume')}>
					<span><Fa icon={faPlay} /></span><span>Resume</span>
				</button>
			{:else}
				<button class="btn variant-filled-primary p-4" on:click={() => handleButton('stop')}>
					<span><Fa icon={faPause} /></span><span>Pause</span>
				</button>
			{/if}
		</div>

		<a class="btn variant-filled-secondary w-full" href={`/manage/${timerData.id}/edit`}>
			<span><Fa icon={faEdit} /></span><span>Edit</span>
		</a>
	{/await}
</div>

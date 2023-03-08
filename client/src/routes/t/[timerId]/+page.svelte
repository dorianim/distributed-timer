<script lang="ts">
	import { Modal, type ModalSettings, modalStore, ProgressRadial } from '@skeletonlabs/skeleton';
	import * as Tone from 'tone';
	import type { PageData } from './$types';
	import { get } from 'svelte/store';
	import screenfull from 'screenfull';
	import Timer from './Timer.svelte';
	import type { Timer as TimerType } from '../../../types/timer';
	import { API_WS_URL } from '../../../stores';
	import { onMount } from 'svelte';

	export let data: PageData;

	let soundEnabled = false;
	let timeOffset: number | undefined;
	let timerData: TimerType | undefined;
	let lastGetTimeSent = 0;
	let latestOffsets: number[] = [];
	let socket: WebSocket | undefined;

	const isOffsetInMargin = (offset: number) => {
		if (!timeOffset) {
			return true;
		}

		let margin = timeOffset * 0.3;
		return offset > timeOffset - margin && offset < timeOffset + margin;
	};

	const handleNewOffset = (offset: number) => {
		// check if we are in a 30% margin
		if (!isOffsetInMargin(offset)) {
			return;
		}

		latestOffsets.push(offset);
		if (latestOffsets.length > 10) {
			latestOffsets.shift();
		}

		let sum = 0;
		for (let i = 0; i < latestOffsets.length; i++) {
			sum += latestOffsets[i];
		}

		timeOffset = sum / latestOffsets.length;
		console.log(timeOffset);
		console.log(latestOffsets);
	};

	const enableSound = () => {
		Tone.start().then(() => (soundEnabled = true));
	};

	const initSocket = () => {
		socket = new WebSocket(get(API_WS_URL));

		// Listen for messages
		socket.addEventListener('message', (event) => {
			const data = JSON.parse(event.data);
			console.log(data);
			switch (data.type) {
				case 'Timer':
					timerData = data.data;
					break;
				case 'Timestamp':
					const now = performance.now();
					const getTimeRoundTrip = now - lastGetTimeSent;
					timeOffset = data.data + getTimeRoundTrip / 2 - now;
					handleNewOffset(timeOffset);
			}
		});

		// Connection opened
		socket.addEventListener('open', (event) => {
			socket?.send(JSON.stringify({ data: data.params.timerId, type: 'Hello' }));

			setInterval(() => {
				lastGetTimeSent = performance.now();
				socket?.send(JSON.stringify({ type: 'GetTime' }));
			}, 1000);
		});

		// Listen for errors
		socket.addEventListener('error', restartSocket);

		// Listen for close
		socket.addEventListener('close', restartSocket);
	};

	const restartSocket = (e: Event) => {
		socket?.removeEventListener('error', restartSocket);
		socket?.removeEventListener('close', restartSocket);
		socket?.close();

		console.log('restarting socket', e);

		setTimeout(() => {
			socket = undefined;
		}, 1000);
	};

	Tone.start().then(() => (soundEnabled = true));

	$: {
		if (!socket) {
			initSocket();
		}
	}

	$: {
		if (timerData && !soundEnabled && get(modalStore).length == 0) {
			const d: ModalSettings = {
				type: 'alert',
				body: 'Tap anywhere to enable sound',
				buttonTextCancel: 'ok',
				response: () => {
					enableSound();
				}
			};
			modalStore.trigger(d);

			setTimeout(() => {
				modalStore.close();
				modalStore.clear();
			}, 5000);
		} else if (soundEnabled && get(modalStore).length > 0) {
			modalStore.close();
			modalStore.clear();
		}
	}
</script>

{#if socket?.readyState === 1 && timerData && timeOffset}
	<Timer {timerData} {soundEnabled} {timeOffset} />
{:else}
	<div
		class="absolute top-0 h-[100vh] left-[50%] translate-x-[-50%] flex items-center justify-center"
	>
		<ProgressRadial class="w-10" />
	</div>
{/if}

<div
	class="absolute top-0 left-0 w-[100vw] h-[100vh] z-50"
	on:click={() => {
		screenfull.toggle();
		enableSound();
	}}
	on:keydown={() => {}}
/>

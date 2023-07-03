<script lang="ts">
	import { type ModalSettings, modalStore, ProgressRadial } from '@skeletonlabs/skeleton';
	import type { PageData } from './$types';
	import { get } from 'svelte/store';
	import screenfull from 'screenfull';
	import Timer from './Timer.svelte';
	import type { Timer as TimerType } from '../../../types/timer';
	import { API_WS_URL } from '../../../stores';
	import NoSleep from 'nosleep.js';
	import { goto } from '$app/navigation';
	import { onDestroy } from 'svelte';

	export let data: PageData;

	const noSleep = new NoSleep();

	let currentOffset: number | undefined;
	let currentFluctuation: number | undefined;
	let latestOffsets: number[] = [];
	let latestFluctuations: number[] = [];

	let soundEnabled: boolean;
	let timerData: TimerType | undefined;
	let lastGetTimeSent = 0;
	let lastGetTimeReceived = 0;
	let socket: WebSocket | undefined;

	const pushValueAndCaluclateAverage = (values: number[], newValue: number) => {
		values.push(newValue);
		if (values.length > 10) {
			values.shift();
		}

		let sum = 0;
		for (let i = 0; i < values.length; i++) {
			sum += values[i];
		}

		return sum / values.length;
	};

	const isOffsetInMargin = (newOffset: number) => {
		if (!currentOffset) {
			return true;
		}

		const fluctuation = Math.abs(currentOffset - newOffset);

		if (
			currentFluctuation &&
			latestFluctuations.length > 5 &&
			fluctuation > currentFluctuation * 4
		) {
			return false;
		}

		currentFluctuation = pushValueAndCaluclateAverage(latestFluctuations, fluctuation);

		if (latestFluctuations.length < 10) {
			return true;
		}

		return fluctuation < currentFluctuation * 2;
	};

	const handleNewOffset = (newOffset: number) => {
		// check if we are in a 30% margin
		if (!isOffsetInMargin(newOffset)) {
			return;
		}

		currentOffset = pushValueAndCaluclateAverage(latestOffsets, newOffset);
	};

	const enableSound = () => {
		new Audio('/sound/silence.mp3')
			.play()
			.then(() => {
				soundEnabled = true;
			})
			.catch((e) => {
				console.error('Sound is disabled');
				soundEnabled = false;
			});
	};

	const throwError = (code: number, message: string) => {
		const d: ModalSettings = {
			type: 'alert',
			body: 'Error: ' + message,
			buttonTextCancel: 'go back',
			response: () => {
				goto('/');
			}
		};
		modalStore.trigger(d);
	};

	const initSocket = () => {
		socket = new WebSocket(get(API_WS_URL));

		// Listen for messages
		socket.addEventListener('message', (event) => {
			const data = JSON.parse(event.data);

			switch (data.type) {
				case 'Timer':
					const timer: TimerType = data.data;
					timerData = timer;
					break;
				case 'Timestamp':
					const lastGetTimeReceived = performance.now();
					const getTimeRoundTrip = lastGetTimeReceived - lastGetTimeSent;
					console.log('roundtrip', getTimeRoundTrip);
					let newOffset = data.data + getTimeRoundTrip / 2 - lastGetTimeReceived;
					handleNewOffset(newOffset);
					break;
				case 'Error':
					throwError(data.data[0], data.data[1]);
					break;
			}
		});

		// Connection opened
		socket.addEventListener('open', (event) => {
			socket?.send(JSON.stringify({ data: data.params.timerId, type: 'Hello' }));

			setInterval(() => {
				let lastGetTimeReceivedOneSecondAgo =
					lastGetTimeReceived > lastGetTimeSent && performance.now() - lastGetTimeReceived >= 1000;
				let lastGetTimeReceivedTimeout =
					lastGetTimeReceived < lastGetTimeSent && performance.now() - lastGetTimeSent >= 10000;

				if (
					lastGetTimeSent !== 0 &&
					!lastGetTimeReceivedOneSecondAgo &&
					!lastGetTimeReceivedTimeout
				) {
					return;
				}
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

	enableSound();

	$: {
		if (!socket) {
			initSocket();
		}
	}

	onDestroy(() => {
		socket?.close();
	});

	$: {
		if (timerData && !soundEnabled && get(modalStore).length == 0) {
			const d: ModalSettings = {
				type: 'alert',
				body: 'Tap anywhere to enable sound and wakelock.',
				buttonTextCancel: 'ok',
				response: () => {
					enableSound();
					noSleep.enable();
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

{#if timerData && currentOffset}
	<Timer
		{timerData}
		{soundEnabled}
		timeOffset={currentOffset}
		displayOptionsOverride={{
			...timerData.display_options,
			clock: data.url.searchParams.get('clock') != 'false'
		}}
	/>
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
		noSleep.enable();
		enableSound();
	}}
	on:keydown={() => {}}
/>

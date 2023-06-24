<script lang="ts">
	import type { Timer } from '../../../types/timer';
	import { onMount } from 'svelte';
	import type { DisplayOptions } from '../../../types/displayOptions';
	import {
		calculateTimeInCurrentRound,
		calculateTimeInCurrentSegment,
		getTimerText
	} from '../../../utils/timer';

	export let timerData: Timer;
	export let soundEnabled: boolean;
	export let timeOffset: number;
	export let displayOptionsOverride: DisplayOptions | undefined;

	const preloadSounds = (sounds: string[]) => {
		let audios: { [sound: string]: HTMLAudioElement } = {};
		for (const sound of sounds) {
			console.log('preloading sound: ', sound);
			const audio = new Audio(sound);
			audio.load();
			audios[sound] = audio;
		}
		return audios;
	};

	const playSoundAt = (playAtSeconds: number, currentSeconds: number, sound: string) => {
		if (!audios) return;

		const audio = audios[sound];

		if (playAtSeconds === currentSeconds && audio.paused) {
			console.log('playing sound: ', sound, 'at', currentSeconds, 'seconds remaining');
			audio.play();
		}
	};

	const playCurrentSound = (seconds: number) => {
		if (!soundEnabled || !audios) return;

		playSoundAt(60, seconds, '/sound/beep.mp3');
		playSoundAt(5, seconds, '/sound/countdown.mp3');
	};

	const calculateCurrentSegment: () => {
		timerText: string;
		label: string;
		seconds: number;
		color?: string;
		sound: boolean;
		currentTime: number;
	} = () => {
		const currentTime = performance.now() + timeOffset;
		const segments = timerData.segments;
		let { timeInCurrentRound, state } = calculateTimeInCurrentRound(timerData, currentTime);

		if (state == 'waiting') {
			return {
				timerText: getTimerText(0),
				label: '',
				seconds: 0,
				sound: false,
				currentTime: currentTime
			};
		}

		if (state == 'finished') {
			return {
				timerText: getTimerText(0),
				label: segments[segments.length - 1].label,
				seconds: 0,
				sound: false,
				currentTime: currentTime
			};
		}

		const { timeInCurrentSegment, currentSegment } = calculateTimeInCurrentSegment(
			timeInCurrentRound,
			timerData.segments
		);

		const effectiveTimeInCurrentSegment = timeInCurrentSegment + currentSegment.count_to;

		return {
			timerText: getTimerText(effectiveTimeInCurrentSegment),
			label: currentSegment.label,
			seconds: Math.floor(effectiveTimeInCurrentSegment / 1000),
			color: currentSegment.color,
			sound: currentSegment.sound,
			currentTime: currentTime
		};
	};

	const update = () => {
		currentSegment = calculateCurrentSegment();
		const { timerText, label, color, seconds } = currentSegment;
		console.log(new Date(currentSegment.currentTime).toLocaleTimeString());

		if (timerSpan !== null && timerSpan.innerText !== timerText) {
			timerSpan.innerText = timerText;
			labelSpan.innerText = label;
			backgroundDiv.style.setProperty(
				`--backgroundColor`,
				color ?? 'rgb(var(--color-surface-900))'
			);
		}
		playCurrentSound(seconds);
	};

	let audios: { [sound: string]: HTMLAudioElement } | undefined;
	let currentSegment = calculateCurrentSegment();
	let backgroundDiv: HTMLElement;
	let labelSpan: HTMLElement;
	let timerSpan: HTMLElement;

	onMount(() => {
		setInterval(() => {
			update();
		}, 10);
	});

	$: {
		if (soundEnabled) {
			audios = preloadSounds(['/sound/beep.mp3', '/sound/countdown.mp3']);
		} else {
			audios = undefined;
		}
	}
</script>

<div
	bind:this={backgroundDiv}
	class="absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] overflow-hidden h-[100vh] w-[100vw] flex items-center justify-center background-color"
>
	<span
		bind:this={labelSpan}
		class="absolute top-[10%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-[5vw] text-auto"
	/>
	<span
		bind:this={timerSpan}
		class="{currentSegment.timerText.length > 5 ? 'text-[23.5vw]' : 'text-[35vw]'} text-auto"
	/>
	{#if timerData.display_options.clock && (!displayOptionsOverride || displayOptionsOverride.clock)}
		<span
			class="absolute bottom-[10%] left-[50%] translate-x-[-50%] translate-y-[50%] text-[5vw] text-auto"
		>
			{new Date(currentSegment.currentTime).toLocaleTimeString([], {
				hour: '2-digit',
				minute: '2-digit'
			})}
		</span>
	{/if}
</div>

<style lang="postcss">
	.background-color {
		background-color: var(--backgroundColor);
	}

	span {
		line-height: normal;
	}
</style>

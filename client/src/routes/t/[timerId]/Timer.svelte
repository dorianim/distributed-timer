<script lang="ts">
	import type { Timer } from '../../../types/timer';
	import { onMount } from 'svelte';

	export let timerData: Timer;
	export let soundEnabled: boolean;
	export let timeOffset: number;

	const zeroPad = (num: number, places: number) => String(num).padStart(places, '0');
	const getTimerText = (millis: number) => {
		let seconds = Math.floor(millis / 1000);
		if (seconds < 0) {
			seconds = 0;
		}

		let remaningHours = zeroPad(Math.floor(seconds / 60 / 60) % (60 * 60), 2);
		let remaningMinutes = zeroPad(Math.floor(seconds / 60) % 60, 2);
		let remaningSeconds = zeroPad(seconds % 60, 2);

		if (parseInt(remaningHours) === 0) {
			return remaningMinutes + ':' + remaningSeconds;
		}

		return remaningHours + ':' + remaningMinutes + ':' + remaningSeconds;
	};

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
	} = () => {
		const currentTime = performance.now() + timeOffset;
		const elapsedTime = currentTime - timerData.start_at;

		if (elapsedTime < 0) {
			return {
				timerText: getTimerText(0),
				label: '',
				seconds: 0,
				sound: false
			};
		}

		const segments = timerData.segments.map((segment) => ({
			...segment,
			time: segment.time + 1000
		}));
		const totalTimePerRound = segments.reduce((acc, curr) => acc + curr.time, 0);

		if (!timerData.repeat && elapsedTime > totalTimePerRound) {
			return {
				timerText: getTimerText(0),
				label: segments[segments.length - 1].label,
				seconds: 0,
				sound: false
			};
		}

		let timeInCurrentRound = elapsedTime % totalTimePerRound;

		let currentSegmentIndex = 0;
		let timeInCurrentSegment = 0;
		while (timeInCurrentRound > 0) {
			timeInCurrentSegment = Math.floor(segments[currentSegmentIndex].time - timeInCurrentRound);
			timeInCurrentRound -= segments[currentSegmentIndex].time;
			currentSegmentIndex++;
		}

		const currentSegment = segments[currentSegmentIndex - 1];

		return {
			timerText: getTimerText(timeInCurrentSegment),
			label: currentSegment.label,
			seconds: Math.floor(timeInCurrentSegment / 1000),
			color: currentSegment.color,
			sound: currentSegment.sound
		};
	};

	const update = () => {
		const { timerText, label, color, seconds } = calculateCurrentSegment();

		if (timerSpan.innerText !== timerText) {
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
		}, 100);
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
		class="absolute top-[10%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-[5vw]"
	/>
	<span
		bind:this={timerSpan}
		class={currentSegment.timerText.length > 5 ? 'text-[23.5vw]' : 'text-[35vw]'}
	/>
</div>

<style>
	.background-color {
		background-color: var(--backgroundColor);
	}

	span {
		background: inherit;
		-webkit-background-clip: text;
		background-clip: text;
		color: transparent;
		filter: sepia(5) saturate(100) invert(1) grayscale(1) contrast(9);
	}
</style>

<script lang="ts">
	import type { Timer } from '../../../types/timer';
	import type { AudioContext, GainNode, OscillatorNode } from 'standardized-audio-context';
	import type { Action } from 'svelte/types/runtime/action';

	export let timerData: Timer;
	export let audioContext: AudioContext;
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

	const playBeep = async (frequency: number, duration: number, volume: number) => {
		if (audioContext.state !== 'running') {
			return;
		}

		const gainNode = audioContext.createGain();
		gainNode.connect(audioContext.destination);
		const oscillatorNode = audioContext.createOscillator();
		oscillatorNode.connect(gainNode);

		oscillatorNode.type = 'sine';

		oscillatorNode.frequency.setValueAtTime(frequency, audioContext.currentTime);
		gainNode.gain.setTargetAtTime(volume, audioContext.currentTime, 0.02);
		gainNode.gain.setTargetAtTime(0, audioContext.currentTime + duration / 1000, 0.02);

		oscillatorNode.start();
	};

	const playCurrentSound = (seconds: number) => {
		if (lastSoundPlayed == seconds) return;

		lastSoundPlayed = seconds;
		if (seconds == 60) {
			playBeep(463, 1000, 0.5);
		}

		if (seconds == 0) {
			playBeep(158, 1000, 1);
		} else if (seconds <= 5) {
			playBeep(463, 200, 0.5);
		}
	};

	const calculateCurrentSegment = () => {
		const currentTime = performance.now() + timeOffset;
		const elapsedTime = currentTime - timerData.start_at;

		const segments = timerData.segments.map((segment) => ({
			...segment,
			time: segment.time + 1000
		}));
		const totalTimePerRound = segments.reduce((acc, curr) => acc + curr.time, 0);

		if (!timerData.repeat && elapsedTime > totalTimePerRound) {
			return {
				timerText: getTimerText(0),
				label: segments[segments.length - 1].label,
				seconds: 0
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

		if (currentSegment.sound) {
			playCurrentSound(Math.floor(timeInCurrentSegment / 1000));
		}

		return {
			timerText: getTimerText(timeInCurrentSegment),
			label: currentSegment.label,
			seconds: Math.floor(timeInCurrentSegment / 1000),
			color: currentSegment.color
		};
	};

	let lastSoundPlayed = -1;
	let currentSegment = calculateCurrentSegment();
	let backgroundDiv: HTMLElement;

	$: {
		setInterval(() => {
			currentSegment = calculateCurrentSegment();
		}, 100);
	}

	$: {
		if (backgroundDiv) {
			backgroundDiv.style.setProperty(
				`--backgroundColor`,
				currentSegment.color ?? 'rgb(var(--color-surface-900))'
			);
		}
	}
</script>

<div
	bind:this={backgroundDiv}
	class="absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] overflow-hidden h-[100vh] w-[100vw] flex items-center justify-center background-color"
>
	<span class="absolute top-[10%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-[5vw]"
		>{currentSegment.label}</span
	>
	<span class={currentSegment.timerText.length > 5 ? 'text-[23.5vw]' : 'text-[35vw]'}>
		{currentSegment.timerText}
	</span>
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

<script lang="ts">
	import type { Timer } from '../../../types/timer';
	import * as Tone from 'tone';

	export let timerData: Timer;
	export let soundEnabled: boolean;
	export let timeOffset: number;

	const synth = new Tone.Synth({
		onsilence: () => {
			console.log('silence');
			soundPlaying = false;
		}
	}).toDestination();

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

	const playCountdown = async () => {
		if (soundPlaying || !soundEnabled) {
			return;
		}
		soundPlaying = true;
		const now = Tone.now();
		synth.triggerAttackRelease('C4', '8n', now);
		synth.triggerAttackRelease('C4', '8n', now + 1);
		synth.triggerAttackRelease('C4', '8n', now + 2);
		synth.triggerAttackRelease('C4', '8n', now + 3);
		synth.triggerAttackRelease('C4', '8n', now + 4);
		synth.triggerAttackRelease('C3', '2n', now + 5);
	};

	const playBeep = async () => {
		if (soundPlaying || !soundEnabled) {
			return;
		}
		soundPlaying = true;
		const now = Tone.now();
		synth.triggerAttackRelease('C5', '2n', now);
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
			timeInCurrentSegment = segments[currentSegmentIndex].time - timeInCurrentRound;
			timeInCurrentRound -= segments[currentSegmentIndex].time;
			currentSegmentIndex++;
		}

		const currentSegment = segments[currentSegmentIndex - 1];

		if (
			currentSegment.sound &&
			Math.floor(timeInCurrentSegment / 1000) == Math.floor((currentSegment.time - 1) / 1000)
		) {
			playBeep();
		}

		if (currentSegment.sound && Math.floor(timeInCurrentSegment / 1000) == 60) {
			playBeep();
		}

		if (currentSegment.sound && Math.floor(timeInCurrentSegment / 1000) == 5) {
			playCountdown();
		}

		return {
			timerText: getTimerText(timeInCurrentSegment),
			label: currentSegment.label,
			seconds: Math.floor(timeInCurrentSegment / 1000)
		};
	};
	let soundPlaying = false;
	let currentSegment = calculateCurrentSegment();
	$: {
		setInterval(() => {
			currentSegment = calculateCurrentSegment();
		}, 100);
	}
</script>

<span class="absolute top-[10%] left-[50%] translate-x-[-50%] translate-y-[-50%] text-[5vw]"
	>{currentSegment.label}</span
>

<span
	class="absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] {currentSegment
		.timerText.length > 5
		? 'text-[23.5vw]'
		: 'text-[35vw]'}"
>
	{currentSegment.timerText}
</span>

import type { Segment } from '../types/segment';
import type { Timer } from '../types/timer';

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

function calculateTimeInCurrentRound(
	timerData: Timer,
	currentTime: number
): { timeInCurrentRound: number; state: 'running' | 'waiting' | 'finished' | 'stopped' } {
	let stopped = false;

	if (timerData.stop_at && timerData.stop_at < currentTime) {
		currentTime = timerData.stop_at;
		stopped = true;
	}

	const elapsedTime = currentTime - timerData.start_at;

	if (elapsedTime < 0) {
		return {
			timeInCurrentRound: 0,
			state: 'waiting'
		};
	}

	const totalTimePerRound = timerData.segments.reduce((acc, curr) => acc + curr.time, 0);

	if (!timerData.repeat && !stopped && elapsedTime > totalTimePerRound) {
		return {
			timeInCurrentRound: 0,
			state: 'finished'
		};
	}

	let timeInCurrentRound = elapsedTime % totalTimePerRound;
	return {
		timeInCurrentRound,
		state: stopped ? 'stopped' : 'running'
	};
}

function calculateTimeInCurrentSegment(
	timeInCurrentRound: number,
	segments: Segment[]
): {
	currentSegment: Segment;
	timeInCurrentSegment: number;
} {
	let currentSegmentIndex = 0;
	let timeInCurrentSegment = 0;
	while (timeInCurrentRound > 0) {
		timeInCurrentSegment = Math.floor(segments[currentSegmentIndex].time - timeInCurrentRound);
		timeInCurrentRound -= segments[currentSegmentIndex].time;
		currentSegmentIndex++;
	}

	const currentSegment = segments[currentSegmentIndex - 1];

	return {
		currentSegment,
		timeInCurrentSegment
	};
}

export { calculateTimeInCurrentRound, calculateTimeInCurrentSegment, getTimerText };

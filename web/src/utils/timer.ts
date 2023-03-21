import type { Segment } from '../types/segment';
import type { Timer } from '../types/timer';

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

export { calculateTimeInCurrentRound, calculateTimeInCurrentSegment };

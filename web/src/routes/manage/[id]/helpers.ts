import { error } from '@sveltejs/kit';
import { API_URL } from 'stores';
import { get } from 'svelte/store';
import type { Fetch } from 'types/fetch';
import type { Segment } from 'types/segment';
import type { Timer, TimerUpdateRequest } from 'types/timer';
import {
	calculateTimeInCurrentRound,
	calculateTimeInCurrentSegment,
	getTimerText
} from 'utils/timer';

export const calculateStartTimeAfterResume = (timerData: Timer) => {
	if (!timerData.stop_at) {
		return timerData.start_at;
	}

	const timeElapsedBeforeStop = timerData.stop_at - timerData.start_at;
	const currentTime = new Date().getTime();
	return currentTime - timeElapsedBeforeStop;
};

export const calculateStartTimeAfterSkip = (timerData: Timer) => {
	const currentTime = new Date().getTime();
	const { timeInCurrentRound, state } = calculateTimeInCurrentRound(timerData, currentTime);
	if (state != 'running') {
		return timerData.start_at;
	}

	const { timeInCurrentSegment } = calculateTimeInCurrentSegment(
		timeInCurrentRound,
		timerData.segments
	);

	return timerData.start_at - timeInCurrentSegment;
};
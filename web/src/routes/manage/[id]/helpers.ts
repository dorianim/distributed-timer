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

export const getTimer = async (id: string, fetch: Fetch) => {
	const resp = await fetch(`${get(API_URL)}/timer/${id}`);
	if (!resp.ok) {
		throw error(resp.status, resp.statusText);
	}
	const timerData: Timer | undefined = resp.ok ? await resp.json() : undefined;
	if (!timerData) throw error(404, 'Timer not found');
	return timerData;
};

export const updateTimer = async (id: string, newTimerData: TimerUpdateRequest, fetch: Fetch) => {
	return fetch(`${get(API_URL)}/timer/${id}`, {
		method: 'PUT',
		body: JSON.stringify(newTimerData),
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${localStorage.getItem('token')}`
		}
	})
		.then((res) => {
			if (!res.ok) {
				throw new Error(res.statusText);
			}
			return res.json();
		})
		.then((timer: Timer) => {
			return timer;
		});
};

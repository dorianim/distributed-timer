import { error } from '@sveltejs/kit';
import { API_URL } from 'stores';
import { get } from 'svelte/store';
import type { Fetch } from 'types/fetch';
import type { Segment } from 'types/segment';
import type { Timer, TimerUpdateRequest } from 'types/timer';
import { calculateTimeInCurrentRound, calculateTimeInCurrentSegment } from 'utils/timer';

export type TimerAction = 'save' | 'restart' | 'stop' | 'resume';

/**
 * This function calculates the new start_at
 * It behaves differently depending on the action:
 * - save: if the segments haven't changed, the start_at time is not changed
 *         if the segments have changed, the start_at time is adjusted to make sure the timer is in the same state as before
 * - restart: the start_at time is set to the current time
 * - stop: the start_at time is not changed
 * - resume: the start_at time is adjusted to make sure the timer is in the same state as before
 */
export const calculateNewStartAt = (
	newSegments: Segment[],
	oldTimerData: Timer,
	action: TimerAction
): number => {
	if (action === 'restart') {
		return new Date().getTime();
	}

	const timeInAnySegmentChanged =
		newSegments.length == oldTimerData.segments.length &&
		newSegments.some((segment, index) => {
			return segment.time !== oldTimerData.segments[index].time;
		});

	if (action === 'save' && !timeInAnySegmentChanged) {
		return oldTimerData.start_at;
	}

	let currentTime = new Date().getTime();
	if (action === 'resume') {
		currentTime = oldTimerData.stop_at!;
	}

	const { timeInCurrentRound } = calculateTimeInCurrentRound(oldTimerData, currentTime);
	const { timeInCurrentSegment, currentSegment } = calculateTimeInCurrentSegment(
		timeInCurrentRound,
		oldTimerData.segments
	);
	const currentSegmentIndex = oldTimerData.segments.indexOf(currentSegment);

	// find new segment with the same label
	let newSegment = newSegments.find((segment) => segment.label === currentSegment.label);
	if (!newSegment && newSegments.length > currentSegmentIndex) {
		newSegment = newSegments[currentSegmentIndex];
	} else if (!newSegment) {
		return new Date().getTime();
	}

	// calculate time before new segment
	const newSegmentIndex = newSegments.indexOf(newSegment);
	let timeBeforeNewSegment = 0;
	for (let i = 0; i < newSegmentIndex; i++) {
		timeBeforeNewSegment += newSegments[i].time;
	}

	// calculate time in new segment
	const timeInNewSegment =
		timeInCurrentSegment > newSegment.time ? 0 : newSegment.time - timeInCurrentSegment;

	// calculate new start_at
	return new Date().getTime() - timeBeforeNewSegment - timeInNewSegment;
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

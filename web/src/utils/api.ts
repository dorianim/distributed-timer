import { API_URL } from 'stores';
import { get } from 'svelte/store';
import type { Fetch } from 'types/fetch';
import type {
	Timer,
	TimerCreationRequest,
	TimerCreationResponse,
	TimerLoginResponse,
	TimerUpdateRequest
} from 'types/timer';

const getTimer = async (id: string, fetch: Fetch): Promise<Timer> => {
	const res = await fetch(`${get(API_URL)}/timer/${id}`);

	if (!res.ok) {
		throw new Error(res.statusText);
	}

	return await res.json();
};

const loginTimer = async (
	id: string,
	password: string,
	fetch: Fetch
): Promise<TimerLoginResponse> => {
	const res = await fetch(`${get(API_URL)}/timer/token`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			id,
			password
		})
	});

	if (!res.ok) {
		throw new Error(res.statusText);
	}

	return await res.json();
};

const updateTimer = async (
	id: string,
	newTimerData: TimerUpdateRequest,
	token: string,
	fetch: Fetch
) => {
	const res = await fetch(`${get(API_URL)}/timer/${id}`, {
		method: 'PUT',
		body: JSON.stringify(newTimerData),
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		}
	});

	if (!res.ok) {
		throw new Error(res.statusText);
	}

	return await res.json();
};

interface ErrorMessages {
	[key: number]: string;
}

const errorMessages: ErrorMessages = {
	409: 'A timer with that name already exists'
};

const createTimer = async (
	timerData: TimerCreationRequest,
	fetch: Fetch
): Promise<TimerCreationResponse> => {
	const res = await fetch(`${get(API_URL)}/timer`, {
		method: 'POST',
		body: JSON.stringify(timerData),
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!res.ok && res.status in errorMessages) {
		throw new Error(errorMessages[res.status]);
	} else if (!res.ok) {
		throw new Error(res.statusText);
	}

	return await res.json();
};

export { loginTimer, updateTimer, createTimer, getTimer };

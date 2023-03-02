import { error, type Load } from '@sveltejs/kit';
import { get } from 'svelte/store';
import { API_URL } from '../../../stores';
import type { Timer } from '../../../types/timer';

export const load: Load = async ({ fetch, url, params }) => {
	const resp = await fetch(`${get(API_URL)}/api/timer/${params.id}`);
	if (!resp.ok) {
		throw error(resp.status, resp.statusText);
	}
	const timerData: Timer | undefined = resp.ok ? await resp.json() : undefined;
	if (!timerData) throw error(404, 'Timer not found');

	return { fetch, url, params, timerData };
};

import type { Load } from '@sveltejs/kit';
//import { get } from 'svelte/store';
//import { API_URL } from '../../../stores';
import type { Timer } from '../../../types/timer';

export const load: Load = async ({ fetch, url, params }) => {
	/*const resp = await fetch(`${get(API_URL)}/api/t/${params.timerId}`);
	if (!resp.ok) {
		throw error(resp.status, resp.statusText);
	}
	const timerData: Timer | undefined = resp.ok ? await resp.json() : undefined;*/

	const timerData: Timer | undefined = {
		name: 'Test',
		start_at: new Date().getTime(),
		repeat: false,
		segments: [
			{
				label: 'Switch!',
				time: 15 * 1000,
				sound: false
			},
			{
				label: 'Boulder!',
				time: 4 * 60 * 1000,
				sound: true
			}
		]
	};

	return { fetch, url, params, timerData };
};

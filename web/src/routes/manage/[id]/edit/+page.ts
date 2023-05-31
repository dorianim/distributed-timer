import type { Load } from '@sveltejs/kit';
import { getTimer } from '../helpers';

export const load: Load = async ({ fetch, url, params }) => {
	const timerData = getTimer(params.id ?? '', fetch);
	return { fetch, url, params, timerData };
};

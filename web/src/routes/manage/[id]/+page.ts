import type { Load } from '@sveltejs/kit';
import { getTimer } from 'utils/api';

export const load: Load = async ({ fetch, url, params }) => {
	const timerData = await getTimer(params.id ?? '', fetch);
	return { fetch, url, params, timerData };
};

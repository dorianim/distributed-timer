import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ fetch, url, params }) => {
	return { fetch, url, params };
};

import { error, type Load } from '@sveltejs/kit';
import { get } from 'svelte/store';
import { API_URL } from '../stores';
import type { InstanceProperties } from '../types/instanceProperties';

export const load: Load = async ({ fetch, url, params }) => {
	const resp = await fetch(`${get(API_URL)}/instance`);
	if (!resp.ok) {
		throw error(resp.status, resp.statusText);
	}
	const instanceProperties: InstanceProperties = await resp.json();

	return { fetch, url, params, instanceProperties };
};

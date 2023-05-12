import { browser, dev } from '$app/environment';
import { readable } from 'svelte/store';

const API_HOST = 'timer.itsblue.de'; //dev ? 'localhost:3000' : browser ? window.location.host : '';
const API_SECURE = dev ? false : browser ? window.location.protocol === 'https:' : false;
export const API_URL = readable(`${API_SECURE ? 'https' : 'http'}://${API_HOST}/api`);
export const API_WS_URL = readable(`${API_SECURE ? 'wss' : 'ws'}://${API_HOST}/api/ws`);

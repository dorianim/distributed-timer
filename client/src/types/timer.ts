import type { Segment } from './segment';

export interface TimerCreationRequest {
	name: string;
	password: string;
	start_at: number;
	repeat: boolean;
	segments: Segment[];
}

export interface Timer {
	name: string;
	id: string;
	start_at: number;
	repeat: boolean;
	segments: Segment[];
}

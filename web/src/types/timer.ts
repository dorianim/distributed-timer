import type { Segment } from './segment';

export interface TimerCreationRequest {
	id: string;
	password: string;
	start_at: number;
	repeat: boolean;
	segments: Segment[];
}

export interface TimerUpdateRequest {
	start_at: number;
	repeat: boolean;
	segments: Segment[];
}

export interface Timer {
	id: string;
	start_at: number;
	repeat: boolean;
	segments: Segment[];
}

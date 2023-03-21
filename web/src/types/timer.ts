import type { DisplayOptions } from './displayOptions';
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
	stop_at?: number;
	repeat: boolean;
	display_options: DisplayOptions;
	segments: Segment[];
}

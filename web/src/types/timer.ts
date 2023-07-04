import type { DisplayOptions } from './displayOptions';
import type { Segment } from './segment';
import type { TimerMetadata } from './timerMetadata';

export interface TimerCreationRequest {
	id: string;
	password: string;
	start_at: number;
	repeat: boolean;
	segments: Segment[];
	metadata: TimerMetadata;
	display_options: DisplayOptions;
}

export interface TimerUpdateRequest {
	start_at: number;
	stop_at?: number;
	repeat: boolean;
	segments: Segment[];
	metadata: TimerMetadata;
	display_options: DisplayOptions;
}

export interface Timer {
	id: string;
	start_at: number;
	stop_at?: number;
	repeat: boolean;
	segments: Segment[];
	metadata: TimerMetadata;
	display_options: DisplayOptions;
}

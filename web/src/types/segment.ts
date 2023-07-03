export interface Sound {
	filename: string;
	trigger_time: number;
}

export interface Segment {
	label: string;
	time: number;
	color?: string;
	count_to: number;
	sounds: Sound[];
}

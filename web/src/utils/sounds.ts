import type { Sound } from 'types/segment';

export const soundPresets: Record<string, Sound[]> = {
	beepFourMinutesOneMinute_countdownFiveSeconds: [
		{
			filename: 'beep.mp3',
			trigger_time: 240
		},
		{
			filename: 'beep.mp3',
			trigger_time: 60
		},
		{
			filename: 'countdown.mp3',
			trigger_time: 5
		}
	],
	beepOneMinute_countdownFiveSeconds: [
		{
			filename: 'beep.mp3',
			trigger_time: 60
		},
		{
			filename: 'countdown.mp3',
			trigger_time: 5
		}
	]
};

export const soundsToString = (sounds: Sound[]): string => {
	return sounds
		.sort((a, b) => a.trigger_time - b.trigger_time)
		.map((sound) => `${sound.trigger_time}:${sound.filename}`)
		.join(',');
};

const presetSoundStrings = (() => {
	let presetSoundStrings: Record<string, string> = {};
	for (const [key, value] of Object.entries(soundPresets)) {
		presetSoundStrings[key] = soundsToString(value);
	}
	return presetSoundStrings;
})();

export const detectSoundPreset = (sounds: Sound[]): string | null => {
	const soundString = soundsToString(sounds);
	for (const [key, value] of Object.entries(presetSoundStrings)) {
		if (value === soundString) {
			return key;
		}
	}
	return null;
};

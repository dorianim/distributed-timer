<script lang="ts">
	import type { TimerCreationRequest } from '../../../types/timer';
	import { reporter, ValidationMessage } from '@felte/reporter-svelte';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';
	import type { Segment, Sound } from 'types/segment';
	import type { DisplayOptions } from 'types/displayOptions';
	import type { TimerMetadata } from 'types/timerMetadata';
	import { soundPresets } from 'utils/sounds';

	export let onSubmit: (timerData: TimerCreationRequest) => void;

	const defaultSounds: Sound[] = soundPresets.beepOneMinute_countdownFiveSeconds;

	const templates: {
		name: string;
		repeat: boolean;
		segments: Segment[];
		display_options: DisplayOptions;
		metadata: TimerMetadata;
	}[] = [
		{
			name: 'Boulder quali 4min + 15s',
			repeat: true,
			segments: [
				{
					label: 'Boulder',
					time: 230000,
					sounds: defaultSounds,
					color: '#26A269',
					count_to: 11000
				},
				{ label: 'Boulder', time: 11000, sounds: defaultSounds, color: '#A51D2D', count_to: 0 },
				{ label: 'Change', time: 14000, sounds: defaultSounds, color: '#E66100', count_to: 1000 }
			],
			display_options: {
				clock: true,
				pre_start_behaviour: 'RunNormally'
			},
			metadata: {
				delay_start_stop: 0
			}
		},
		{
			name: 'Boulder quali 5min + 15s',
			repeat: true,
			segments: [
				{
					label: 'Boulder',
					time: 290000,
					sounds: defaultSounds,
					color: '#26A269',
					count_to: 11000
				},
				{ label: 'Boulder', time: 11000, sounds: defaultSounds, color: '#A51D2D', count_to: 0 },
				{ label: 'Change', time: 14000, sounds: defaultSounds, color: '#E66100', count_to: 1000 }
			],
			display_options: {
				clock: true,
				pre_start_behaviour: 'RunNormally'
			},
			metadata: {
				delay_start_stop: 0
			}
		},
		{
			name: 'Boulder final 4min + wait',
			repeat: false,
			segments: [
				{
					label: 'Boulder',
					time: 230000,
					sounds: soundPresets.beepFourMinutesOneMinute_countdownFiveSeconds,
					color: '#26A269',
					count_to: 11000
				},
				{ label: 'Boulder', time: 11000, sounds: defaultSounds, color: '#A51D2D', count_to: 0 },
				{ label: 'Wait', time: 1000, sounds: [], color: '#1C71D8', count_to: 240000 }
			],
			display_options: {
				clock: true,
				pre_start_behaviour: 'ShowLastSegment'
			},
			metadata: {
				delay_start_stop: 3000
			}
		}
	];

	const schema = yup.object({
		name: yup
			.string()
			.matches(/^[a-zA-Z0-9-_]+$/, 'You can only use letters, numbers, - and _')
			.required('Please enter a name'),
		password: yup.string().required('Please enter a password'),
		segments: yup.number().required()
	});

	const { form } = createForm<yup.InferType<typeof schema>>({
		onSubmit: async (values) => {
			let formData: TimerCreationRequest = {
				id: values.name,
				password: values.password,
				start_at: new Date().getTime(),
				repeat: templates[values.segments].repeat,
				segments: templates[values.segments].segments,
				display_options: templates[values.segments].display_options,
				metadata: templates[values.segments].metadata
			};
			onSubmit(formData);
		},
		extend: [reporter, validator({ schema, castValues: true })]
	});
</script>

<form use:form class="w-full grid gap-3">
	<label class="label">
		<span>Enter a name for your timer:</span>
		<input
			name="name"
			class="input variant-form-material"
			type="text"
			placeholder="Only letters, numbers, - and _"
		/>
		<ValidationMessage for="name" let:messages>
			<span class="text-red-500">{messages?.[0] || ''}</span>
		</ValidationMessage>
	</label>

	<label class="label">
		<span>Enter a password to access your timer later:</span>
		<input
			name="password"
			class="input variant-form-material"
			type="password"
			placeholder="timer password"
		/>
		<ValidationMessage for="password" let:messages>
			<span class="text-red-500">{messages?.[0] || ''}</span>
		</ValidationMessage>
	</label>

	<label class="label">
		<span>The template to use:</span>
		<select class="select" name="segments">
			{#each templates as template, i}
				<option value={i}>{template.name}</option>
			{/each}
		</select>
	</label>

	<button class="btn variant-filled-primary" type="submit">Submit</button>
</form>

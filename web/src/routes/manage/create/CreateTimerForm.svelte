<script lang="ts">
	import type { TimerCreationRequest } from '../../../types/timer';
	import { reporter, ValidationMessage } from '@felte/reporter-svelte';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';

	export let onSubmit: (timerData: TimerCreationRequest) => void;

	const templates = [
		{
			name: 'Boulder 4min + 15s',
			segments: [
				{ label: 'Boulder', time: 231000, sound: true, color: '#26A269', count_to: 10000 },
				{ label: 'Boulder', time: 10000, sound: true, color: '#A51D2D', count_to: 0 },
				{ label: 'Change', time: 14000, sound: true, color: '#E66100', count_to: 1000 }
			]
		},
		{
			name: 'Boulder 4min',
			segments: [
				{ label: 'Boulder', time: 231000, sound: true, color: '#26A269', count_to: 10000 },
				{ label: 'Boulder', time: 10000, sound: true, color: '#A51D2D', count_to: 0 }
			]
		},
		{
			name: 'Simple',
			segments: [
				{
					label: 'Work',
					time: 4 * 60 * 1000,
					count_to: 1,
					sound: true
				}
			]
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
				repeat: true,
				segments: templates[values.segments].segments
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

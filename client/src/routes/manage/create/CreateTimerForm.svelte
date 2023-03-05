<script lang="ts">
	import type { TimerCreationRequest } from '../../../types/timer';

	export let onSubmit: (timerData: TimerCreationRequest) => void;

	const templates = [
		{
			name: 'Boulder 4min + 10s',
			segments: [
				{
					label: 'Switch',
					time: 10 * 1000,
					sound: true
				},
				{
					label: 'Boulder',
					time: 4 * 60 * 1000,
					sound: true
				}
			]
		},
		{
			name: 'Boulder 4min',
			segments: [
				{
					label: 'Boulder',
					time: 4 * 60 * 1000,
					sound: true
				}
			]
		},
		{
			name: 'Simple',
			segments: [
				{
					label: 'Work',
					time: 4 * 60 * 1000,
					sound: true
				}
			]
		}
	];

	let formData: TimerCreationRequest = {
		name: '',
		password: '',
		start_at: 0,
		repeat: false,
		segments: templates[0].segments
	};

	const handleSubmit = (e: Event) => {
		e.preventDefault();
		formData.start_at = new Date().getTime();
		onSubmit(formData);
	};
</script>

<form class="grid gap-3" on:submit={handleSubmit}>
	<label class="label">
		<span>Enter a name for your timer:</span>
		<input
			class="input variant-form-material"
			type="text"
			placeholder="timer name"
			required
			pattern="[a-z0-9-_]*"
			bind:value={formData.name}
		/>
	</label>

	<label class="label">
		<span>Enter a password to access your timer later:</span>
		<input
			class="input variant-form-material"
			type="password"
			placeholder="timer password"
			required
			bind:value={formData.password}
		/>
	</label>

	<label class="label">
		<span>The template to use:</span>
		<select class="select" bind:value={formData.segments}>
			{#each templates as template}
				<option value={template.segments}>{template.name}</option>
			{/each}
		</select>
	</label>

	<button class="btn variant-filled-primary">Submit</button>
</form>

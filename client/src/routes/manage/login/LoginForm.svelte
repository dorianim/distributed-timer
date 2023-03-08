<script lang="ts">
	export let onSubmit: (name: string, password: string) => void;

	const handleSubmit = async (e: Event) => {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const name = formData.get('name') as string;
		const password = formData.get('password') as string;
		onSubmit(name, password);
	};
</script>

<form class="grid gap-3" on:submit|preventDefault={handleSubmit}>
	<label class="label">
		<span>Enter the name of your timer:</span>
		<input
			class="input variant-form-material"
			type="text"
			placeholder="timer name"
			required
			name="name"
		/>
	</label>

	<label class="label">
		<span>Enter the password of your timer:</span>
		<input
			class="input variant-form-material"
			type="password"
			placeholder="timer password"
			required
			name="password"
		/>
	</label>

	<button class="btn variant-filled-primary">Submit</button>
</form>

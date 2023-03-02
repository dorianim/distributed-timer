<script lang="ts">
	export let onSubmit: (name: string, password: string) => void;

	const handleSubmit = async (e: Event) => {
		e.preventDefault();
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const name = formData.get('name') as string;
		const password = formData.get('password') as string;
		onSubmit(name, password);
	};
</script>

<form class="grid gap-3" on:submit={handleSubmit}>
	<label class="label">
		<span>Enter a name of your timer:</span>
		<input
			class="input variant-form-material"
			type="text"
			placeholder="timer name"
			required
			pattern="[a-z0-9-_]*"
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

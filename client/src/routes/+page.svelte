<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';

	let visible: boolean = true;

	const handleSubmit = (e: Event) => {
		e.preventDefault();
		if (!e.target) return;
		const data = new FormData(e.target as HTMLFormElement);
		const timerId = data.get('timerId');
		goto(`/t/${timerId}`);
	};
</script>

<div class="m-auto items-center grid gap-10 pt-10 p-4">
	<h2 class="text-center">Welcome to the boulder timer</h2>

	<div class="card p-4 md:w-[50%] m-auto">
		<form on:submit={handleSubmit}>
			<label class="label">
				<span>To view a timer, enter its ID:</span>
				<div class="input-group  grid-cols-2">
					<input
						class="input  variant-form-material"
						type="text"
						name="timerId"
						placeholder="timer id"
					/>

					<button class="variant-filled-primary">Submit</button>
				</div>
			</label>
		</form>
	</div>

	<div class="md:w-[50%] m-auto text-center">
		Alternatively, you can <a href="/manage/create">create a new timer</a> or
		<a href="/manage/login">manage an existing timer</a>
	</div>
</div>

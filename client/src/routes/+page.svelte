<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import { io } from 'socket.io-client';
	import { browser } from '$app/environment';

	let visible: boolean = true;

	if (browser) {
		const socket = new WebSocket('ws://131.159.218.193:3000/ws');

		// Connection opened
		socket.addEventListener('open', (event) => {
			socket.send('Hello AAAAAAAAAADRIAN!');
		});

		// Listen for messages
		socket.addEventListener('message', (event) => {
			console.log('Message from server ', event.data);
		});
	}
</script>

<div class="m-auto items-center grid gap-10 pt-10 p-4">
	<h2 class="text-center">Welcome to the boulder timer</h2>

	<div class="card p-4 md:w-[50%] m-auto">
		<form>
			<label class="label">
				<span>To view a timer, enter its ID:</span>
				<div class="input-group  grid-cols-2">
					<input class="input  variant-form-material" type="text" placeholder="timer id" />

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

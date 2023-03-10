<script lang="ts">
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';
	import DonationButton from './DonationButton.svelte';

	export let data: PageData;
	const { instanceProperties } = data;

	const handleSubmit = (e: Event) => {
		if (!e.target) return;
		const data = new FormData(e.target as HTMLFormElement);
		const timerId = data.get('timerId');
		goto(`/t/${timerId}`);
	};
</script>

<div class="m-auto items-center grid gap-5 pt-10 p-4">
	<h2 class="text-center">Welcome to the distributed timer</h2>

	<div class="card p-4 md:w-[50%] m-auto grid gap-2 grid-cols-2">
		<form on:submit|preventDefault={handleSubmit} class="col-span-2">
			<label class="label">
				<span>To view a timer, enter its name:</span>
				<div class="input-group flex">
					<input
						class="input  variant-form-material"
						type="text"
						name="timerId"
						placeholder="timer name"
					/>

					<button class="variant-filled-primary">Submit</button>
				</div>
			</label>
		</form>

		<a href="/manage/create" class="btn btn-sm variant-filled-secondary">Create a new timer</a>
		<a href="/manage/login" class="btn btn-sm variant-filled-tertiary ">Manage an existing timer</a>
	</div>

	{#if instanceProperties.demo || instanceProperties.donation}
		<div class="card p-4 md:w-[50%] m-auto">
			{#if instanceProperties.demo}
				<b class="text-primary-500 text-lg">Disclaimer:</b><br />
				This instance only serves demonstration purposes. We are trying our best to keep it online and
				free to use, but it
				<b class="text-primary-500">
					may be reset or taken down at any time without prior notice!
				</b>
				If you are interested in a supported instance of this timer, please
				<a href="mailto:boulder-timer@dorian.im">reach out to us</a>.
			{/if}

			{#if instanceProperties.demo && instanceProperties.donation}
				<br /><br />
			{/if}

			{#if instanceProperties.donation}
				If you like this timer and want to keep it free for everyone, please consider supporting us
				with a donation:

				{#each instanceProperties.donation as donationOption}
					<DonationButton type={donationOption.type} data={donationOption.data} />
				{/each}
			{/if}
		</div>
	{/if}

	<div class="card p-4 md:w-[50%] m-auto">
		This software is <b class="text-primary-500">free and open source</b>. You can find the source
		code on
		<a target="_blank" rel="noreferrer" href="https://github.com/dorianim/distributed-timer"
			>GitHub</a
		>. It is licensed under the
		<a
			target="_blank"
			rel="noreferrer"
			href="https://github.com/dorianim/distributed-timer/blob/main/LICENSE"
			>GNU Affero General Public License v3.0
		</a>.
	</div>
</div>

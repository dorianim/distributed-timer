<script lang="ts">
	export let time: number;
	export let label: string = '';
	let clazz: string = '';
	export { clazz as class };

	let seconds: number;
	let minutes: number;
	let hours: number;

	const updateParts = (time: number) => {
		time = Math.floor(time / 1000);
		seconds = time % 60;
		minutes = Math.floor(time / 60) % 60;
		hours = Math.floor(Math.floor(time / 60) / 60);
	};

	const updateTime = (hours: number, minutes: number, seconds: number) => {
		time = Math.floor(time / 1000);
		time = (hours * 60 * 60 + minutes * 60 + seconds) * 1000;
	};

	$: updateParts(time);
	$: updateTime(hours, minutes, seconds);
</script>

<label class="label min-w-[200px] {clazz}"
	>{label}

	<div class="flex flex-row gap-1 items-center font-bold text-xl">
		<input
			class="input variant-form-material"
			type="number"
			placeholder="hours"
			bind:value={hours}
			required
		/>
		:
		<input
			class="input variant-form-material"
			type="number"
			placeholder="minutes"
			bind:value={minutes}
			required
		/>
		:
		<input
			class="input variant-form-material"
			type="number"
			placeholder="seconds"
			bind:value={seconds}
			required
		/>
	</div>
</label>

<style>
	/* Chrome, Safari, Edge, Opera */
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	/* Firefox */
	input[type='number'] {
		-moz-appearance: textfield;
		appearance: textfield;
	}
</style>

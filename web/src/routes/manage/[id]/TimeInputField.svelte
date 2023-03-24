<script lang="ts">
	export let time: number;
	export let label: string | undefined = undefined;
	let timeString: string | undefined;

	const timeToTimeString = (time: number) => {
		return `${Math.floor(time / (1000 * 60))
			.toString()
			.padStart(2, '0')}:${(Math.floor(time / 1000) % 60).toString().padStart(2, '0')}`;
	};

	const timeStringToTime = (timeString: string) => {
		const time = timeString.split(':');
		return parseInt(time[0]) * 60 * 1000 + parseInt(time[1]) * 1000;
	};

	const handleTimeFieldChange = () => {
		if (!timeString) return;

		let tmpTime = timeString;
		tmpTime = tmpTime.replaceAll(':', '');
		while (tmpTime.length < 4) {
			tmpTime = '0' + tmpTime;
		}

		tmpTime =
			tmpTime.substring(0, tmpTime.length - 2) + ':' + tmpTime.substring(tmpTime.length - 2);

		while (tmpTime[0] === '0' && tmpTime.length > 5) {
			tmpTime = tmpTime.substring(1);
		}

		time = timeStringToTime(tmpTime);
	};

	$: timeString = timeToTimeString(time);
</script>

{#if label}
	<label class="label"
		>{label}
		<input
			class="input variant-form-material"
			type="text"
			placeholder="time in milliseconds"
			pattern="^[0-9]+:[0-9][0-9]$"
			bind:value={timeString}
			on:input={handleTimeFieldChange}
			required
		/>
	</label>
{:else}
	<input
		class="input variant-form-material"
		type="text"
		placeholder="time in milliseconds"
		pattern="^[0-9]+:[0-9][0-9]$"
		bind:value={timeString}
		on:input={handleTimeFieldChange}
		required
	/>
{/if}

<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	const { timerData } = data;

	// get the timer id from the url
	const timerId = data.params.timerId;
</script>

<h1>{timerData.name}</h1>

<script lang="ts">
	import type { Segment } from 'types/segment';
	import { getTimerText } from 'utils/timer';

	export let segment: Segment;

	let clazz = '';
	export { clazz as class };
</script>

<div
	class="p-4 col-span-2 h-full flex items-center gap-x-4 flex-wrap transition-colors duration-500 {clazz}"
	style="background-color: {segment.color ?? 'rgb(var(--color-surface-900))'};"
>
	<span class="text-2xl text-auto justify-self-start transition-colors duration-500"
		>{segment.label}:</span
	>

	<span class="text-2xl text-auto font-bold transition-colors duration-500"
		>{getTimerText(segment.time + segment.count_to - 1)} - {getTimerText(segment.count_to)}</span
	>
</div>

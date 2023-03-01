<script>
	import '../app.postcss';
	// Your selected Skeleton theme:
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';

	// This contains the bulk of Skeletons required styles:
	import '@skeletonlabs/skeleton/styles/all.css';
	import { autoModeWatcher } from '@skeletonlabs/skeleton';

	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	import '../app.postcss';
</script>

<svelte:head
	>{@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}</svelte:head
>

<div class="container mx-auto">
	<slot />
</div>

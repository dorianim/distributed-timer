<script>
	import '../app.postcss';
	// Your selected Skeleton theme:
	import '../theme.postcss';

	// This contains the bulk of Skeletons required styles:
	import '@skeletonlabs/skeleton/styles/all.css';
	import { autoModeWatcher, Modal } from '@skeletonlabs/skeleton';

	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	import '../app.postcss';

	export const prerender = false;
	export const ssr = false;
</script>

<svelte:head
	>{@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}</svelte:head
>

<div class="container mx-auto">
	<slot />
</div>

<Modal />

<script>
	import '../app.postcss';
	// Your selected Skeleton theme:
	import '../theme.postcss';

	import '@fontsource/roboto/400.css';
	import '@fontsource/roboto/500.css';

	// This contains the bulk of Skeletons required styles:
	import '@skeletonlabs/skeleton/styles/all.css';
	import { Modal } from '@skeletonlabs/skeleton';

	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	import '../app.postcss';
</script>

<div class="container mx-auto">
	<slot />
</div>

<Modal />

<style>
	:global(body) {
		font-family: 'Roboto', sans-serif !important;
	}
</style>

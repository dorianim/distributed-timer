<script>
	import { faQuestionCircle } from '@fortawesome/free-solid-svg-icons';
	import { popup } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import newUniqueId from 'locally-unique-id-generator';

	const id = newUniqueId();
</script>

<span>
	<div
		class="w-fit"
		use:popup={{
			event: 'hover',
			target: id,
			placement: 'top'
		}}
	>
		<Fa icon={faQuestionCircle} />
	</div>

	<div class="card variant-filled-primary p-4" data-popup={id}>
		<slot />
		<div class="arrow variant-filled-primary" />
	</div>
</span>

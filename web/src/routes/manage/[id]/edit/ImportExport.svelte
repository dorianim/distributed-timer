<script lang="ts">
	import { faFileExport } from '@fortawesome/free-solid-svg-icons';
	import { type ModalSettings, modalStore } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import ImportExportModal, { type ImportExportResult } from './ImportExportModal.svelte';
	import type { PageData } from './$types';
	import type { Timer } from 'types/timer';

	export let data: PageData;
	export let timerData: Timer;

	const { fetch } = data;

	const openImportExportModal = () => {
		const modal: ModalSettings = {
			type: 'component',
			component: { ref: ImportExportModal },
			meta: { timerData, fetch },

			response: (r?: ImportExportResult) => {
				if (r && r.action === 'Import') {
					timerData = r.newTimer;
				}
			}
		};
		modalStore.trigger(modal);
	};
</script>

<div class="w-full grid gap-2">
	<strong class="pt-4 text-lg">Danger zone:</strong>

	<button class="btn variant-filled-tertiary" on:click={openImportExportModal}>
		<span><Fa icon={faFileExport} /></span><span>Import / Export timer</span>
	</button>
</div>

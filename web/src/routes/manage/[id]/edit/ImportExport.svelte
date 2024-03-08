<script lang="ts">
	import {
		faCopy,
		faDeleteLeft,
		faEdit,
		faFileExport,
		faFileImport,
		faTrash
	} from '@fortawesome/free-solid-svg-icons';
	import {
		type ModalSettings,
		modalStore,
		ProgressRadial,
		type ModalComponent
	} from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';
	import ImportExportModal, { type ImportExportResult } from './ImportExportModal.svelte';
	import { loginTimer } from 'utils/api';
	import type { PageData } from './$types';
	import type { Timer } from 'types/timer';
	import type { ImportExportFormData } from './ImportExportForm.svelte';

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
					console.log('response!');
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

<script lang="ts" context="module">
	export interface ImportExportResult {
		newTimer: Timer;
		action: ImportExportAction;
	}
</script>

<script lang="ts">
	import { modalStore, ProgressRadial } from '@skeletonlabs/skeleton';
	import type { SvelteComponent } from 'svelte';
	import { getTimer, loginTimer, updateTimer } from 'utils/api';
	import Fa from 'svelte-fa';
	import { faCircleExclamation, faClose } from '@fortawesome/free-solid-svg-icons';
	import ImportExportForm, {
		type ImportExportAction,
		type ImportExportFormData
	} from './ImportExportForm.svelte';
	import type { TimerUpdateRequest, Timer } from 'types/timer';
	import type { Fetch } from 'types/fetch';

	export let parent: SvelteComponent;
	const { fetch, timerData }: { fetch: Fetch; timerData: Timer } = $modalStore[0].meta;

	let formData: ImportExportFormData = {
		action: 'Import',
		id: '',
		password: ''
	};
	let submitResult: Promise<Timer | void> = Promise.resolve();

	async function onSubmit() {
		if (formData.action === 'Import') {
			submitResult = handleImport(formData.id);
		} else if (formData.action === 'Export') {
			submitResult = handleExport(formData.id, formData.password, timerData);
		}

		try {
			let newTimer = await submitResult;
			if (!newTimer) return;

			let result: ImportExportResult = {
				newTimer,
				action: formData.action
			};

			if ($modalStore[0].response) $modalStore[0].response(result);
			modalStore.close();
		} catch (e) {
			console.log('Error during import/export: ', e);
		}
	}

	const handleImport = async (id: string): Promise<Timer> => {
		const timerResponse = await getTimer(id, fetch);
		const token = localStorage.getItem('token');
		if (!token) throw new Error('No token found');
		return updateTimer(timerData.id, timerResponse, token, fetch);
	};

	const handleExport = async (
		id: string,
		password: string,
		timerData: TimerUpdateRequest
	): Promise<Timer> => {
		const loginResponse = await loginTimer(id, password, fetch);
		return updateTimer(id, timerData, loginResponse.token, fetch);
	};

	async function onCancel() {
		parent.onClose();
	}
</script>

{#if $modalStore[0]}
	<div class="modal-example-form card p-4 w-modal shadow-xl space-y-4">
		{#await submitResult}
			<div class="flex items-center justify-center">
				<ProgressRadial class="w-10" />
			</div>
		{:then result}
			{#if result}
				Success!
			{:else}
				<ImportExportForm bind:formData {onSubmit} {onCancel} currentTimerId={timerData.id} />
			{/if}
		{:catch error}
			<div class="alert variant-ghost-error">
				<Fa icon={faCircleExclamation} class="text-2xl" />
				<div class="alert-message">
					<h3>Error</h3>
					<p>{error}</p>
				</div>
				<button
					class="btn-icon"
					on:click={() => {
						submitResult = Promise.resolve();
					}}><Fa icon={faClose} /></button
				>
			</div>
			<ImportExportForm {formData} {onSubmit} {onCancel} currentTimerId={timerData.id} />
		{/await}
	</div>
{/if}

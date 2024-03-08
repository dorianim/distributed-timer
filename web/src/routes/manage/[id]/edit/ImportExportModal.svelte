<script lang="ts" context="module">
	export interface ImportExportResult {
		newTimer: Timer;
		action: ImportExportAction;
	}
</script>

<script lang="ts">
	import {
		type ModalSettings,
		modalStore,
		ProgressRadial,
		TabGroup,
		TabAnchor,
		Tab
	} from '@skeletonlabs/skeleton';
	import type { SvelteComponent } from 'svelte';
	import type { PageData } from './$types';
	import { getTimer, loginTimer, updateTimer } from 'utils/api';
	import { page } from '$app/stores';
	import Fa from 'svelte-fa';
	import {
		faCircleExclamation,
		faClose,
		faFileExport,
		faFileImport,
		faWarning
	} from '@fortawesome/free-solid-svg-icons';
	import ImportExportForm, {
		type ImportExportAction,
		type ImportExportFormData
	} from './ImportExportForm.svelte';
	import { Result } from 'postcss';
	import type { TimerUpdateRequest, Timer } from 'types/timer';
	import type { Fetch } from 'types/fetch';
	import ImportExport from './ImportExport.svelte';

	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;
	const { fetch, timerData }: { fetch: Fetch; timerData: Timer } = $modalStore[0].meta;

	// Form Data
	let formData: ImportExportFormData = {
		action: 'Import',
		id: '',
		password: ''
	};
	let submitResult: Promise<Timer | void> = Promise.resolve();

	// We've created a custom submit function to pass the response and close the modal.
	async function onSubmit() {
		console.log('Handling import/export');
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
		} catch {}
	}

	const handleImport = async (id: string): Promise<Timer> => {
		const timerResponse = await getTimer(id, fetch);
		return updateTimer(timerData.id, timerResponse, localStorage.getItem('token')!, fetch);
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

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
</script>

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
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

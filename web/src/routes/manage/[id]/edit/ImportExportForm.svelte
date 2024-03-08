<script lang="ts" context="module">
	export type ImportExportAction = 'Import' | 'Export';
	export type ImportExportFormData = {
		action: ImportExportAction;
		id: string;
		password: string;
	};
</script>

<script lang="ts">
	import { faFileExport, faFileImport, faWarning } from '@fortawesome/free-solid-svg-icons';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';

	export let formData: ImportExportFormData;
	export let currentTimerId: string;
	export let onSubmit: () => void;
	export let onCancel: () => void;

	let formValid = false;

	$: {
		formValid = checkFormValid(formData);
	}

	const checkFormValid = (formData: ImportExportFormData): boolean => {
		if (formData.action === 'Import') {
			return formData.id.length > 0;
		} else {
			return formData.id.length > 0 && formData.password.length > 0;
		}
	};
</script>

<header class="text-2xl font-bold">Import/Export timer settings</header>
<article>
	Do you want to import (copy from another timer to this one) or export (copy from this one to
	another timer)?
</article>
<!-- Enable for debugging: -->

<TabGroup
	justify="justify-around"
	active="variant-filled-primary"
	hover="hover:variant-soft-primary"
	flex="flex-1 lg:flex-none"
	rounded="rounded-container-token"
	border=""
	class="bg-surface-100-800-token w-full"
>
	<Tab bind:group={formData.action} name="export" value={'Import'}>
		<svelte:fragment slot="lead"
			><Fa class="m-auto" size="4x" icon={faFileImport} /></svelte:fragment
		>
		<span class="pt-3 text-lg font-bold">Import</span>
	</Tab>

	<Tab bind:group={formData.action} name="import" value={'Export'}>
		<svelte:fragment slot="lead"
			><Fa class="m-auto" size="4x" icon={faFileExport} /></svelte:fragment
		>
		<span class="pt-3 text-lg font-bold">Export</span>
	</Tab>
</TabGroup>

<form class="modal-form border border-surface-500 p-4 space-y-4 rounded-container-token">
	<label class="label">
		<span>Other timer ID</span>
		<input class="input" type="text" required bind:value={formData.id} placeholder="Enter ID..." />
	</label>

	{#if formData.action === 'Export'}
		<label class="label">
			<span>Other timer password</span>
			<input
				class="input"
				type="password"
				required
				bind:value={formData.password}
				placeholder="Enter password..."
			/>
		</label>
	{/if}
</form>

{#if formValid}
	<aside class="alert variant-filled-warning">
		<!-- Icon -->
		<div><Fa icon={faWarning} /></div>

		<div class="alert-message">
			<h3 class="h3">Warning</h3>
			<p>
				This action will overwrite all setting of the timer
				<b>{formData.action === 'Import' ? currentTimerId : formData.id}</b> and cannot be undone!
			</p>
		</div>
	</aside>
{/if}

<footer class="modal-footer flex justify-end space-x-2">
	<button class="btn variant-ghost-surface" on:click={onCancel}> Cancel </button>
	<button class="btn variant-filled" on:click={onSubmit} disabled={!formValid}>
		Run {formData.action === 'Import' ? 'import' : 'export'}
	</button>
</footer>

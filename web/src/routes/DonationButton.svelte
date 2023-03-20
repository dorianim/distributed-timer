<script lang="ts">
	export let type: string;
	export let data: string;
</script>

{#if type === 'PayPal'}
	<form action="https://www.paypal.com/donate" method="post" target="_top">
		<input type="hidden" name="hosted_button_id" value={data} />
		<input
			type="image"
			src="https://www.paypalobjects.com/en_US/i/btn/btn_donate_LG.gif"
			name="submit"
			title="PayPal - The safer, easier way to pay online!"
			alt="Donate with PayPal button"
		/>
	</form>
{/if}

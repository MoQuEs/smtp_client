<script lang="ts">
	export let className = '';
</script>

<fieldset class="border-solid border-1 border-gray-300 dark:border-gray-600 p-3 rounded {className}">
	{#if $$slots.legend}
		<legend class="px-4">
			<slot name="legend" />
		</legend>
	{/if}
	<slot />
</fieldset>

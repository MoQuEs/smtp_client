<script lang="ts">
	import { getContext } from 'svelte';
	import { TABS, type Tabs } from './Tabs.svelte';

	const panel = {};

	const { registerPanel, selectedPanel } = getContext<Tabs>(TABS);

	registerPanel(panel);
</script>

{#if $selectedPanel === panel}
	<slot />
{/if}

<script lang="ts">
	import { getContext } from 'svelte';
	import { TABS, type Tabs } from './Tabs.svelte';

	export let className: string = '';

	const tab = {};

	const { registerTab, selectTab, selectedTab } = getContext<Tabs>(TABS);

	registerTab(tab);
</script>

<button
	class="px-3 py-2 border-b-2 {$selectedTab === tab ? 'border-primary-500' : 'border-secondary-500'}
    active:brightness-75 hover:brightness-125 {className}"
	on:click={() => selectTab(tab)}
>
	<slot />
</button>

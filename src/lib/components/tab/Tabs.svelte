<script context="module" lang="ts">
	export type Tabs = {
		registerTab: (tab: object) => void;
		registerPanel: (panel: object) => void;
		selectTab: (tab: object) => void;
		selectedTab: Writable<object[]>;
		selectedPanel: Writable<object[]>;
	};

	export const TABS = {};
</script>

<script lang="ts">
	import { setContext, onDestroy } from 'svelte';
	import { writable, type Writable } from 'svelte/store';

	const tabs: object[] = [];
	const panels: object[] = [];
	const selectedTab = writable<object>();
	const selectedPanel = writable<object>();

	setContext(TABS, {
		registerTab: (tab: object) => {
			tabs.push(tab);
			selectedTab.update((current) => current || tab);

			onDestroy(() => {
				const i = tabs.indexOf(tab);
				tabs.splice(i, 1);
				selectedTab.update((current) =>
					current === tab ? tabs[i] || tabs[tabs.length - 1] : current
				);
			});
		},

		registerPanel: (panel: object) => {
			panels.push(panel);
			selectedPanel.update((current) => current || panel);

			onDestroy(() => {
				const i = panels.indexOf(panel);
				panels.splice(i, 1);
				selectedPanel.update((current) =>
					current === panel ? panels[i] || panels[panels.length - 1] : current
				);
			});
		},

		selectTab: (tab: object) => {
			const i = tabs.indexOf(tab);
			selectedTab.set(tab);
			selectedPanel.set(panels[i]);
		},

		selectedTab,
		selectedPanel
	});
</script>

<div class="flex flex-col space-y-5">
	<slot />
</div>

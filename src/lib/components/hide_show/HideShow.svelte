<script lang="ts">
	import HideShowIcon from '../../components/hide_show/HideShowIcon.svelte';
	import Separator, { SeparatorSize } from '../../components/Separator.svelte';

	export let text: string;
	export let showHide = false;
</script>

<div class="flex flex-col">
	<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
	<div
		class="flex flex-row justify-between cursor-pointer text-black dark:text-white"
		on:click={() => (showHide = !showHide)}
		role="button" tabindex="0"
	>
		<span>{text}</span>

		<HideShowIcon bind:hidden={showHide} />
	</div>

	<Separator size={SeparatorSize.XS} />

	{#if showHide}
		<div class="px-3">
			<slot />
		</div>

		<Separator size={SeparatorSize.XS} />
	{/if}
</div>

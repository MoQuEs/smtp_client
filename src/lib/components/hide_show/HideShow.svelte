<script lang="ts">
	import HideShowIcon from '$src/lib/components/hide_show/HideShowIcon.svelte';
	import Separator, { SeparatorMarginSize, SeparatorSize } from '$components/Separator.svelte';

	export let text: string;
	export let showHide: boolean = false;
</script>

<div class="flex flex-col">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="flex flex-row justify-between cursor-pointer text-white"
		on:click={() => (showHide = !showHide)}
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

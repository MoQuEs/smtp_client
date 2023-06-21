<script lang="ts">
	import HideShowIcon from '$src/lib/components/hide_show/HideShowIcon.svelte';
	import Separator, { SeparatorSize } from '$components/Separator.svelte';

	export let text: string;
	export let showHide = false;

	const handleDummy = () => {};
</script>

<div class="flex flex-col">
	<div
		class="flex flex-row justify-between cursor-pointer text-white"
		on:click={() => (showHide = !showHide)}
		on:keydown={handleDummy}
		on:keyup={handleDummy}
		on:keypress={handleDummy}
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

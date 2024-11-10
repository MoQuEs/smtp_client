<script lang="ts">
	import { tooltip, tooltipTestPosition } from '$lib/stores/tooltips';
</script>

<div
	id="tooltip-container-test"
	style="left: {$tooltipTestPosition.x}px; top: {$tooltipTestPosition.y}px;"
	class="fixed max-w-75 text-white border-2 border-gray-600 bg-gray-800 py-1 px-2 rounded-md invisible z-99999 break-words whitespace-pre-line"
>
	{$tooltip.title}
</div>

{#if $tooltip.show}
	<div
		style="left: {$tooltip.position.x}px; top: {$tooltip.position.y}px;"
		class="fixed max-w-75 text-white border-2 border-gray-600 bg-gray-800 py-1 px-2 rounded-md z-99999 break-words whitespace-pre-line"
	>
		{$tooltip.title}
	</div>
{/if}

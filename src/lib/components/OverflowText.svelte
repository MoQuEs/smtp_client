<script lang="ts">
	import Tooltip from '../components/tooltip/Tooltip.svelte';

	export let text: string;
	export let className = '';
</script>

<Tooltip className="text-ellipsis overflow-hidden {className}" title={text}>
	<span class="self-center text-ellipsis overflow-hidden {className}">
		{text}
	</span>
</Tooltip>

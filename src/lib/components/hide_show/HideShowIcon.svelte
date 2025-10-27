<script lang="ts">
	import { Icon } from 'svelte-icons-pack';
	import { RiArrowsArrowUpSLine } from 'svelte-icons-pack/ri';
	import { RiArrowsArrowDownSLine } from 'svelte-icons-pack/ri';
	import { fillIconClass } from '$lib/stores/theme';

	export let hidden = false;

	export let color = 'white';
	export let size = '24';
</script>

<Icon
	className="cursor-pointer {$fillIconClass}"
	src={hidden ? RiArrowsArrowDownSLine : RiArrowsArrowUpSLine}
	{color}
	{size}
/>

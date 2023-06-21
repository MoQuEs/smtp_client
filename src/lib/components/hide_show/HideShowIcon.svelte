<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import RiSystemArrowUpSLine from 'svelte-icons-pack/ri/RiSystemArrowUpSLine';
	import RiSystemArrowDownSLine from 'svelte-icons-pack/ri/RiSystemArrowDownSLine';

	export let hidden = false;

	export let color = 'white';
	export let size = '24';
</script>

<Icon
	className="cursor-pointer"
	src={hidden ? RiSystemArrowDownSLine : RiSystemArrowUpSLine}
	{color}
	{size}
/>

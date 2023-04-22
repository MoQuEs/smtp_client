<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import RiSystemArrowUpSLine from 'svelte-icons-pack/ri/RiSystemArrowUpSLine';
	import RiSystemArrowDownSLine from 'svelte-icons-pack/ri/RiSystemArrowDownSLine';

	export let hidden: boolean = false;

	export let color: string = 'white';
	export let size: string = '24';
</script>

<Icon
	className="cursor-pointer"
	src={hidden ? RiSystemArrowDownSLine : RiSystemArrowUpSLine}
	{color}
	{size}
/>

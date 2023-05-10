<script lang="ts">
	import DropdownItem from './DropdownItem.svelte';

	export let text: string;
	export let className: string = '';
</script>

<DropdownItem alowHover={false}>
	<h4
		class="text-gray-400 font-semibold text-sm px-4 block w-full whitespace-nowrap bg-transparent {className}"
	>
		{text}
	</h4>
</DropdownItem>

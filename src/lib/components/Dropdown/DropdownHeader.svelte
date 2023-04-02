<script lang="ts">
	import DropdownItem from './DropdownItem.svelte';

	export let text: string;
</script>

<DropdownItem alowHover={false}>
	<h6
		class="text-gray-400 font-semibold text-sm py-2 px-4 block w-full whitespace-nowrap bg-transparent"
	>
		{text}
	</h6>
</DropdownItem>

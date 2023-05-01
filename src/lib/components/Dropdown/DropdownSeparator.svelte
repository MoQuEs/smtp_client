<script lang="ts">
	import DropdownItem from './DropdownItem.svelte';
	export let className: string = '';
</script>

<DropdownItem alowHover={false}>
	<hr class="h-0 my-1 border border-solid border-gray-300 opacity-25 {className}" />
</DropdownItem>

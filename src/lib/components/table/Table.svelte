<script context="module" lang="ts">
</script>

<script lang="ts">
	import SvelteTable, { type TableColumn } from 'svelte-table';


	export let rows: object[];

	export let columns: TableColumn<object>[] = [];
</script>

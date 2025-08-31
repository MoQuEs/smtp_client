<script context="module" lang="ts">
</script>

<script lang="ts">
	import SvelteTable, { type TableColumn } from 'svelte-table';


	export let data: object[];
	export let columns: TableColumn<object>[];

	let headers: string[] = [];
	for (const item of data) {
		for (const key in item) {
			if (!headers.includes(key)) {
				headers.push(key);
			}
		}
	}

	let rows: object[] = [];

	h: for (const header of headers) {
		for (const column of columns) {
			if (column.key === header) {
				break h;
			}
		}

		columns.push({
			key: header,
			title: header.charAt(0).toUpperCase() + header.slice(1).replace(/_/g, ' ')
		});
	}


</script>

<SvelteTable columns="{columns}" rows="{rows}"></SvelteTable>

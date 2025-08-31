<script lang="ts">
	import SvelteTable from 'svelte-table';

	type Row = {
		id: number;
		first_name: string;
		last_name: string;
		pet: string;
	};
</script>

<div class="flex flex-col">
	<SvelteTable columns="{columns}" rows="{rows}"></SvelteTable>
</div>

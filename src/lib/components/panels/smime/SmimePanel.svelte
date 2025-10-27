<script lang="ts">
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';
	import SmimeList from '$lib/components/panels/smime/SmimeList.svelte';
	import SmimeDropdown from '$lib/components/panels/smime/SmimeDropdown.svelte';
</script>

<div class="flex flex-col">
	<SmimeDropdown />

	<Separator size={SeparatorSize.XS} />

	<SmimeList />
</div>
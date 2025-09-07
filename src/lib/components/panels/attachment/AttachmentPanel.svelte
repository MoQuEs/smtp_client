<script lang="ts">
	import AttachmentDropdown from '$lib/components/panels/attachment/AttachmentDropdown.svelte';
	import AttachmentList from '$lib/components/panels/attachment/AttachmentList.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';
</script>

<div class="flex flex-col">
	<AttachmentDropdown />

	<Separator size={SeparatorSize.XS} />

	<AttachmentList />
</div>
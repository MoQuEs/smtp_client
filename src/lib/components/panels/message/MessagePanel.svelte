<script lang="ts">
	import Message from './Message.svelte';
	import MessageDropdown from '$lib/components/panels/message/MessageDropdown.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';
</script>

<div class="flex flex-col">
	<MessageDropdown />

	<Separator size={SeparatorSize.XS} />

	<Message />
</div>

<script lang="ts">
	import Message from './Message.svelte';
	import MessageList from '$lib/components/panels/message/MessageList.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';
</script>

<div class="flex flex-col">
	<MessageList />

	<Separator size={SeparatorSize.XS} />

	<Message />
</div>

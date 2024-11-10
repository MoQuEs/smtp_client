<script lang="ts">
	import SMTPMessage from '../../components/smtp/SMTPMessage.svelte';
	import SMTPMessageList from '../../components/smtp/SMTPMessageList.svelte';
	import Separator, { SeparatorSize } from '../../components/Separator.svelte';
</script>

<div class="flex flex-col">
	<SMTPMessageList />

	<Separator size={SeparatorSize.XS} />

	<SMTPMessage />
</div>

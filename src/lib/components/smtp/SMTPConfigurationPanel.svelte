<script lang="ts">
	import SMTPConfiguration from '../../components/smtp/SMTPConfiguration.svelte';
	import SMTPConfigurationList from '../../components/smtp/SMTPConfigurationList.svelte';
	import Separator, { SeparatorSize } from '../../components/Separator.svelte';
</script>

<div class="flex flex-col">
	<SMTPConfigurationList />

	<Separator size={SeparatorSize.XS} />

	<SMTPConfiguration />
</div>

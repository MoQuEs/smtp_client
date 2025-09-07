<script lang="ts">
	import Configuration from './Configuration.svelte';
	import ConfigurationDropdown from './ConfigurationDropdown.svelte';
	import Separator, { SeparatorSize } from '../../Separator.svelte';
</script>

<div class="flex flex-col">
	<ConfigurationDropdown />

	<Separator size={SeparatorSize.XS} />

	<Configuration />
</div>

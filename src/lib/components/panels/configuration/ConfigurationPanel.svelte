<script lang="ts">
	import Configuration from './Configuration.svelte';
	import ConfigurationList from './ConfigurationList.svelte';
	import Separator, { SeparatorSize } from '../../Separator.svelte';
</script>

<div class="flex flex-col">
	<ConfigurationList />

	<Separator size={SeparatorSize.XS} />

	<Configuration />
</div>

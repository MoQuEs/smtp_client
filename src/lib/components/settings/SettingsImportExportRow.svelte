<script lang="ts">
	import t from '../../i18n/translate';
	import Button, { ButtonPaddingSize, ButtonTheme } from '../../components/form/Button.svelte';
	import Tooltip from '../../components/tooltip/Tooltip.svelte';
	import { Icon } from 'svelte-icons-pack';
	import { CgImport } from 'svelte-icons-pack/cg';
	import { CgExport } from 'svelte-icons-pack/cg';

	export let text: string;
	export let importFn: () => void;
	export let exportFn: () => void;
</script>

<div class="flex flex-row items-center space-x-5">
	<span class="grow">{text}</span>

	<Tooltip title={$t('settings.import.title')}>
		<Button
			text=""
			theme={ButtonTheme.Success}
			padding={ButtonPaddingSize.SM}
			on:click={importFn}
		>
			<Icon src={CgImport} size="22" color="white" slot="icon" />
		</Button>
	</Tooltip>

	<Tooltip title={$t('settings.export.title')}>
		<Button
			text=""
			theme={ButtonTheme.Success}
			padding={ButtonPaddingSize.SM}
			on:click={exportFn}
		>
			<Icon src={CgExport} size="22" color="white" slot="icon" />
		</Button>
	</Tooltip>
</div>

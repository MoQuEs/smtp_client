<script lang="ts">
	import Icon from 'svelte-icons-pack';

	import BiRepost from 'svelte-icons-pack/bi/BiRepost';
	import AiOutlineMinus from 'svelte-icons-pack/ai/AiOutlineMinus';
	import AiOutlineDownload from 'svelte-icons-pack/ai/AiOutlineDownload';
	import AiOutlineUpload from 'svelte-icons-pack/ai/AiOutlineUpload';

	import t from '$i18n/translate';
	import {
		allConfigurations,
		customConfiguration,
		addConfiguration,
		removeConfiguration,
		repleaceConfiguration,
		loadConfiguration
	} from '$stores/smtp_configuration';
	import Input from '$components/form/Input.svelte';
	import Button, { ButtonTheme, ButtonPaddingSize } from '$components/form/Button.svelte';

	import Dropdown from '$components/dropdown/Dropdown.svelte';
	import DropdownItem from '$components/dropdown/DropdownItem.svelte';
	import DropdownSeparator from '$components/dropdown/DropdownSeparator.svelte';
	import Tooltip from '$components/tooltip/Tooltip.svelte';
</script>

<Dropdown text="Saved configurations">
	<DropdownItem>
		<div class="flex flex-row space-x-5">
			<Input
				className="flex-grow"
				placeholder={t('smtp.configuration.configuration_name')}
				bind:value={$customConfiguration.name}
			/>
			<Tooltip title={t('save')}>
				<Button theme={ButtonTheme.Success} text="" on:click={() => addConfiguration()}>
					<Icon src={AiOutlineDownload} size="22" color="white" slot="icon" />
				</Button>
			</Tooltip>
		</div>
	</DropdownItem>

	<DropdownSeparator />

	{#each $allConfigurations as confuguration, index}
		<DropdownItem>
			<div class="flex flex-row space-x-2">
				<span class="flex flex-grow text-base self-center">{confuguration.name}</span>
				<Tooltip title={t('load')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => loadConfiguration(index)}
					>
						<Icon src={AiOutlineUpload} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
				<Tooltip title={t('repleace')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => repleaceConfiguration(index)}
					>
						<Icon src={BiRepost} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
				<Tooltip title={t('remove')}>
					<Button
						text=""
						padding={ButtonPaddingSize.SM}
						on:click={() => removeConfiguration(index)}
					>
						<Icon src={AiOutlineMinus} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>
		</DropdownItem>
	{/each}
</Dropdown>

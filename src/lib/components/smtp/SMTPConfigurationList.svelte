<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import { BiRepost } from 'svelte-icons-pack/bi';
	import { AiOutlineMinus } from 'svelte-icons-pack/ai';
	import { RiDeviceSave3Line } from 'svelte-icons-pack/ri';
	import { RiDocumentContactsBookUploadLine } from 'svelte-icons-pack/ri';

	import { RiSystemFilter2Line } from 'svelte-icons-pack/ri';
	import { RiSystemFilter2Fill } from 'svelte-icons-pack/ri';

	import t from '../../i18n/translate';
	import {
		allConfigurations,
		customConfiguration,
		saveConfiguration,
		removeConfiguration,
		replaceConfiguration,
		loadConfiguration
	} from '../../stores/smtp_configuration';
	import Input from '../../components/form/Input.svelte';
	import Button, { ButtonTheme, ButtonPaddingSize } from '../../components/form/Button.svelte';

	import Dropdown from '../../components/dropdown/Dropdown.svelte';
	import DropdownItem from '../../components/dropdown/DropdownItem.svelte';
	import DropdownSeparator from '../../components/dropdown/DropdownSeparator.svelte';
	import Tooltip from '../../components/tooltip/Tooltip.svelte';
	import { getConfigurationLabelForSelect } from '../../utils/utils';
	import OverflowText from '../../components/OverflowText.svelte';

	let filter: boolean = false;
	$: filtered = $allConfigurations
		.filter((configuration) => {
			return (
				!filter ||
				configuration.name.indexOf($customConfiguration.name) !== -1 ||
				configuration.configuration.address.address.indexOf($customConfiguration.name) !== -1
			);
		})
		.sort((c1, c2) => c1.name.localeCompare(c2.name));
</script>

<Dropdown text={$t('smtp.configuration.name')}>
	<DropdownItem>
		<div class="flex flex-row space-x-5">
			<Input
				className="flex-grow"
				placeholder="{$t('smtp.configuration.configuration_name')} / {$t('smtp.configuration.configuration_filter')}"
				bind:value={$customConfiguration.name}
				iconAfter={filter ? RiSystemFilter2Fill : RiSystemFilter2Line}
				iconAfterInteractive={true}
				on:click_after={() => (filter = !filter)}
				iconAfterTooltip={filter ? $t('turn_off_filter') : $t('turn_on_filter')}
			/>
			<Tooltip title={$t('save')}>
				<Button theme={ButtonTheme.Success} text="" on:click={() => saveConfiguration()}>
					<Icon src={RiDeviceSave3Line} size="22" color="white" slot="icon" />
				</Button>
			</Tooltip>
		</div>
	</DropdownItem>

	<DropdownSeparator />

	{#if $allConfigurations.length === 0}
		<DropdownItem>
			{$t('smtp.configuration.no_configurations_saved')}
		</DropdownItem>
	{/if}

	{#each filtered as configuration, index}
		<DropdownItem>
			<div class="flex flex-row space-x-2">
				<OverflowText
					className="whitespace-pre-line"
					text={getConfigurationLabelForSelect(configuration.name, configuration.configuration)}
				/>
				<div class="flex flex-row flex-grow justify-end self-center space-x-2">
					<Tooltip title={$t('load')}>
						<Button
							text=""
							theme={ButtonTheme.Success}
							padding={ButtonPaddingSize.SM}
							on:click={() => loadConfiguration(configuration)}
						>
							<Icon src={RiDocumentContactsBookUploadLine} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
					<Tooltip title={$t('repleace')}>
						<Button
							text=""
							theme={ButtonTheme.Success}
							padding={ButtonPaddingSize.SM}
							on:click={() => replaceConfiguration(configuration)}
						>
							<Icon src={BiRepost} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
					<Tooltip title={$t('remove')}>
						<Button
							text=""
							theme={ButtonTheme.Error}
							padding={ButtonPaddingSize.SM}
							on:click={() => removeConfiguration(configuration)}
						>
							<Icon src={AiOutlineMinus} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
				</div>
			</div>
		</DropdownItem>
	{/each}
</Dropdown>

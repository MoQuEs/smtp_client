<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import { BiRepost } from 'svelte-icons-pack/bi';
	import { AiOutlineMinus } from 'svelte-icons-pack/ai';
	import { RiDeviceSave3Line } from 'svelte-icons-pack/ri';
	import { RiDocumentContactsBookUploadLine } from 'svelte-icons-pack/ri';

	import { RiSystemFilter2Line } from 'svelte-icons-pack/ri';
	import { RiSystemFilter2Fill } from 'svelte-icons-pack/ri';

	import t from '$lib/i18n/translate';
	import {
		allMessages,
		customMessage,
		saveMessage,
		removeMessage,
		repleaceMessage,
		loadMessage
	} from '$lib/stores/smtp_message';
	import Input from '$lib/components/form/Input.svelte';
	import Button, { ButtonTheme, ButtonPaddingSize } from '$lib/components/form/Button.svelte';
	import { theme } from '$lib/stores/theme';
	import { SettingsTheme } from '$lib/../generated/tauri';
	import Dropdown from '$lib/components/dropdown/Dropdown.svelte';
	import DropdownItem from '$lib/components/dropdown/DropdownItem.svelte';
	import DropdownSeparator from '$lib/components/dropdown/DropdownSeparator.svelte';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import { getMessageLabelForSelect } from '$lib/utils/utils';
	import OverflowText from '$lib/components/OverflowText.svelte';
	import Modal from '$lib/components/modal/Modal.svelte';

	let filterIconColor = $derived($theme == SettingsTheme.Dark ? 'white' : 'black');

	let filter: boolean = $state(false);
	let filtered = $derived($allMessages
		.filter((message) => {
			return (
				!filter ||
				message.name.indexOf($customMessage.name) !== -1 ||
				message.message.to.name?.indexOf($customMessage.name) !== -1 ||
				message.message.to.email.indexOf($customMessage.name) !== -1
			);
		})
		.sort((c1, c2) => c1.name.localeCompare(c2.name)));
</script>

<Dropdown text={$t('smtp.message.name')}>
	<DropdownItem>
		<div class="flex flex-row space-x-5">
			<Input
				className="flex-grow"
				placeholder="{$t('smtp.message.message_name')} / {$t('smtp.message.message_filter')}"
				bind:value={$customMessage.name}
				iconAfter={filter ? RiSystemFilter2Fill : RiSystemFilter2Line}
				iconAfterInteractive={true}
				iconAfterColor={filterIconColor}
				on:click_after={() => (filter = !filter)}
				iconAfterTooltip={filter ? $t('turn_off_filter') : $t('turn_on_filter')}
			/>
			<Tooltip title={$t('save')}>
				<Button theme={ButtonTheme.Success} text="" on:click={() => saveMessage()}>
					<Icon src={RiDeviceSave3Line} size="22" color="white" slot="icon" />
				</Button>
			</Tooltip>
		</div>
	</DropdownItem>

	<DropdownSeparator />

	{#if $allMessages.length === 0}
		<DropdownItem>
			{$t('smtp.message.no_messages_saved')}
		</DropdownItem>
	{/if}

	{#each filtered as message, index}
		<DropdownItem>
			<div class="flex flex-row space-x-2">
				<OverflowText
					className="whitespace-pre-line"
					text={getMessageLabelForSelect(message.name, message.message)}
				/>
				<div class="flex flex-row flex-grow justify-end self-center space-x-2">
					<Tooltip title={$t('load')}>
						<Button
							text=""
							theme={ButtonTheme.Success}
							padding={ButtonPaddingSize.SM}
							on:click={() => loadMessage(message)}
						>
							<Icon src={RiDocumentContactsBookUploadLine} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
					<Tooltip title={$t('repleace')}>
						<Button
							text=""
							theme={ButtonTheme.Success}
							padding={ButtonPaddingSize.SM}
							on:click={() => repleaceMessage(message)}
						>
							<Icon src={BiRepost} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
					<Tooltip title={$t('remove')}>
						<Button
							text=""
							theme={ButtonTheme.Error}
							padding={ButtonPaddingSize.SM}
							on:click={() => removeMessage(message)}
						>
							<Icon src={AiOutlineMinus} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
				</div>
			</div>
		</DropdownItem>
	{/each}
</Dropdown>

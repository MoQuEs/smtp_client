<script lang="ts">
	import Icon from 'svelte-icons-pack';

	import BiRepost from 'svelte-icons-pack/bi/BiRepost';
	import AiOutlineMinus from 'svelte-icons-pack/ai/AiOutlineMinus';
	import RiDeviceSave3Line from 'svelte-icons-pack/ri/RiDeviceSave3Line';
	import RiDocumentContactsBookUploadLine from 'svelte-icons-pack/ri/RiDocumentContactsBookUploadLine';

	import RiSystemFilter2Line from 'svelte-icons-pack/ri/RiSystemFilter2Line';
	import RiSystemFilter2Fill from 'svelte-icons-pack/ri/RiSystemFilter2Fill';

	import t from '$i18n/translate';
	import {
		allMessages,
		customMessage,
		saveMessage,
		removeMessage,
		repleaceMessage,
		loadMessage
	} from '$stores/smtp_message';
	import Input from '$components/form/Input.svelte';
	import Button, { ButtonTheme, ButtonPaddingSize } from '$components/form/Button.svelte';

	import Dropdown from '$components/dropdown/Dropdown.svelte';
	import DropdownItem from '$components/dropdown/DropdownItem.svelte';
	import DropdownSeparator from '$components/dropdown/DropdownSeparator.svelte';
	import Tooltip from '$components/tooltip/Tooltip.svelte';
	import { getMessageLabelForSelect } from '$utils/utils';
	import OverflowText from '$components/OverflowText.svelte';

	let filter: boolean = false;
	$: filtered = $allMessages
		.filter((message) => {
			return (
				!filter ||
				message.name.indexOf($customMessage.name) !== -1 ||
				message.message.to.name?.indexOf($customMessage.name) !== -1 ||
				message.message.to.email.indexOf($customMessage.name) !== -1
			);
		})
		.sort((c1, c2) => c1.name.localeCompare(c2.name));
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

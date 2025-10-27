<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import { RiDeviceSave3Line } from 'svelte-icons-pack/ri';

	import t from '$lib/i18n/translate';
	import {
		addAttachment,
		newAttachment
	} from '$lib/stores/attachment';
	import Button, { ButtonTheme } from '$lib/components/form/Button.svelte';
	import Dropdown from '$lib/components/dropdown/Dropdown.svelte';
	import DropdownItem from '$lib/components/dropdown/DropdownItem.svelte';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import Input, { InputType } from '$lib/components/form/Input.svelte';
	import { AddAttachmentFrom, addAttachmentFromFromString } from '$lib/api/tauri_classes';
	import type { SNEvent } from '$lib/utils/types';
	import Radio from '$lib/components/form/Radio.svelte';
	import Group from '$lib/components/form/Group.svelte';

	function onChange(e: SNEvent<HTMLInputElement>) {
		$newAttachment.from = addAttachmentFromFromString(e.currentTarget.value);
	}

	let attachments = $state(AddAttachmentFrom.File);
</script>

<Dropdown text={$t('attachment.add_attachment')}>
	<DropdownItem alowHover={false}>
		<div class="flex flex-col space-y-5">
			<div class="flex flex-row space-x-5">
				<Input
					className="flex-grow"
					placeholder={$t('attachment.attachment_name')}
					bind:value={$newAttachment.name}
				/>
				<Tooltip title={$t('save')}>
					<Button theme={ButtonTheme.Success} text="" on:click={() => addAttachment()}>
						<Icon src={RiDeviceSave3Line} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>

			<Group>
				<span slot="legend">{$t('attachment.type_group')}</span>
				<div class="flex flex-col space-y-5">
					<Radio name="newAttachmentType" change={onChange} value={AddAttachmentFrom.File} bind:group={attachments}
					       checked={true}>
						{$t('attachment.File')}
					</Radio>

					<Radio name="newAttachmentType" change={onChange} value={AddAttachmentFrom.Url} bind:group={attachments}>
						{$t('attachment.Url')}
					</Radio>

					{#if attachments === AddAttachmentFrom.Url}
						<div class="flex flex-row space-x-5 ml-6">
							<Input
								name="serverAuthUser"
								type={InputType.Text}
								placeholder={$t('attachment.url')}
								className="flex flex-grow"
								bind:value={$newAttachment.url}
							>
								<span slot="label">{$t('attachment.url')}</span>
							</Input>
						</div>
					{/if}
				</div>
			</Group>
		</div>
	</DropdownItem>
</Dropdown>

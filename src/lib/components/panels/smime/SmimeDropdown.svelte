<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import { RiDeviceSave3Line } from 'svelte-icons-pack/ri';

	import t from '$lib/i18n/translate';
	import {
		addSmime,
		newSmime
	} from '$lib/stores/smime';
	import Button, { ButtonTheme } from '$lib/components/form/Button.svelte';
	import Dropdown from '$lib/components/dropdown/Dropdown.svelte';
	import DropdownItem from '$lib/components/dropdown/DropdownItem.svelte';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import Input, { InputType } from '$lib/components/form/Input.svelte';
	import { AddSmimeFrom, addSmimeFromFromString } from '$lib/api/tauri_classes';
	import type { SNEvent } from '$lib/utils/types';
	import Radio from '$lib/components/form/Radio.svelte';
	import Group from '$lib/components/form/Group.svelte';

	function onChange(e: SNEvent<HTMLInputElement>) {
		$newSmime.from = addSmimeFromFromString(e.currentTarget.value);
	}

	let smimes = $state(AddSmimeFrom.PKCS12);
</script>

<Dropdown text={$t('smime.add_smime')}>
	<DropdownItem alowHover={false}>
		<div class="flex flex-col space-y-5">
			<div class="flex flex-row space-x-5">
				<Input
					className="flex-grow"
					placeholder={$t('smime.smime_name')}
					bind:value={$newSmime.name}
				/>
				<Tooltip title={$t('save')}>
					<Button theme={ButtonTheme.Success} text="" on:click={() => addSmime()}>
						<Icon src={RiDeviceSave3Line} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>

			<Input
				name="serverAuthUser"
				type={InputType.Text}
				placeholder={$t('smime.email')}
				className="flex flex-grow"
				bind:value={$newSmime.email}
			>
				<span slot="label">{$t('smime.email')}</span>
			</Input>

			<Group>
				<span slot="legend">{$t('smime.type_group')}</span>
				<div class="flex flex-col space-y-5">
					<Radio name="newSmimeType" change={onChange} value={AddSmimeFrom.PKCS12} bind:group={smimes}
					       checked={true}>
						{$t('smime.PKCS12')}
					</Radio>

					<Radio name="newSmimeType" change={onChange} value={AddSmimeFrom.Separate} bind:group={smimes}>
						{$t('smime.Separate')}
					</Radio>
				</div>
			</Group>
		</div>
	</DropdownItem>
</Dropdown>

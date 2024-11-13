<script lang="ts">
	import t from '../../i18n/translate';
	import { sendMail } from '../../api/tauri';
	import { allMessages, customMessage } from '../../stores/smtp_message';
	import { allConfigurations, customConfiguration } from '../../stores/smtp_configuration';
	import Button, { ButtonMode } from '../../components/form/Button.svelte';
	import Select, { SelectDispatch } from '../../components/form/Select.svelte';
	import { addToast } from '../../stores/toasts';
	import { ToastType } from '../../components/toast/Toast.svelte';
	import { get } from 'svelte/store';
	import { getConfigurationLabelForSelect, getMessageLabelForSelect } from '../../utils/utils';
	import Input, { InputType } from '$lib/components/form/Input.svelte';
	import * as cache from '$lib/stores/cache.svelte';

	let cc = cache.getSelectedConfiguration();
	let cm = cache.getSelectedMessage();
	let x = cache.getXMail();

	let configurations = [
		new SelectDispatch(
			getConfigurationLabelForSelect(
				$t('smtp.configuration.unsaved'),
				$customConfiguration.configuration
			),
			$customConfiguration,
			cc === undefined
		),
		...get(allConfigurations).map(
			(configuration) =>
				new SelectDispatch(
					getConfigurationLabelForSelect(configuration.name, configuration.configuration),
					configuration,
					cc !== undefined && cc.value.name === configuration.name
				)
		)
	];

	let messages = [
		new SelectDispatch(
			getMessageLabelForSelect($t('smtp.configuration.unsaved'), $customMessage.message),
			$customMessage,
			cm === undefined
		),
		...get(allMessages).map(
			(message) => new SelectDispatch(
				getMessageLabelForSelect(message.name, message.message),
				message,
				cm !== undefined && cm.value.name === message.name
			)
		)
	];

	let sendMailMode: ButtonMode = ButtonMode.Normal;
	const sendMailHandle = () => {
		sendMailMode = ButtonMode.Loading;

		const cc = cache.getSelectedConfiguration();
		const cm = cache.getSelectedMessage();

		if (cc === undefined || cm === undefined) {
			sendMailMode = ButtonMode.Normal;
			addToast({
				type: ToastType.Error,
				title: $t('api.send_mail.error')
			});

			return;
		}

		sendMail(
			cc.value.configuration,
			cm.value.message,
			cache.getXMail()
		).then(
			(response_data) => {
				sendMailMode = ButtonMode.Normal;

				if (response_data.success) {
					addToast({
						type: ToastType.Success,
						title: $t('api.send_mail.success')
					});
				} else {
					addToast({
						type: ToastType.Error,
						title: $t('api.send_mail.error')
					});
				}
			}
		);
	};
</script>

<div class="flex flex-row space-x-5">
	<div class="flex flex-col flex-grow space-y-5">
		<Select
			className="flex-grow"
			bind:selected={cc}
			bind:options={configurations}
			on:select={(selectEvent) => {
				cache.setSelectedConfiguration(selectEvent.detail);
			}}
		/>
		<Select
			className="flex-grow"
			bind:selected={cc}
			bind:options={messages}
			on:select={(selectEvent) => {
				cache.setSelectedMessage(selectEvent.detail);
			}}
		/>
	</div>

	<div class="flex flex-col flex-grow space-y-5">
		<Input
			name="sendXMail"
			type={InputType.Number}
			placeholder={$t('smtp.configuration.count')}
			className="flex flex-grow"
			bind:value={x}
			on:input={(inputEvent: CustomEvent<Number>) => {
				cache.setXMail(inputEvent.detail);
			}}
		/>
		<Button text={$t('smtp.send_mail')} className="" mode={sendMailMode} on:click={sendMailHandle} />
	</div>
</div>

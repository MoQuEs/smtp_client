<script lang="ts">
	import t from '$i18n/translate';
	import { sendMail } from '$api/tauri';
	import { allMessages, customMessage } from '$stores/smtp_message';
	import { allConfigurations, customConfiguration } from '$stores/smtp_configuration';
	import Button, { ButtonMode } from '$components/form/Button.svelte';
	import Select, { SelectDispatch } from '$components/form/Select.svelte';
	import { addToast } from '$stores/toasts';
	import { ToastType } from '$components/toast/Toast.svelte';
	import { get } from 'svelte/store';
	import type { NamedSMTPConfiguration, NamedSMTPMessage } from '$api/tauri_classes';
	import { getConfigurationLabelForSelect, getMessageLabelForSelect } from '$utils/utils';

	let selectedConfiguration: SelectDispatch<NamedSMTPConfiguration>;
	$: configurations = [
		new SelectDispatch(
			getConfigurationLabelForSelect(
				t('smtp.configuration.unsaved'),
				$customConfiguration.configuration
			),
			$customConfiguration
		),
		...get(allConfigurations).map(
			(configuration) =>
				new SelectDispatch(
					getConfigurationLabelForSelect(configuration.name, configuration.configuration),
					configuration
				)
		)
	];

	let selectedMessage: SelectDispatch<NamedSMTPMessage>;
	$: messages = [
		new SelectDispatch(
			getMessageLabelForSelect(t('smtp.configuration.unsaved'), $customMessage.message),
			$customMessage
		),
		...get(allMessages).map(
			(message) =>
				new SelectDispatch(getMessageLabelForSelect(message.name, message.message), message)
		)
	];

	let sendMailMode: ButtonMode = ButtonMode.Normal;
	const sendMailHandle = () => {
		sendMailMode = ButtonMode.Loading;

		sendMail(selectedConfiguration.value.configuration, selectedMessage.value.message).then(
			(response_data) => {
				sendMailMode = ButtonMode.Normal;

				if (response_data !== null && response_data.success) {
					addToast({
						type: ToastType.Success,
						title: t('api.send_mail.success')
					});
				} else {
					addToast({
						type: ToastType.Error,
						title: t('api.send_mail.error')
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
			bind:selected={selectedConfiguration}
			bind:options={configurations}
		/>
		<Select className="flex-grow" bind:selected={selectedMessage} bind:options={messages} />
	</div>

	<Button text={t('smtp.send_mail')} className="" mode={sendMailMode} on:click={sendMailHandle} />
</div>

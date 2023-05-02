<script lang="ts">
	import t from '$i18n/translate';
	import { sendMail } from '$api/tauri';
	import { smtp_message } from '$stores/smtp_message';
	import { allConfigurations, customConfiguration } from '$stores/smtp_configuration';
	import Button, { ButtonMode } from '$components/form/Button.svelte';
	import Select, { SelectDispatch } from '$components/form/Select.svelte';
	import { addToast } from '$stores/toasts';
	import { ToastType } from '$components/toast/Toast.svelte';
	import { get } from 'svelte/store';
	import type { NamedSMTPConfiguration } from '$api/tauri_classes';
	import { getConfigurationLabelForSelect } from '$utils/utils';

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

	let sendMailMode: ButtonMode = ButtonMode.Normal;
	const sendMailHandle = () => {
		sendMailMode = ButtonMode.Loding;

		sendMail(selectedConfiguration.value.configuration, $smtp_message.message).then(
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
	<Select
		className="flex-grow"
		bind:selected={selectedConfiguration}
		bind:options={configurations}
	/>

	<Button
		text={t('smtp.send_mail')}
		className="mb-5"
		mode={sendMailMode}
		on:click={sendMailHandle}
	/>
</div>

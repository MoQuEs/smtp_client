<script lang="ts">
	import t from '$lib/i18n/translate';
	import { sendMail } from '$lib/api/tauri';
	import { allMessages, customMessage } from '$lib/stores/message';
	import { allConfigurations, customConfiguration } from '$lib/stores/configuration';
	import Button, { ButtonMode } from '$lib/components/form/Button.svelte';
	import Select, { SelectDispatch } from '$lib/components/form/Select.svelte';
	import { addToast } from '$lib/stores/toasts';
	import { ToastType } from '$lib/components/toast/Toast.svelte';
	import { get } from 'svelte/store';
	import { getConfigurationLabelForSelect, getMessageLabelForSelect } from '$lib/utils/utils';
	import Input, { InputType } from '$lib/components/form/Input.svelte';
	import * as cache from '$lib/stores/cache.svelte.js';

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
			getMessageLabelForSelect($t('smtp.message.unsaved'), $customMessage.message),
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

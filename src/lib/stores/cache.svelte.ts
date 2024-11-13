import type { NamedSMTPConfiguration, NamedSMTPMessage } from '$lib/api/tauri_classes';
import type { SelectDispatch } from '$lib/components/form/Select.svelte';

let selectedConfiguration: SelectDispatch<NamedSMTPConfiguration> | undefined = $state(undefined);
let selectedMessage: SelectDispatch<NamedSMTPMessage> | undefined = $state(undefined);
let xMail: Number = $state(1);

export function getSelectedConfiguration(): SelectDispatch<NamedSMTPConfiguration> | undefined {
	return selectedConfiguration;
}

export function setSelectedConfiguration(configuration: SelectDispatch<NamedSMTPConfiguration>) {
	selectedConfiguration = configuration;
}

export function getSelectedMessage(): SelectDispatch<NamedSMTPMessage> | undefined {
	return selectedMessage;
}

export function setSelectedMessage(message: SelectDispatch<NamedSMTPMessage>) {
	selectedMessage = message;
}

export function getXMail(): Number {
	return xMail;
}

export function setXMail(x: Number) {
	xMail = x;
}

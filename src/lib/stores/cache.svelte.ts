import type { NamedConfiguration, NamedMessage } from '$lib/api/tauri_classes';
import type { SelectDispatch } from '$lib/components/form/Select.svelte';

let selectedConfiguration: SelectDispatch<NamedConfiguration> | undefined = $state(undefined);
let selectedMessage: SelectDispatch<NamedMessage> | undefined = $state(undefined);
let xMail: Number = $state(1);

export function getSelectedConfiguration(): SelectDispatch<NamedConfiguration> | undefined {
	return selectedConfiguration;
}

export function setSelectedConfiguration(configuration: SelectDispatch<NamedConfiguration>) {
	selectedConfiguration = configuration;
}

export function getSelectedMessage(): SelectDispatch<NamedMessage> | undefined {
	return selectedMessage;
}

export function setSelectedMessage(message: SelectDispatch<NamedMessage>) {
	selectedMessage = message;
}

export function getXMail(): Number {
	return xMail;
}

export function setXMail(x: Number) {
	xMail = x;
}

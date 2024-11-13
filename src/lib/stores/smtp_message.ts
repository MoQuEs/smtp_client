import * as tauriApi from '$lib/api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPMessage, type NamedSMTPMessages, type TauriResponse } from '$lib/api/tauri_classes';
import { clone } from '$lib/utils/utils';
import { addToast } from '$lib/stores/toasts';
import { ToastType } from '$lib/components/toast/Toast.svelte';
import { ts } from '$lib/i18n/translate';
import { error } from '@tauri-apps/plugin-log';
import { setConfigurations } from '$lib/stores/smtp_configuration';

export const customMessage: Writable<NamedSMTPMessage> = writable(new NamedSMTPMessage(''));
export const allMessages: Writable<NamedSMTPMessage[]> = writable([]);

export const setCustomMessages = (message: NamedSMTPMessage) => {
	customMessage.set(message);
};

export const setMessages = (messages: NamedSMTPMessages) => {
	allMessages.set([...messages]);
};

export const loadMessages = async () => {
	try {
		const messages = await tauriApi.getMessages();
		if (!messages.success || messages.data === undefined) {
			throw new Error('Error loading messages');
		}

		setMessages(messages.data);
	} catch (e) {
		addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('smtp.configuration.load_error')
		});
		await error('Error loading messages');
	}
};

export const saveMessage = () => {
	const cloned = cloneCustom();

	if (cloned.name === '') {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_cant_be_empty_error')
		});
	}

	if (get(allMessages).filter((message) => cloned.name === message.name).length > 0) {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_exists_error')
		});
	}

	tauriApi
		.saveMessage(cloned)
		.then((asd) => {
			console.log(asd);
			allMessages.update((all) => [...all, cloned]);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.message.save_error')
			});
			error('Error saving message');
		});
};

export const repleaceMessage = (messageToRepleace: NamedSMTPMessage) => {
	const cloned = cloneCustom();

	get(allMessages).forEach((message) => {
		if (message.name !== messageToRepleace.name) {
			cloned.name = message.name;
		}
	});

	tauriApi
		.saveMessage(cloned)
		.then(() => {
			allMessages.update((all) =>
				all.map((message) => {
					if (message.name !== messageToRepleace.name) {
						return cloned;
					}
					return message;
				})
			);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.message.repleace_error')
			});
			error('Error repleacing message');
		});
};

export const removeMessage = (messageToRemove: NamedSMTPMessage) => {
	tauriApi
		.removeMessage(messageToRemove)
		.then(() => {
			allMessages.update((all) =>
				all.filter((message) => {
					if (message.name !== messageToRemove.name) {
						return true;
					}

					return false;
				})
			);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.message.remove_error')
			});
			error('Error removing message');
		});
};

export const loadMessage = (messageToLoad: NamedSMTPMessage) => {
	const cloned = clone(messageToLoad);
	cloned.name = get(customMessage).name;
	customMessage.set(cloned);
};

const cloneCustom = (): NamedSMTPMessage => {
	return clone(get(customMessage));
};

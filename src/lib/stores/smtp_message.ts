import * as tauriApi from '$api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPMessage, type NamedSMTPMessages, type TauriResponse } from '$api/tauri_classes';
import { clone } from '$utils/utils';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import { ts } from '$i18n/translate';
import { error } from 'tauri-plugin-log-api';

export const customMessage: Writable<NamedSMTPMessage> = writable(new NamedSMTPMessage(''));
export const allMessages: Writable<NamedSMTPMessage[]> = writable([]);

export const setCustomMessages = (message: NamedSMTPMessage) => {
	customMessage.set(message);
};

export const setMessages = (messages: NamedSMTPMessages) => {
	allMessages.set([...messages]);
};

export const loadMessages = () => {
	tauriApi
		.getMessages()
		.then((messages: TauriResponse<NamedSMTPMessages>) => {
			if (messages.data !== undefined) {
				setMessages(messages.data);
			}
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.message.load_error')
			});
			error('Error loading messages');
		});
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

import * as tauriApi from '$api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPMessage } from '$api/tauri_classes';
import { clone } from '$utils/utils';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import t from '$i18n/translate';

export const customMessage: Writable<NamedSMTPMessage> = writable(new NamedSMTPMessage(''));
export const allMessages: Writable<NamedSMTPMessage[]> = writable([]);

export const setCustomMessages = (message: NamedSMTPMessage) => {
	customMessage.set(message);
};

export const setMessages = (messages: NamedSMTPMessage[]) => {
	allMessages.set([...messages]);
};

export const saveMessage = () => {
	let cloned = cloneCustom();

	if (cloned.name === '') {
		return addToast({
			title: t('ERROR'),
			type: ToastType.Error,
			text: t('name_cant_be_empty_error')
		});
	}

	if (get(allMessages).filter((message) => cloned.name === message.name).length > 0) {
		return addToast({
			title: t('ERROR'),
			type: ToastType.Error,
			text: t('name_exists_error')
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
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('smtp.message.saved')
			});
		});
};

export const repleaceMessage = (messageToRepleace: NamedSMTPMessage) => {
	let cloned = cloneCustom();

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
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('smtp.message.repleace_error')
			});
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
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('smtp.message.remove_error')
			});
		});
};

export const loadMessage = (messageToLoad: NamedSMTPMessage) => {
	let cloned = clone(messageToLoad);
	cloned.name = get(customMessage).name;
	customMessage.set(cloned);
};

const cloneCustom = (): NamedSMTPMessage => {
	return clone(get(customMessage));
};

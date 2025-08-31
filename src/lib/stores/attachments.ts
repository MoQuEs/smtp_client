import * as tauriApi from '$lib/api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import {
	NamedAttachment, type NamedAttachments
} from '$lib/api/tauri_classes';
import { clone } from '$lib/utils/utils';
import { addToast } from '$lib/stores/toasts';
import { ToastType } from '$lib/components/toast/Toast.svelte';
import { ts } from '$lib/i18n/translate';
import { error } from '@tauri-apps/plugin-log';

export const customAttachment: Writable<NamedAttachment> = writable(
	new NamedAttachment('')
);
export const allAttachments: Writable<NamedAttachment[]> = writable([]);

export const setCustomAttachment = (attachment: NamedAttachment) => {
	customAttachment.set(attachment);
};

export const setAttachments = (attachments: NamedAttachments) => {
	allAttachments.set([...attachments]);
};

export const loadAttachments = async () => {
	try {
		const attachments = await tauriApi.getAttachments();
		if (!attachments.success || attachments.data === undefined) {
			throw new Error('Error loading attachments');
		}
		setAttachments(attachments.data);
	} catch (e) {
		addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('attachment.load_error')
		});
		await error('Error loading attachments');
	}
};

export const saveAttachment = () => {
	const cloned = cloneCustomAttachment();

	if (cloned.name === '') {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_cant_be_empty_error')
		});
	}

	if (
		get(allAttachments).filter((attachment) => cloned.name === attachment.name).length > 0
	) {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_exists_error')
		});
	}

	tauriApi
		.saveAttachment(cloned)
		.then(() => {
			allAttachments.update((all) => [...all, cloned]);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('attachment.save_error')
			});
			error('Error saving attachment');
		});
};

export const replaceAttachment = (attachmentToReplace: NamedAttachment) => {
	const cloned = cloneCustomAttachment();
	cloned.name = attachmentToReplace.name;
	tauriApi
		.saveAttachment(cloned)
		.then(() => {
			allAttachments.update((all) =>
				all.map((attachment) => {
					if (attachment.name === attachmentToReplace.name) {
						return cloned;
					}
					return attachment;
				})
			);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('attachment.replace_error')
			});
			error('Error replacing attachment');
		});
};

export const removeAttachment = (attachmentToRemove: NamedAttachment) => {
	tauriApi
		.removeAttachment(attachmentToRemove)
		.then(() => {
			allAttachments.update((all) =>
				all.filter((attachment) => {
					if (attachment.name !== attachmentToRemove.name) {
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
				text: ts('attachment.remove_error')
			});
			error('Error removing attachment');
		});
};

export const loadAttachment = (attachmentToLoad: NamedAttachment) => {
	const cloned = clone(attachmentToLoad);
	cloned.name = get(customAttachment).name;
	customAttachment.set(cloned);
};

const cloneCustomAttachment = (): NamedAttachment => {
	return clone(get(customAttachment));
};


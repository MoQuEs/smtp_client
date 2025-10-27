import * as tauriApi from '$lib/api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import {
	AddSmime, AddSmimeFrom,
	NamedSmime, type NamedSmimes
} from '$lib/api/tauri_classes';
import { clone } from '$lib/utils/utils';
import { addToast } from '$lib/stores/toasts';
import { ToastType } from '$lib/components/toast/Toast.svelte';
import { ts } from '$lib/i18n/translate';
import { error } from '@tauri-apps/plugin-log';

export const filterSmime: Writable<string> = writable('');
export const newSmime: Writable<AddSmime> = writable(
	new AddSmime('', AddSmimeFrom.PKCS12, '', '', false)
);
export const allSmimes: Writable<NamedSmime[]> = writable([]);

export const setSmimes = (smimes: NamedSmimes) => {
	allSmimes.set([...smimes]);
};

export const loadSmimes = async () => {
	try {
		const smimes = await tauriApi.getSmimes();
		if (!smimes.success || smimes.data === undefined) {
			throw new Error('Error loading smimes');
		}
		setSmimes(smimes.data);
	} catch (e) {
		addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('smime.load_error')
		});
		await error('Error loading smimes');
	}
};

export const addSmime = () => {
	const cloned = cloneNew();

	if (cloned.name === '') {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_cant_be_empty_error')
		});
	}

	if (
		get(allSmimes).filter((smime) => cloned.name === smime.name).length > 0
	) {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_exists_error')
		});
	}

	tauriApi
		.addSmime(cloned)
		.then(loadSmimes)
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smime.save_error')
			});
			error('Error saving smime');
		});
};

export const removeSmime = (smimeToRemove: NamedSmime) => {
	tauriApi
		.removeSmime(smimeToRemove)
		.then(() => {
			allSmimes.update((all) =>
				all.filter((smime) => {
					return smime.name !== smimeToRemove.name;
				})
			);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smime.remove_error')
			});
			error('Error removing smime');
		});
};

const cloneNew = (): AddSmime => {
	return clone(get(newSmime));
};

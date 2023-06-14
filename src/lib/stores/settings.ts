import * as tauriApi from '$api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import type { Settings, TauriResponse } from '$api/tauri_classes';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import t from '$src/lib/i18n/translate';

export const settings: Writable<Settings> = writable();

export const setSettings = (settingsToSet: Settings) => {
	settings.set(settingsToSet);
};

export const loadSettings = () => {
	tauriApi
		.getSettings()
		.then((settings: TauriResponse<Settings>) => {
			if (settings.data !== undefined) {
				setSettings(settings.data);
			}
		})
		.catch(() => {
			addToast({
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('settings.load_error')
			});
		});
};

export const saveSettings = () => {
	tauriApi.saveSettings(get(settings)).catch(() => {
		addToast({
			title: t('ERROR'),
			type: ToastType.Error,
			text: t('settings.save_error')
		});
	});
};

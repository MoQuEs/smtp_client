import * as tauriApi from '$api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import type { Settings, TauriResponse } from '$api/tauri_classes';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import { ts, changeLocale } from '$src/lib/i18n/translate';
import { error } from 'tauri-plugin-log-api';

export const settings: Writable<Settings> = writable();

export const loadSettings = () => {
	tauriApi
		.getSettings()
		.then((settingsResponse: TauriResponse<Settings>) => {
			if (settingsResponse.data !== undefined) {
				settings.set(settingsResponse.data);
			}

			settings.subscribe((settings: Settings) => {
				if (settings === undefined) {
					return;
				}

				if (settings.language !== undefined) {
					changeLocale(settings.language.toString());
				}

				saveSettings();
			});
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('settings.load_error')
			});
			error('Error loading settings');
		});
};

const saveSettings = () => {
	tauriApi.saveSettings(get(settings)).catch(() => {
		addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('settings.save_error')
		});
		error('Error saving settings');
	});
};

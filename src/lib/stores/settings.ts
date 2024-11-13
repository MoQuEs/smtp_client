import * as tauriApi from '$lib/api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import type { Settings } from '$lib/api/tauri_classes';
import { addToast } from '$lib/stores/toasts';
import { setTheme } from '$lib/stores/theme';
import { ToastType } from '$lib/components/toast/Toast.svelte';
import { ts, locale } from '$lib/i18n/translate';
import { error } from '@tauri-apps/plugin-log';
import { SettingsLanguage } from '../../generated/tauri';

export const settings: Writable<Settings> = writable();

export const loadSettings = async () => {
	try {
		const settingsResponse = await tauriApi.getSettings();
		if (!settingsResponse.success || settingsResponse.data === undefined) {
			throw new Error('Error loading settings');
		}

		settings.set(settingsResponse.data);
		locale.set(SettingsLanguage[settingsResponse.data.language].toLowerCase());
		setTheme(settingsResponse.data.theme);

		settings.subscribe((settings: Settings) => {
			saveSettings();
		});
	} catch (e) {
		addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('smtp.configuration.load_error')
		});
		await error('Error loading settings');
	}
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

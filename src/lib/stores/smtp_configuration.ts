import * as tauriApi from '$lib/api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import {
	NamedSMTPConfiguration,
	type NamedSMTPConfigurations,
	type TauriResponse
} from '$lib/api/tauri_classes';
import { clone } from '$lib/utils/utils';
import { addToast } from '$lib/stores/toasts';
import { ToastType } from '$lib/components/toast/Toast.svelte';
import { ts } from '$lib/i18n/translate';
import { error } from '@tauri-apps/plugin-log';

export const customConfiguration: Writable<NamedSMTPConfiguration> = writable(
	new NamedSMTPConfiguration('')
);
export const allConfigurations: Writable<NamedSMTPConfiguration[]> = writable([]);

export const setCustomConfigurations = (configuration: NamedSMTPConfiguration) => {
	customConfiguration.set(configuration);
};

export const setConfigurations = (configurations: NamedSMTPConfigurations) => {
	allConfigurations.set([...configurations]);
};

export const loadConfigurations = () => {
	tauriApi
		.getConfigurations()
		.then((configurations: TauriResponse<NamedSMTPConfigurations>) => {
			if (configurations.data !== undefined) {
				setConfigurations(configurations.data);
			}
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.configuration.load_error')
			});
			error('Error loading configurations');
		});
};

export const saveConfiguration = () => {
	const cloned = cloneCustom();

	if (cloned.name === '') {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_cant_be_empty_error')
		});
	}

	if (
		get(allConfigurations).filter((configuration) => cloned.name === configuration.name).length > 0
	) {
		return addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('name_exists_error')
		});
	}

	tauriApi
		.saveConfiguration(cloned)
		.then(() => {
			allConfigurations.update((all) => [...all, cloned]);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.configuration.save_error')
			});
			error('Error saving configuration');
		});
};

export const repleaceConfiguration = (configurationToRepleace: NamedSMTPConfiguration) => {
	const cloned = cloneCustom();

	get(allConfigurations).forEach((configuration) => {
		if (configuration.name !== configurationToRepleace.name) {
			cloned.name = configuration.name;
		}
	});

	tauriApi
		.saveConfiguration(cloned)
		.then(() => {
			allConfigurations.update((all) =>
				all.map((configuration) => {
					if (configuration.name !== configurationToRepleace.name) {
						return cloned;
					}
					return configuration;
				})
			);
		})
		.catch(() => {
			addToast({
				title: ts('ERROR'),
				type: ToastType.Error,
				text: ts('smtp.configuration.repleace_error')
			});
			error('Error repleacing configuration');
		});
};

export const removeConfiguration = (configurationToRemove: NamedSMTPConfiguration) => {
	tauriApi
		.removeConfiguration(configurationToRemove)
		.then(() => {
			allConfigurations.update((all) =>
				all.filter((configuration) => {
					if (configuration.name !== configurationToRemove.name) {
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
				text: ts('smtp.configuration.remove_error')
			});
			error('Error removing configuration');
		});
};

export const loadConfiguration = (configurationToLoad: NamedSMTPConfiguration) => {
	const cloned = clone(configurationToLoad);
	cloned.name = get(customConfiguration).name;
	customConfiguration.set(cloned);
};

const cloneCustom = (): NamedSMTPConfiguration => {
	return clone(get(customConfiguration));
};

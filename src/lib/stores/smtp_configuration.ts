import * as tauriApi from '$api/tauri';
import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPConfiguration } from '$api/tauri_classes';
import { clone } from '$utils/utils';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import t from '$i18n/translate';

export const customConfiguration: Writable<NamedSMTPConfiguration> = writable(
	new NamedSMTPConfiguration('')
);
export const allConfigurations: Writable<NamedSMTPConfiguration[]> = writable([]);

export const setCustomConfigurations = (configuration: NamedSMTPConfiguration) => {
	customConfiguration.set(configuration);
};

export const setConfigurations = (configurations: NamedSMTPConfiguration[]) => {
	allConfigurations.set([...configurations]);
};

export const saveConfiguration = () => {
	let cloned = cloneCustom();

	if (cloned.name === '') {
		return addToast({
			title: t('ERROR'),
			type: ToastType.Error,
			text: t('name_cant_be_empty_error')
		});
	}

	if (
		get(allConfigurations).filter((configuration) => cloned.name === configuration.name).length > 0
	) {
		return addToast({
			title: t('ERROR'),
			type: ToastType.Error,
			text: t('name_exists_error')
		});
	}

	tauriApi
		.saveConfiguration(cloned)
		.then(() => {
			allConfigurations.update((all) => [...all, cloned]);
		})
		.catch(() => {
			addToast({
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('smtp.configuration.saved')
			});
		});
};

export const repleaceConfiguration = (configurationToRepleace: NamedSMTPConfiguration) => {
	let cloned = cloneCustom();

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
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('smtp.configuration.repleace_error')
			});
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
				title: t('ERROR'),
				type: ToastType.Error,
				text: t('smtp.configuration.remove_error')
			});
		});
};

export const loadConfiguration = (configurationToLoad: NamedSMTPConfiguration) => {
	let cloned = clone(configurationToLoad);
	cloned.name = get(customConfiguration).name;
	customConfiguration.set(cloned);
};

const cloneCustom = (): NamedSMTPConfiguration => {
	return clone(get(customConfiguration));
};

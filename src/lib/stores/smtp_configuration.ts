import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPConfiguration } from '$api/tauri';

export const customConfiguration: Writable<NamedSMTPConfiguration> = writable(
	new NamedSMTPConfiguration('Custom')
);
export const allConfigurations: Writable<NamedSMTPConfiguration[]> = writable([
	get(customConfiguration)
]);

export const addConfiguration = (configuration: NamedSMTPConfiguration) => {
	allConfigurations.update((all) => [...all, configuration]);
};

export const setConfigurations = (configurations: NamedSMTPConfiguration[]) => {
	allConfigurations.set([get(customConfiguration), ...configurations]);
};

export const selectCustomConfigurationByName = (name: string) => {
	get(allConfigurations).forEach((configuration) => {
		if (configuration.name === name) {
			customConfiguration.set(configuration);
		}
	});
};

export const setCustomConfiguration = (configuration: NamedSMTPConfiguration) => {
	customConfiguration.set(configuration);
};

export const getConfigurationByName = (name: string): NamedSMTPConfiguration => {
	let all = get(allConfigurations);
	for (let index = 0; index < all.length; index++) {
		if (all[index].name === name) {
			return all[index];
		}
	}

	return get(customConfiguration);
};

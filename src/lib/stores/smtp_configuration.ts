import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPConfiguration } from '$api/tauri';
import { clone } from '$utils/utils';

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

export const addConfiguration = () => {
	allConfigurations.update((all) => [...all, clone(get(customConfiguration))]);
};

export const getConfiguration = (indexToSelect: number): NamedSMTPConfiguration | null => {
	let configurationToReturn = null;
	get(allConfigurations).forEach((configuration, index) => {
		if (index === indexToSelect) {
			configurationToReturn = configuration;
		}
	});
	return configurationToReturn;
};

export const loadConfiguration = (indexToLoad: number) => {
	let configuration = getConfiguration(indexToLoad);
	if (configuration !== null) {
		customConfiguration.set(clone(configuration));
	}
};

export const repleaceConfiguration = (indexToRepleace: number) => {
	allConfigurations.update((all) =>
		all.map((configuration, index) =>
			index === indexToRepleace ? clone(get(customConfiguration)) : configuration
		)
	);
};

export const removeConfiguration = (indexToRemove: number) => {
	allConfigurations.update((all) => all.filter((_, index) => index !== indexToRemove));
};

import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPConfiguration } from '$api/tauri';

const empty = new NamedSMTPConfiguration('Empty');

export const smtp_configuration: Writable<NamedSMTPConfiguration> = writable(empty);
export const smtp_configurations: Writable<NamedSMTPConfiguration[]> = writable([empty]);

export const addSMTPConfiguration = (configuration: NamedSMTPConfiguration) => {
	smtp_configurations.update((all) => [...all, configuration]);
};

export const setSMTPConfigurations = (configurations: NamedSMTPConfiguration[]) => {
	smtp_configurations.set([empty, ...configurations]);
};

export const setSMTPConfigurationByName = (name: string) => {
	get(smtp_configurations).forEach((configuration) => {
		if (configuration.name === name) {
			smtp_configuration.set(configuration);
		}
	});
};

export const setSMTPConfiguration = (configuration: NamedSMTPConfiguration) => {
	smtp_configuration.set(configuration);
};

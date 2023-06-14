import t from '$src/lib/i18n/translate';
import type { SMTPConfiguration, SMTPMessage } from '$api/tauri_classes';

export const clone = <T = Object>(toClone: T): T => {
	if (typeof structuredClone === 'function') {
		return structuredClone(toClone);
	}

	return JSON.parse(JSON.stringify(toClone)) as T;
};

export const getConfigurationLabelForSelect = (
	name: string,
	configuration: SMTPConfiguration
): string => {
	if (configuration.auth.use_auth) {
		return t('smtp.configuration.select_label_with_auth', {
			name: name,
			login: configuration.auth.user,
			address: configuration.address.address,
			port: configuration.address.port
		});
	}

	return t('smtp.configuration.select_label', {
		name: name,
		address: configuration.address.address,
		port: configuration.address.port
	});
};

export const getMessageLabelForSelect = (name: string, message: SMTPMessage): string => {
	if (message.to.name !== '') {
		return t('smtp.message.select_label_with_name', {
			name: name,
			to_name: message.to.name,
			to_email: message.to.email
		});
	}

	return t('smtp.message.select_label', {
		name: name,
		to_email: message.to.email
	});
};

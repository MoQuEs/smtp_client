import type { SMTPConfiguration, SMTPMessage } from '$lib/api/tauri_classes';

export const clone = <T = object>(toClone: T): T => {
	if (typeof structuredClone === 'function') {
		return structuredClone(toClone);
	}

	return JSON.parse(JSON.stringify(toClone)) as T;
};

export const setVarsInText = (text: string, vars: object): string => {
	Object.entries(vars).forEach(([key, value]) => {
		const regex = new RegExp(`{{[ \t]*${key}[ \t]*}}`, 'g');
		text = text.replace(regex, value);
	});

	return text;
};

export const getConfigurationLabelForSelect = (
	name: string,
	configuration: SMTPConfiguration
): string => {
	if (configuration.auth.use_auth) {
		return setVarsInText('{{name}}\n[{{login}}:***@{{address}}:{{port}}]', {
			name: name,
			login: configuration.auth.user,
			address: configuration.address.address,
			port: configuration.address.port
		});
	}

	return setVarsInText('{{name}}\n[{{address}}:{{port}}]', {
		name: name,
		address: configuration.address.address,
		port: configuration.address.port
	});
};

export const getMessageLabelForSelect = (name: string, message: SMTPMessage): string => {
	if (message.to.name !== '') {
		return setVarsInText('{{name}}\n["{{to_name}}" <{{to_email}}>]', {
			name: name,
			to_name: message.to.name,
			to_email: message.to.email
		});
	}

	return setVarsInText('{{name}}\n[{{to_email}}]', {
		name: name,
		to_email: message.to.email
	});
};

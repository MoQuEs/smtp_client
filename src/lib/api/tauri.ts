import { invoke, type InvokeArgs } from '@tauri-apps/api/core';
import type {
	TauriResponse,
	SMTPConfiguration,
	SMTPMessage,
	NamedSMTPConfiguration,
	NamedSMTPConfigurations,
	NamedSMTPMessages,
	NamedSMTPMessage,
	Settings,
	Secret,
	ImportExportSettings
} from '$lib/api/tauri_classes';

export type Callback<T> = (response_data: TauriResponse<T>) => Promise<void>;
export type PTauriResponse<T> = Promise<TauriResponse<T>>;

export const sendMail = (
	configuration: SMTPConfiguration,
	message: SMTPMessage
): PTauriResponse<null> => {
	return callTauri('send_mail_command', { configuration, message });
};

export const getConfigurations = (): PTauriResponse<NamedSMTPConfigurations> => {
	return callTauri('get_configurations_command');
};

export const saveConfiguration = (configuration: NamedSMTPConfiguration): PTauriResponse<null> => {
	return callTauri('save_configuration_command', { configuration });
};

export const removeConfiguration = (
	configuration: NamedSMTPConfiguration
): PTauriResponse<null> => {
	return callTauri('remove_configuration_command', { configuration });
};

export const getMessages = (): PTauriResponse<NamedSMTPMessages> => {
	return callTauri('get_messages_command');
};

export const saveMessage = (message: NamedSMTPMessage): PTauriResponse<null> => {
	return callTauri('save_message_command', { message });
};

export const removeMessage = (message: NamedSMTPMessage): PTauriResponse<null> => {
	return callTauri('remove_message_command', { message });
};

export const setSecret = <T>(secret: Secret<T>): PTauriResponse<null> => {
	return callTauri('get_secret_command', { secret });
};

export const saveSecret = <T>(secret: Secret<T>): PTauriResponse<null> => {
	return callTauri('save_secret_command', { secret });
};

export const removeSecret = <T>(secret: Secret<T>): PTauriResponse<null> => {
	return callTauri('remove_secret_command', { secret });
};

export const getSettings = (): PTauriResponse<Settings> => {
	return callTauri('get_settings_command');
};

export const saveSettings = (settings: Settings): PTauriResponse<null> => {
	return callTauri('save_settings_command', { settings });
};

export const importData = (settings: ImportExportSettings): PTauriResponse<null> => {
	return callTauri('import_command', { settings });
};

export const exportFile = (settings: ImportExportSettings): PTauriResponse<null> => {
	return callTauri('export_command', { settings });
};

export async function callTauri<T>(
	function_name: string,
	data: InvokeArgs = {}
): PTauriResponse<T> {
	return (await invoke<TauriResponse<T>>(function_name, data)) as TauriResponse<T>;
}

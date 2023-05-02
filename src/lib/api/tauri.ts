import { invoke, type InvokeArgs } from '@tauri-apps/api/tauri';
import t from '$i18n/translate';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import type {
	TauriResponse,
	SMTPConfiguration,
	SMTPMessage,
	NamedSMTPConfiguration,
	NamedSMTPConfigurations
} from '$api/tauri_classes';

export type Callback<T> = (response_data: TauriResponse<T>) => Promise<void>;
export type PTauriResponse<T> = Promise<TauriResponse<T>>;

function isTauriResponse<T>(res: unknown): res is TauriResponse<T> {
	return (res as TauriResponse<T>) !== undefined;
}

export const sendMail = (
	configuration: SMTPConfiguration,
	message: SMTPMessage
): PTauriResponse<null> => {
	return callTauri('send_mail_command', { configuration, message });
};

export const getConfigurations = (): PTauriResponse<NamedSMTPConfigurations> => {
	return callTauri('get_configurations_command');
};

export const saveConfiguration = (
	configuration: NamedSMTPConfiguration
): PTauriResponse<NamedSMTPConfigurations> => {
	return callTauri('save_configuration_command', { configuration });
};

export const removeConfiguration = (
	configuration: NamedSMTPConfiguration
): PTauriResponse<NamedSMTPConfigurations> => {
	return callTauri('remove_configuration_command', { configuration });
};

async function callTauri<T>(function_name: string, data: InvokeArgs = {}): PTauriResponse<T> {
	return (await invoke<TauriResponse<T>>(function_name, data)) as TauriResponse<T>;
}

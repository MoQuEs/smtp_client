import { get, writable, type Writable } from 'svelte/store';
import { NamedSMTPMessage } from '$src/lib/api/tauri';

const empty = new NamedSMTPMessage('Empty');

export const smtp_message: Writable<NamedSMTPMessage> = writable(empty);
export const smtp_messages: Writable<NamedSMTPMessage[]> = writable([empty]);

export const addSMTPMessage = (message: NamedSMTPMessage) => {
	smtp_messages.update((all) => [...all, message]);
};

export const setSMTPMessages = (messages: NamedSMTPMessage[]) => {
	smtp_messages.set([empty, ...messages]);
};

export const setSMTPMessageByName = (name: string) => {
	get(smtp_messages).forEach((message) => {
		if (message.name === name) {
			smtp_message.set(message);
		}
	});
};

export const setSMTPMessage = (message: NamedSMTPMessage) => {
	smtp_message.set(message);
};

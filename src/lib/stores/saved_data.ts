import { writable, type Writable } from 'svelte/store';

export type SavedData = {
	servers: null;
	messages: null;
};

export type Header = {
	name: string;
	value: string;
};

export const saved_data: Writable<SavedData> = writable({
	servers: null,
	messages: null
});

export const servers = (servers: null) => {
	saved_data.update((data) => {
		data.servers = servers;
		return data;
	});
};

export const messages = (messages: null) => {
	saved_data.update((data) => {
		data.messages = messages;
		return data;
	});
};

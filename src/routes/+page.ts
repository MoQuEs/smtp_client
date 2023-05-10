import type { PageLoad } from './$types';
import { getConfigurations, getMessages } from '$api/tauri';
import { setConfigurations } from '$stores/smtp_configuration';
import { setMessages } from '$stores/smtp_message';

export const load = (async ({ params }) => {
	let configurations = await getConfigurations();
	if (configurations !== null && configurations.data !== undefined) {
		setConfigurations(configurations.data);
	}

	let messages = await getMessages();
	if (messages !== null && messages.data !== undefined) {
		setMessages(messages.data);
	}

	return {};
}) satisfies PageLoad;

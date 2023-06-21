import { loadSettings } from './../lib/stores/settings';
import type { PageLoad } from './$types';
import { loadConfigurations } from '$stores/smtp_configuration';
import { loadMessages } from '$stores/smtp_message';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const load = (async ({ params }) => {
	loadConfigurations();
	loadMessages();
	loadSettings();

	return {};
}) satisfies PageLoad;

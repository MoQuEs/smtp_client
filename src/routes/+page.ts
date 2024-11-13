import type { PageLoad } from './$types';
import { loadSettings } from '$lib/stores/settings';
import { loadConfigurations } from '$lib/stores/smtp_configuration';
import { loadMessages } from '$lib/stores/smtp_message';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const load = (async ({ params }) => {
	await Promise.all([
		loadConfigurations(),
		loadMessages(),
		loadSettings()
	]);

	return {};
}) satisfies PageLoad;

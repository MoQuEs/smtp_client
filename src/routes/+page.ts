import { loadSettings } from './../lib/stores/settings';
import type { PageLoad } from './$types';
import { loadConfigurations } from '$stores/smtp_configuration';
import { loadMessages } from '$stores/smtp_message';

export const load = (async ({ params }) => {
	await Promise.all([loadConfigurations(), loadMessages(), loadSettings()]);

	return {};
}) satisfies PageLoad;

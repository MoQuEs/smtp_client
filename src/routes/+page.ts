import type { PageLoad } from './$types';
import { loadSettings } from '$lib/stores/settings';
import { loadConfigurations } from '$lib/stores/configuration';
import { loadMessages } from '$lib/stores/message';
import { loadAttachments } from '$lib/stores/attachments';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const load = (async ({ params }) => {
	await Promise.all([
		loadConfigurations(),
		loadMessages(),
		loadAttachments(),
		loadSettings()
	]);

	return {};
}) satisfies PageLoad;

import type { PageLoad } from './$types';
import { setConfigurations } from '$stores/smtp_configuration';
import { getConfigurations } from '$api/tauri';

export const load = (async ({ params }) => {
	let data = await getConfigurations();
	if (data !== null && data.data !== undefined) {
		setConfigurations(data.data);
	}

	return {};
}) satisfies PageLoad;

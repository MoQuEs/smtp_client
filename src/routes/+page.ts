import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';
import { setConfigurations } from '$stores/smtp_configuration';
import { NamedSMTPConfiguration } from '$api/tauri';

export const load = (async ({ params }) => {
	let data = await invoke('get_saved_servers')
		.then((data) => data)
		.catch((data) => data);

	console.log(['page : get_saved_servers : ', data]);

	setConfigurations([
		new NamedSMTPConfiguration('Test 1'),
		new NamedSMTPConfiguration('Test 2'),
		new NamedSMTPConfiguration('Test 3')
	]);

	return {};
}) satisfies PageLoad;

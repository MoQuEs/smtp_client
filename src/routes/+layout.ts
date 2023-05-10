export const ssr = false;
export const prerender = true;
export const csr = true;

import type { LayoutLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

export const load = (async ({ params }) => {
	// let data = await invoke('get_saved_servers')
	// 	.then((data) => data)
	// 	.catch((data) => data);

	// console.log(['layout : get_saved_servers : ', data]);
	return {};
}) satisfies LayoutLoad;

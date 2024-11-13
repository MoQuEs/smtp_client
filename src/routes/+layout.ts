export const ssr = false;
export const prerender = true;
export const csr = true;

import type { LayoutLoad } from './$types';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const load = (async ({ params }) => {
	return {};
}) satisfies LayoutLoad;

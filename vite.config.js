import { createLogger } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

const logger = createLogger();

const originalWarning = logger.warn;
logger.warn = (
	/** @type {string} */ msg,
	/** @type {import('vite').LogOptions | undefined} */ options
) => {
	if (msg.includes('vite:css')) {
		if (
			msg.includes('Complex selectors in') &&
			msg.includes(' can not be transformed to an equivalent selector without \':is()\'')
		) {
			return;
		}
	}

	originalWarning(msg, options);
};

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
/** @type {import('vite').UserConfig} */
export default defineConfig({
	customLogger: logger,
	plugins: [sveltekit()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
				protocol: 'ws',
				host,
				port: 1421
			}
			: undefined,
		watch: {
			ignored: ['**/src-tauri/**']
		}
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		// @ts-ignore
		target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',

		// @ts-ignore
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,

		// @ts-ignore
		sourcemap: !!process.env.TAURI_DEBUG
	},
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});

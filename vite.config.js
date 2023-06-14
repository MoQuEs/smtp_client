import { createLogger, defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

const logger = createLogger();
const originalWarning = logger.warn;
logger.warn = (msg, options) => {
	if (
		msg.includes('vite:css') &&
		msg.includes('Complex selectors in') &&
		msg.includes(" can not be transformed to an equivalent selector without ':is()'")
	) {
		return;
	}
	originalWarning(msg, options);
};

// https://vitejs.dev/config/
/** @type {import('vite').UserConfig} */
export default defineConfig({
	customLogger: logger,
	plugins: [sveltekit()],
	clearScreen: true,
	server: {
		port: 1420,
		strictPort: true
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_DEBUG
	}
});

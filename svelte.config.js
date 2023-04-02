import staticAdapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: staticAdapter(),
		alias: {
			$src: 'src',
			'$src/*': 'src/*',
			$lib: 'src/lib',
			'$lib/*': 'src/lib/*',
			$components: 'src/lib/components',
			'$components/*': 'src/lib/components/*',
			$i18n: 'src/lib/i18n',
			'$i18n/*': 'src/lib/i18n/*',
			$stores: 'src/lib/stores',
			'$stores/*': 'src/lib/stores/*',
			$elements: 'src/lib/elements',
			'$elements/*': 'src/lib/elements/*',
			$api: 'src/lib/api',
			'$api/*': 'src/lib/api/*',
			$routes: 'src/routes',
			'$routes/*': 'src/routes/*',
			$generated: 'src/generated',
			'$generated/*': 'src/generated/*'
		}
	},
	preprocess: [vitePreprocess()]
};

export default config;

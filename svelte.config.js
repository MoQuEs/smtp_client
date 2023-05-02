import staticAdapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: staticAdapter(),
		alias: {
			$src: 'src',
			'$src/*': 'src/*',
			$assets: 'src/assets',
			'$assets/*': 'src/assets/*',
			$generated: 'src/generated',
			'$generated/*': 'src/generated/*',
			$lib: 'lib',
			'$lib/*': 'lib/*',
			$api: 'src/lib/api',
			'$api/*': 'src/lib/api/*',
			$components: 'src/lib/components',
			'$components/*': 'src/lib/components/*',
			$i18n: 'src/lib/i18n',
			'$i18n/*': 'src/lib/i18n/*',
			$stores: 'src/lib/stores',
			'$stores/*': 'src/lib/stores/*',
			$utils: 'src/lib/utils',
			'$utils/*': 'src/lib/utils/*',
			$routes: 'src/routes',
			'$routes/*': 'src/routes/*'
		}
	},
	preprocess: [vitePreprocess()]
};

export default config;

const colors = require('tailwindcss/colors');

function keyValueRange(min, max, step, sufixOnValue) {
	const obj = {};

	for (let i = min; i <= max; i = i + step ?? 1) {
		obj[i] = i.toString() + (sufixOnValue ?? '');
	}

	return obj;
}

function range(min, max, step, sufix) {
	const arr = [];

	for (let i = min; i <= max; i = i + step ?? 1) {
		arr.push(i.toString() + (sufix ?? ''));
	}

	return arr;
}

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {
			zIndex: {
				...keyValueRange(10, 100),
				...{
					999: '999',
					9999: '9999',
					99999: '99999'
				}
			},
			maxWidth: {
				25: '25%',
				50: '50%',
				75: '75%'
			},
			lineHeight: {
				11: '2.75rem',
				12: '3rem'
			},
			colors: {
				primary: {
					50: '#F0F7FF',
					100: '#DBEBFF',
					200: '#B3D5FF',
					300: '#80B9FF',
					400: '#338FFF',
					500: '#0055BD',
					600: '#004CA8',
					700: '#004394',
					800: '#00377A',
					900: '#002552',
					950: '#001938'
				},
				secondary: {
					50: '#FFF4E0',
					100: '#FFEBC7',
					200: '#FFD485',
					300: '#FFB62E',
					400: '#EB9800',
					500: '#BD7A00',
					600: '#AD7100',
					700: '#946000',
					800: '#7A5000',
					900: '#573800',
					950: '#382400'
				},
				success: colors.emerald,
				error: {
					50: '#FBF4F4',
					100: '#FAEBEB',
					200: '#F5D1D1',
					300: '#F2B0B0',
					400: '#F08A8A',
					500: '#F14747',
					600: '#E43434',
					700: '#CA2121',
					800: '#A42323',
					900: '#751F1F',
					950: '#451717'
				},
				warning: colors.yellow,
				info: colors.cyan,
				gray: {
					50: '#E7E8E9',
					100: '#D9DADD',
					200: '#BFC2CA',
					300: '#A4AAB6',
					400: '#8891A5',
					500: '#697591',
					600: '#545F78',
					700: '#3F495F',
					800: '#2C3344',
					900: '#181C26',
					950: '#0F1219'
				}
			},
			borderWidth: keyValueRange(0, 5, 1, 'px')
		},
		fontFamily: {
			sans: ['Ubuntu', 'sans-serif'],
			mono: ['Ubuntu Mono', 'monospace']
		},
		fontWeight: {
			light: '300',
			normal: '400',
			medium: '500',
			bold: '700'
		}
	},
	plugins: [require('tailwind-scrollbar'), require('flowbite/plugin')],
	darkMode: 'class'
};

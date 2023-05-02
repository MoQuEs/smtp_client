const colors = require('tailwindcss/colors');

function zIndex(min, max, step) {
	const obj = {};

	for (let i = min; i <= max; i = i + step ?? 1) {
		obj[i] = i.toString();
	}

	return obj;
}

function range(min, max, step) {
	const arr = [];

	for (let i = min; i <= max; i = i + step ?? 1) {
		arr.push(i);
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
				...zIndex(10, 100),
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
				primary: colors.rose,
				secondary: colors.cyan,
				success: colors.green,
				error: colors.red,
				warning: colors.orange,
				info: colors.sky
			},
			borderWidth: range(0, 5)
		}
	},
	darkMode: 'class',
	plugins: [require('tailwind-scrollbar'), require('flowbite/plugin')]
};

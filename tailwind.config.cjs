const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {},
		colors: {
			inherit: colors.inherit,
			current: colors.current,
			transparent: colors.transparent,
			black: colors.black,
			white: colors.white,
			primary: colors.rose,
			secondary: colors.cyan,
			success: colors.green,
			error: colors.red,
			warning: colors.orange,
			info: colors.sky,
			gray: colors.gray
		},
		borderWidth: [0, 1, 2, 3, 4, 5]
	},
	darkMode: 'class',
	plugins: [require('tailwind-scrollbar'), require('flowbite/plugin')]
};

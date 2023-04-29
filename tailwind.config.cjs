const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {
			zIndex: {
				100: '100',
				999: '999',
				9999: '9999',
				99999: '99999'
			},
			maxWidth: {
				25: '25%',
				50: '50%',
				75: '75%'
			},
			lineHeight: {
				11: '2.75rem',
				12: '3rem'
			}
		},
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

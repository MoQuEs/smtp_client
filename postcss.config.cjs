module.exports = {
	plugins: {
		'postcss-import': {},
		'postcss-nesting': {},
		'postcss-preset-env': {
			features: { 'nesting-rules': false }
		},
		'@tailwindcss/postcss': {},
		...(process.env.NODE_ENV === 'production' ? { cssnano: {} } : {})
	}
};

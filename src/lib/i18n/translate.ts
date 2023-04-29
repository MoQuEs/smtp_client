import { writable, type Writable, get } from 'svelte/store';
import type translate_arr from './translations/en';

export const locale: Writable<string> = writable('en');

export let locales: Array<string> = [];
let translations: Map<string, Map<string, string>> = new Map();

export function loadFilesFrompath(globPath: string) {
	const languagesModules: any = import.meta.glob('./translations/*.(js|ts)', {
		import: 'default',
		eager: true
	});

	for (const languageModule in languagesModules) {
		var language = languageModule
			.split('/')
			.reverse()[0]
			.replace(/\.[^/.]+$/, '');

		locales.push(language);
		translations.set(language, new Map(Object.entries(languagesModules[languageModule])));
	}
}

function translate(locale: string, key: string, vars: object): string {
	let text = translations.get(locale)?.get(key) ?? key;
	if (!text) {
		console.error(`No translation found for ${locale}.${key}`);
	}

	Object.entries(vars).forEach(([key, value], _) => {
		const regex = new RegExp(`{{[ \t]*${key}[ \t]*}}`, 'g');

		text = text.replace(regex, value);
	});

	return text;
}

loadFilesFrompath('./translations/*.(js|ts)');

export const currentLocale = () => get(locale);
export const allowedLocales = () => locales;
export const changeLocale = (localeToSet: (typeof locales)[number]) => locale.set(localeToSet);

type TranslateKeys = keyof typeof translate_arr;
export default (key: TranslateKeys, vars: object = {}) => translate(get(locale), key, vars);

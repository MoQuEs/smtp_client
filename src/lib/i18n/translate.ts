import { derived, writable, type Writable, get } from 'svelte/store';
import { setVarsInText } from '$lib/utils/utils';
import { ToastType } from '$lib/components/toast/Toast.svelte';
import { addToast } from '$lib/stores/toasts';
import { error } from '@tauri-apps/plugin-log';

interface Translations {
	[key: string]: any;
}

const locales: Array<string> = [];
const translations: Translations = {};
const locale: Writable<string> = writable('en');

const languagesModules: Record<string, object> = import.meta.glob('./translations/*.(js|ts)', {
	import: 'default',
	eager: true
});

for (const languageModule in languagesModules) {
	const language = languageModule
		.split('/')
		.reverse()[0]
		.replace(/\.[^/.]+$/, '');

	locales.push(language);
	translations[language] = languagesModules[languageModule];
}

function translate(locale: string, key: string, vars: object): string {
	const translationMap = translations[locale];
	if (translationMap === undefined) {
		console.error(`No translations found for ${locale}`);
		return key;
	}

	let text: string = key
		.toLowerCase()
		.split('.')
		.reduce((accumulator, currentValue: string) => {
			const translation = accumulator[currentValue];
			if (translation === undefined) {
				console.error(`No translation found for ${locale} for ${key}`);
				return key;
			}

			return translation;
		}, translationMap);

	text = setVarsInText(text, vars);

	return text;
}

export const currentLocale = () => get(locale);

export const allowedLocales = () => locales;

export const allowedLocale = (locale: string) =>
	allowedLocales().indexOf(locale.toLowerCase()) !== -1;

export const changeLocale = (localeToSet: string) => {
	localeToSet = localeToSet.toLowerCase();

	if (!allowedLocale(localeToSet)) {
		addToast({
			title: ts('ERROR'),
			type: ToastType.Error,
			text: ts('settings.locale_error')
		});

		error(`Locale ${localeToSet} is not allowed`);
	}

	locale.set(localeToSet.toLowerCase());
};

export const ts = (key: string, vars: object = {}) => translate(get(locale), key, vars);

export default derived(locale, () => ts);

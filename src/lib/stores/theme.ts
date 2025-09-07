import { get, writable, type Writable } from 'svelte/store';
import { SettingsTheme } from '../../generated/tauri';

export const theme: Writable<SettingsTheme> = writable(SettingsTheme.Dark);

export const getIconClass = (settingsTheme: SettingsTheme, light: string, dark: string): string => settingsTheme == SettingsTheme.Dark ? light : dark;

export const getFillIconClass = (settingsTheme: SettingsTheme): string => getIconClass(settingsTheme, 'icon-fill-gray-200', 'icon-fill-gray-800');
export const getStrokeIconClass = (settingsTheme: SettingsTheme): string => getIconClass(settingsTheme, 'icon-stroke-gray-200', 'icon-stroke-gray-800');

export const fillIconClass: Writable<string> = writable(getFillIconClass(get(theme)));
export const strokeIconClass: Writable<string> = writable(getStrokeIconClass(get(theme)));

theme.subscribe((newTheme) => {
	fillIconClass.set(getFillIconClass(newTheme));
	strokeIconClass.set(getStrokeIconClass(newTheme));
});

export const setTheme = (newTheme: SettingsTheme) => {
	if (newTheme === get(theme)) {
		return;
	}

	document.documentElement.classList.remove('light');
	document.documentElement.classList.remove('dark');

	switch (newTheme) {
		case SettingsTheme.Dark:
			document.documentElement.classList.add('dark');
			break;
		case SettingsTheme.Light:
			document.documentElement.classList.add('light');
			break;
	}

	theme.set(newTheme);
};

export const getTheme = () => get(theme);

export const isTheme = (t: string | SettingsTheme) => {
	let th: string | SettingsTheme = get(theme);
	if (typeof t === 'string') {
		th = th.toString();
	}

	return t == th;
};

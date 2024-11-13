import { get, writable, type Writable } from 'svelte/store';
import { SettingsTheme } from '../../generated/tauri';

export const theme: Writable<SettingsTheme> = writable(SettingsTheme.Dark);

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

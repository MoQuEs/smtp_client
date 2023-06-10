<script lang="ts">
	import * as tauri from '$generated/tauri';
	import t from '$i18n/translate';
	import { settings } from '$stores/settings';
	import Select, { SelectDispatch } from '$components/form/Select.svelte';

	$: settingsLanguages = Object.keys(tauri.SettingsLanguage).map(
		(language) => new SelectDispatch(t(`settings.language.${language}`), language)
	);

	$: settingsThemes = Object.keys(tauri.SettingsTheme).map(
		(theme) => new SelectDispatch(t(`settings.theme.${theme}`), theme)
	);
</script>

<div class="flex flex-col space-y-5">
	<div class="flex flex-row space-x-5">
		<div class="flex flex-row grow space-x-5">
			<Select
				className="flex-grow"
				on:selected={(selectEvent) => ($settings.language = selectEvent.detail.value)}
				bind:options={settingsLanguages}
			>
				<span slot="label">{t('settings.language')}</span>
			</Select>
		</div>
		<div class="flex flex-row grow space-x-5">
			<Select
				className="flex-grow"
				on:selected={(selectEvent) => ($settings.theme = selectEvent.detail.value)}
				bind:options={settingsThemes}
			>
				<span slot="label">{t('settings.theme')}</span>
			</Select>
		</div>
	</div>
</div>

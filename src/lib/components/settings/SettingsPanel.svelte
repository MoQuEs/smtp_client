<script lang="ts">
	import * as tauri from '../../../generated/tauri';
	import t, { changeLocale, getLocale } from '../../i18n/translate';
	import { loadSettings, settings } from '../../stores/settings';
	import Select, { SelectDispatch } from '../../components/form/Select.svelte';
	import HideShow from '../../components/hide_show/HideShow.svelte';
	import { exportFile, importFile } from '$lib/api/tauri';
	import { ImportExportSettings } from '$lib/api/tauri_classes';
	import SettingsImportExportRow from '$lib/components/settings/SettingsImportExportRow.svelte';
	import { loadConfigurations } from '$lib/stores/smtp_configuration';
	import { loadMessages } from '$lib/stores/smtp_message';
	import { addToast } from '$lib/stores/toasts';
	import { ToastType } from '$lib/components/toast/Toast.svelte';
	import Modal from '$lib/components/modal/Modal.svelte';
	import Input, { InputType } from '$lib/components/form/Input.svelte';
	import Button, { ButtonMode } from '$lib/components/form/Button.svelte';
	import { setTheme, getTheme } from '$lib/stores/theme';

	let changeTheme = (theme: tauri.SettingsTheme) => {
		$settings.theme = theme;
		setTheme(theme);
	};

	let settingsLanguages = $state(Object.keys(tauri.SettingsLanguage).map(
		(language) => {
			return new SelectDispatch($t(`settings.languages.${language}`), language, language === getLocale());
		}
	));

	let settingsThemes = $state(Object.keys(tauri.SettingsTheme).map(
		(theme) => new SelectDispatch($t(`settings.themes.${theme}`), theme, theme === getTheme())
	));

	const changeLanguage = (language: tauri.SettingsLanguage) => {
		$settings.language = language;
		changeLocale(language);
	};

	let importModal = $state(false);
	let importPassword = $state('');
	let importFnModal: () => void = () => {
	};
	const importFn = (
		smtp_configurations: boolean,
		smtp_messages: boolean,
		settings: boolean
	) => {
		return () => {
			importModal = true;
			importFnModal = () => {
				importFile(new ImportExportSettings(importPassword, smtp_configurations, smtp_messages, settings))
					.then(() => {
						loadConfigurations();
						loadMessages();
						loadSettings();

						importModal = false;
						importPassword = '';

						addToast({
							type: ToastType.Success,
							title: $t('settings.import.success')
						});
					})
					.catch(() => {
						addToast({
							type: ToastType.Error,
							title: $t('settings.import.error')
						});
					});
			};
		};
	};

	let exportModal = $state(false);
	let exportPassword = $state('');
	let exportFnModal: () => void = () => {
	};
	const exportFn = (
		smtp_configurations: boolean,
		smtp_messages: boolean,
		settings: boolean
	) => {
		return () => {
			exportModal = true;
			exportFnModal = () => {
				exportFile(new ImportExportSettings(exportPassword, smtp_configurations, smtp_messages, settings))
					.then(() => {
						exportModal = false;
						exportPassword = '';

						addToast({
							type: ToastType.Success,
							title: $t('settings.import.success')
						});
					})
					.catch(() => {
						addToast({
							type: ToastType.Error,
							title: $t('settings.import.error')
						});
					});
			};
		};
	};
</script>

<div class="flex flex-col space-y-5">
	<div class="flex flex-row space-x-5">
		<div class="flex flex-row grow space-x-5">
			<Select
				className="flex-grow"
				on:select={(selectEvent) => changeLanguage(selectEvent.detail.value)}
				bind:options={settingsLanguages}
			>
				<span slot="label">{$t('settings.language')}</span>
			</Select>
		</div>
		<div class="flex flex-row grow space-x-5">
			<Select
				className="flex-grow"
				on:select={(selectEvent) => changeTheme(selectEvent.detail.value)}
				bind:options={settingsThemes}
			>
				<span slot="label">{$t('settings.theme')}</span>
			</Select>
		</div>
	</div>
	<HideShow text={$t('settings.imports_exports')}>
		<div class="flex flex-col space-y-5">
			<SettingsImportExportRow
				text={$t('menu.all')}
				importFn={importFn(true, true, true)}
				exportFn={exportFn(true, true, true)} />

			<SettingsImportExportRow
				text={$t('menu.configurations')}
				importFn={importFn(true, false, false)}
				exportFn={exportFn(true, false, false)} />

			<SettingsImportExportRow
				text={$t('menu.messages')}
				importFn={importFn(false, true, false)}
				exportFn={exportFn(false, true, false)} />

			<SettingsImportExportRow
				text={$t('menu.settings')}
				importFn={importFn(false, false, true)}
				exportFn={exportFn(false, false, true)} />
		</div>
	</HideShow>
</div>

<Modal bind:showModal={importModal}>
	{#snippet header()}
		{$t('settings.import.title')}
	{/snippet}

	<div class="flex flex-col flex-grow space-y-5">
		<Input
			name="password"
			type={InputType.Password}
			placeholder={$t('settings.password')}
			className="flex flex-grow"
			bind:value={importPassword}
		/>

		<Button text={$t('settings.import.import')}
		        className="" mode={ButtonMode.Normal}
		        on:click={() => importFnModal()} />
	</div>
</Modal>

<Modal bind:showModal={exportModal}>
	{#snippet header()}
		{$t('settings.export.title')}
	{/snippet}

	<div class="flex flex-col flex-grow space-y-5">
		<Input
			name="password"
			type={InputType.Password}
			placeholder={$t('settings.password')}
			className="flex flex-grow"
			bind:value={exportPassword}
		/>

		<Button text={$t('settings.export.export')}
		        className="" mode={ButtonMode.Normal}
		        on:click={() => exportFnModal()} />
	</div>
</Modal>

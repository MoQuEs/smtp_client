<script lang="ts">
	import * as tauri from '$generated/tauri';
	import t, { changeLocale } from '$src/lib/i18n/translate';
	import { settings } from '$stores/settings';
	import Select, { SelectDispatch } from '$components/form/Select.svelte';
	import HideShow from '$components/hide_show/HideShow.svelte';
	import Button, { ButtonPaddingSize, ButtonTheme } from '$components/form/Button.svelte';
	import Tooltip from '$components/tooltip/Tooltip.svelte';
	import Icon from 'svelte-icons-pack';
	import CgImport from 'svelte-icons-pack/cg/CgImport';
	import CgExport from 'svelte-icons-pack/cg/CgExport';
	import type { SettingsLanguage } from '$generated/tauri';

	$: settingsLanguages = Object.keys(tauri.SettingsLanguage).map(
		(language) => new SelectDispatch($t(`settings.languages.${language}`), language)
	);

	$: settingsThemes = Object.keys(tauri.SettingsTheme).map(
		(theme) => new SelectDispatch($t(`settings.themes.${theme}`), theme)
	);

	const changeLanguage = (language: SettingsLanguage) => {
		$settings.language = language;
		changeLocale(language);
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
				on:select={(selectEvent) => ($settings.theme = selectEvent.detail.value)}
				bind:options={settingsThemes}
			>
				<span slot="label">{$t('settings.theme')}</span>
			</Select>
		</div>
	</div>
	<HideShow text={$t('imports_exports')}>
		<div class="flex flex-col space-y-5">
			<div class="flex flex-row items-center space-x-5">
				<span class="grow">{$t('menu.configurations')}</span>

				<Tooltip title={$t('import')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => {}}
					>
						<Icon src={CgImport} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>

				<Tooltip title={$t('export')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => {}}
					>
						<Icon src={CgExport} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>
			<div class="flex flex-row items-center space-x-5">
				<span class="grow">{$t('menu.messages')}</span>

				<Tooltip title={$t('import')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => {}}
					>
						<Icon src={CgImport} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>

				<Tooltip title={$t('export')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => {}}
					>
						<Icon src={CgExport} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>
			<div class="flex flex-row items-center space-x-5">
				<span class="grow">{$t('menu.settings')}</span>

				<Tooltip title={$t('import')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => {}}
					>
						<Icon src={CgImport} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>

				<Tooltip title={$t('export')}>
					<Button
						text=""
						theme={ButtonTheme.Success}
						padding={ButtonPaddingSize.SM}
						on:click={() => {}}
					>
						<Icon src={CgExport} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>
		</div>
	</HideShow>
</div>

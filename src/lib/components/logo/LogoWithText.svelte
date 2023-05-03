<script lang="ts">
	import t from '$i18n/translate';
	import logo from '$assets/logo/logo.svg';
</script>

<div class="flex flex-row h-11 space-x-1">
	<img src={logo} alt={t('logo')} class="h-full" />

	<div class="flex">
		<span class="text-3xl text-primary-500 font-bold self-start">{t('title-1')}</span>

		<span class="text-3xl text-info-500 font-medium -ml-1 self-end">{t('title-2')}</span>
	</div>
</div>

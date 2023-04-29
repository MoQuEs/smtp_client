<script lang="ts">
	import t from '$i18n/translate';
	import logo from '$assets/logo/logo.svg';
</script>

<div class="flex flex-row h-11">
	<div class="flex flex-row mr-1">
		<span class="text-3xl text-primary-500 font-bold self-start">{t('title-1')}</span>
		<span class="text-3xl text-info-500 font-medium -ml-1 self-end">{t('title-2')}</span>
	</div>

	<img src={logo} alt={t('logo')} class="h-11" />
</div>

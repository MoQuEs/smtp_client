<script lang="ts">
	import t from '$i18n/translate';
	import logo from '$assets/logo.svg';
	import logoText from '$assets/logo_text.svg';
</script>

<div class="flex flex-row h-11 space-x-1">
	<img src={logo} alt={t('logo')} class="h-full" />
	<img src={logoText} alt={t('logo.text')} class="h-full" />
</div>

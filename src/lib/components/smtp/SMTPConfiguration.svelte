<script lang="ts">
	import t from '$i18n/translate';
	import { slide } from 'svelte/transition';
	import Checkbox from '$components/form/Checkbox.svelte';
	import Input, { InputType } from '$components/form/Input.svelte';
	import type { NamedSMTPConfiguration } from '$api/tauri';

	export let configuration: NamedSMTPConfiguration;
</script>

<div class="flex flex-col space-y-5">
	<div class="flex flex-row space-x-5">
		<Input
			name="serverAddress"
			type={InputType.Text}
			placeholder={t('smtp.configuration.address')}
			className="flex flex-grow"
			bind:value={configuration.configuration.address.address}
		>
			<span slot="label">{t('smtp.configuration.address')}</span>
		</Input>
		<Input
			name="serverPort"
			type={InputType.Number}
			placeholder={t('smtp.configuration.port')}
			bind:value={configuration.configuration.address.port}
		>
			<span slot="label">{t('smtp.configuration.port')}</span>
		</Input>
	</div>

	<Checkbox name="serverUseAuth" bind:checked={configuration.configuration.auth.use_auth}
		>{t('smtp.configuration.use_auth')}</Checkbox
	>

	{#if configuration.configuration.auth.use_auth}
		<div class="flex flex-row space-x-5 ml-6" transition:slide>
			<Input
				name="serverAuthUser"
				type={InputType.Text}
				placeholder={t('smtp.configuration.user')}
				className="flex flex-grow"
				disabled={!configuration.configuration.auth.use_auth}
				readonly={!configuration.configuration.auth.use_auth}
				bind:value={configuration.configuration.auth.user}
			>
				<span slot="label">{t('smtp.configuration.user')}</span>
			</Input>
			<Input
				name="ServerAuthPassword"
				type={InputType.Password}
				placeholder={t('smtp.configuration.password')}
				className="flex flex-grow"
				disabled={!configuration.configuration.auth.use_auth}
				readonly={!configuration.configuration.auth.use_auth}
				bind:value={configuration.configuration.auth.password}
			>
				<span slot="label">{t('smtp.configuration.password')}</span>
			</Input>
		</div>
	{/if}

	<Checkbox name="serverRequireSSL" bind:checked={configuration.configuration.require_ssl}
		>{t('smtp.configuration.require_ssl')}</Checkbox
	>

	<Checkbox
		name="serverVerifyCertificates"
		bind:checked={configuration.configuration.verify_certificates}
		>{t('smtp.configuration.verify_certificates')}</Checkbox
	>
</div>

<script lang="ts">
	import t from '$i18n/translate';
	import { slide } from 'svelte/transition';
	import Separator from '$components/Separator.svelte';
	import Hide from '$components/Hide.svelte';
	import type { NamedSMTPConfiguration } from '$api/tauri';
	import SMTPConfiguration from './SMTPConfiguration.svelte';

	export let configuration: NamedSMTPConfiguration;
	let showParams = true;
</script>

<div class="flex flex-col">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="flex flex-row justify-between cursor-pointer"
		on:click={() => (showParams = !showParams)}
	>
		<h1>{t('smtp.configuration.title')}</h1>
		{#if configuration.configuration.address.address != ''}
			<span class="text-info-400 text-sm"
				>{configuration.configuration.address.address}:{configuration.configuration.address
					.port}</span
			>
		{/if}
		<Hide bind:hidden={showParams} />
	</div>

	<Separator />

	{#if showParams}
		<div transition:slide>
			<SMTPConfiguration bind:configuration />

			<Separator />
		</div>
	{/if}
</div>

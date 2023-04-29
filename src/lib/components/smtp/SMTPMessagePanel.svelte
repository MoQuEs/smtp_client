<script lang="ts">
	import { convert } from 'html-to-text';
	import Separator from '$components/Separator.svelte';
	import { NamedSMTPMessage, SMTPMessageHeader } from '$api/tauri';
	import Hide from '$components/Hide.svelte';
	import { slide } from 'svelte/transition';
	import t from '$i18n/translate';
	import SMTPMessage from './SMTPMessage.svelte';

	export let message: NamedSMTPMessage;

	let showParams: boolean = true;
	let convertHTMLToTEXT: boolean = true;

	const addHeader = () => {
		message.message.headers = [...message.message.headers, new SMTPMessageHeader('', '')];
	};

	const removeHeader = (index: number) =>
		(message.message.headers = message.message.headers.filter(
			(_, headerIndex) => index !== headerIndex
		));

	$: {
		if (message.message.body.html && convertHTMLToTEXT) {
			message.message.body.text = convert(message.message.body.html);
		}
	}
</script>

<div class="flex flex-col">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="flex flex-row justify-between cursor-pointer"
		on:click={() => (showParams = !showParams)}
	>
		<h1>{t('smtp.message.title')}</h1>
		<Hide bind:hidden={showParams} />
	</div>

	<Separator />

	{#if showParams}
		<div transition:slide>
			<SMTPMessage bind:message />

			<Separator />
		</div>
	{/if}
</div>

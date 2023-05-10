<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import { convert } from 'html-to-text';
	import AiOutlineMinus from 'svelte-icons-pack/ai/AiOutlineMinus';
	import Button, { ButtonTheme } from '$components/form/Button.svelte';
	import Input, { InputType } from '$components/form/Input.svelte';
	import Textarea from '$components/form/Textarea.svelte';
	import Badge, { BadgeColor, BadgeSize, BadgeTheme } from '$components/Badge.svelte';
	import { SMTPMessageHeader } from '$api/tauri_classes';
	import Checkbox from '$components/form/Checkbox.svelte';
	import t from '$i18n/translate';
	import { customMessage } from '$stores/smtp_message';

	let convertHTMLToTEXT: boolean = true;

	const addHeader = () => {
		$customMessage.message.headers = [
			...$customMessage.message.headers,
			new SMTPMessageHeader('', '')
		];
	};

	const removeHeader = (index: number) =>
		($customMessage.message.headers = $customMessage.message.headers.filter(
			(_, headerIndex) => index !== headerIndex
		));

	$: {
		if ($customMessage.message.body.html && convertHTMLToTEXT) {
			$customMessage.message.body.text = convert($customMessage.message.body.html);
		}
	}
</script>

<div class="flex flex-col space-y-5">
	<div class="flex place-content-between space-x-5">
		<Input
			name="messageToName"
			type={InputType.Text}
			placeholder={t('smtp.message.to.name')}
			className="flex flex-grow"
			bind:value={$customMessage.message.to.name}
		>
			<span slot="label">{t('smtp.message.to.name')}</span>
		</Input>
		<Input
			name="messageToEmail"
			type={InputType.Text}
			placeholder={t('smtp.message.to.email')}
			className="flex flex-grow"
			bind:value={$customMessage.message.to.email}
		>
			<span slot="label">{t('smtp.message.to.email')}</span>
		</Input>
	</div>
	<div class="flex place-content-between space-x-5">
		<Input
			name="fromName"
			type={InputType.Text}
			placeholder={t('smtp.message.from.name')}
			className="flex flex-grow"
			bind:value={$customMessage.message.from.name}
		>
			<span slot="label">{t('smtp.message.from.name')}</span>
		</Input>
		<Input
			name="fromEmail"
			type={InputType.Text}
			placeholder={t('smtp.message.from.email')}
			className="flex flex-grow"
			bind:value={$customMessage.message.from.email}
		>
			<span slot="label">{t('smtp.message.from.email')}</span>
		</Input>
	</div>
	<Badge
		text={t('smtp.message.header.add')}
		size={BadgeSize.SM}
		theme={BadgeTheme.Normal}
		color={BadgeColor.Primary}
		interactive={true}
		on:click={addHeader}
	/>
	{#if $customMessage.message.headers.length > 0}
		<div class="flex flex-grow flex-col mt-5 space-y-5">
			{#each $customMessage.message.headers as header, index}
				<div class="flex flex-grow place-content-start space-x-5">
					<Input
						name="headerName_{index}"
						type={InputType.Text}
						placeholder={t('smtp.message.header.name')}
						className="flex flex-grow"
						bind:value={header.name}
					/>
					<Input
						name="headerValue_{index}"
						type={InputType.Text}
						placeholder={t('smtp.message.header.value')}
						className="flex flex-grow"
						bind:value={header.value}
					/>
					<Button
						name="removeHeader_{index}"
						text=""
						theme={ButtonTheme.Error}
						on:click={() => removeHeader(index)}
					>
						<Icon slot="icon" src={AiOutlineMinus} className="fill-white" />
					</Button>
				</div>
			{/each}
		</div>
	{/if}
	<Input
		name="subject"
		type={InputType.Text}
		placeholder={t('smtp.message.subject')}
		className="flex flex-grow"
		bind:value={$customMessage.message.subject}
	>
		<span slot="label">{t('smtp.message.subject')}</span>
	</Input>
	<Textarea
		name="body"
		placeholder=""
		className="flex flex-grow"
		bind:value={$customMessage.message.body.html}
	>
		<div slot="label">
			<div class="flex flex-row justify-between">
				<div>{t('smtp.message.body.html')}</div>
				<Checkbox name="convertHTMLToTEXT" bind:checked={convertHTMLToTEXT}
					>{t('smtp.message.body.convert_html_to_text')}</Checkbox
				>
			</div>
		</div>
	</Textarea>
	{#if !convertHTMLToTEXT}
		<Textarea
			name="body"
			placeholder=""
			className="flex flex-grow"
			bind:value={$customMessage.message.body.text}
		>
			<div slot="label">{t('smtp.message.body.text')}</div>
		</Textarea>
	{/if}
</div>

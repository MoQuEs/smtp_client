<script lang="ts">
	import { sendMail } from '$api/tauri';

	import SMTPConfiguration from '$elements/SMTPConfiguration.svelte';
	import SMTPMessage from '$elements/SMTPMessage.svelte';
	import { smtp_configuration } from '$stores/smtp_configuration';
	import { smtp_message } from '$stores/smtp_message';

	import Button from '$components/form/Button.svelte';

	const sendMailHandle = () => {
		sendMail($smtp_configuration.configuration, $smtp_message.message);
	};
</script>

<div class="flex flex-col">
	<Button name="sendMail" text="Send Mail" className="mb-5" on:click={sendMailHandle} />

	<SMTPConfiguration bind:configuration={$smtp_configuration} />

	<SMTPMessage bind:message={$smtp_message} />
</div>

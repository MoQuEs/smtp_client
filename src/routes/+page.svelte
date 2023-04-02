<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import AiOutlineMinus from 'svelte-icons-pack/ai/AiOutlineMinus';
	import { sendMail, ServerRequest, MessageRequest } from '$api/tauri';

	import Button, { ButtonTheme } from '$components/form/Button.svelte';
	import Checkbox from '$components/form/Checkbox.svelte';
	import Input, { InputType } from '$components/form/Input.svelte';
	import Separator from '$components/Separator.svelte';
	import Textarea from '$components/form/Textarea.svelte';
	import Badge, { BadgeColor, BadgeSize, BadgeTheme } from '$components/Badge.svelte';

	type Header = { id: number; name: string; value: string };

	let serverAddress: string = '';
	let serverPort: number = 25;

	let serverUseAuth: boolean = false;
	let serverAuthUser: string = '';
	let ServerAuthPassword: string = '';

	let serverUseSSL: boolean = false;
	let serverSSLVerify: boolean = true;

	let messageToName: string = '';
	let messageToEmail: string = '';
	let messageFromName: string = '';
	let messageFromEmail: string = '';
	let messageReplayToName: string = '';
	let messageReplayToEmail: string = '';

	let headersCount = 0;
	let messageHeaders: Header[] = [];

	let messageSubject: string = '';

	let messageBody: string = '';

	const addHeader = () => {
		messageHeaders = [
			...messageHeaders,
			{
				id: headersCount++,
				name: '',
				value: ''
			}
		];
	};

	const removeHeader = (messageHeader: Header) =>
		(messageHeaders = messageHeaders.filter((i) => i.id !== messageHeader.id));

	const sendMailHandle = () => {
		let server = new ServerRequest(serverAddress, serverPort);
		server.setAuth(serverUseAuth, serverAuthUser, ServerAuthPassword);
		server.setSSL(serverUseSSL, serverSSLVerify);

		let message = new MessageRequest(
			messageToName,
			messageToEmail,
			messageFromName,
			messageFromEmail
		);
		message.setReplayTo(messageReplayToName, messageReplayToEmail);
		messageHeaders.forEach((header) => message.addHeaderRaw(header.name, header.value));
		message.setSubject(messageSubject);
		message.setBody(messageBody);

		sendMail(server, message);
	};
</script>

<div class="flex flex-col">
	<Button name="sendMail" text="Send Mail" className="mb-5" on:click={sendMailHandle} />

	<h1>SMTP Configuration</h1>
	<Separator />

	<div class="flex flex-col space-y-5">
		<div class="flex place-content-between space-x-5">
			<Input
				name="serverAddress"
				type={InputType.Text}
				placeholder="SMTP Server"
				className="flex flex-grow"
				bind:value={serverAddress}
			>
				<span slot="label">SMTP Server</span>
			</Input>
			<Input
				name="serverPort"
				type={InputType.Number}
				placeholder="SMTP Port"
				bind:value={serverPort}
			>
				<span slot="label">SMTP Port</span>
			</Input>
		</div>
		<div class="flex place-content-between space-x-5">
			<Checkbox name="serverUseAuth" bind:checked={serverUseAuth}>Use Auth</Checkbox>
		</div>
		<div class="flex place-content-between space-x-5 ml-6">
			<Input
				name="serverAuthUser"
				type={InputType.Text}
				placeholder="SMTP user"
				className="flex flex-grow"
				disabled={!serverUseAuth}
				readonly={!serverUseAuth}
				bind:value={serverAuthUser}
			>
				<span slot="label">SMTP user</span>
			</Input>
			<Input
				name="ServerAuthPassword"
				type={InputType.Password}
				placeholder="SMTP Password"
				className="flex flex-grow"
				disabled={!serverUseAuth}
				readonly={!serverUseAuth}
				bind:value={ServerAuthPassword}
			>
				<span slot="label">SMTP Password</span>
			</Input>
		</div>
		<div class="flex place-content-start space-x-5">
			<Checkbox name="serverUseSSL" bind:checked={serverUseSSL}>Use SSL</Checkbox>
		</div>
		<div class="flex place-content-between space-x-5 ml-6">
			<Checkbox
				name="serverSSLVerify"
				bind:checked={serverSSLVerify}
				disabled={!serverUseSSL}
				className="flex flex-grow">Verify SSL</Checkbox
			>
		</div>
	</div>

	<h1 class="mt-10">SMTP Message</h1>
	<Separator />

	<div class="flex flex-col space-y-5">
		<div class="flex place-content-between space-x-5">
			<Input
				name="messageToName"
				type={InputType.Text}
				placeholder="To name"
				className="flex flex-grow"
				bind:value={messageToName}
			>
				<span slot="label">To name</span>
			</Input>
			<Input
				name="messageToEmail"
				type={InputType.Text}
				placeholder="To email"
				className="flex flex-grow"
				bind:value={messageToEmail}
			>
				<span slot="label">To email</span>
			</Input>
		</div>
		<div class="flex place-content-between space-x-5">
			<Input
				name="fromName"
				type={InputType.Text}
				placeholder="From name"
				className="flex flex-grow"
				bind:value={messageFromName}
			>
				<span slot="label">From name</span>
			</Input>
			<Input
				name="fromEmail"
				type={InputType.Text}
				placeholder="From email"
				className="flex flex-grow"
				bind:value={messageFromEmail}
			>
				<span slot="label">From email</span>
			</Input>
		</div>
		<div class="flex place-content-between space-x-5">
			<Input
				name="messageReplayToName"
				type={InputType.Text}
				placeholder="Replay to name"
				className="flex flex-grow"
				bind:value={messageReplayToName}
			>
				<span slot="label">Replay to name</span>
			</Input>
			<Input
				name="messageReplayToEmail"
				type={InputType.Text}
				placeholder="Replay to email"
				className="flex flex-grow"
				bind:value={messageReplayToEmail}
			>
				<span slot="label">Replay to email</span>
			</Input>
		</div>
		<div class="flex flex-col">
			<Badge
				text="Add custom header"
				size={BadgeSize.SM}
				theme={BadgeTheme.Normal}
				color={BadgeColor.Info}
				interactive={true}
				on:click={addHeader}
			/>
			<div class="flex flex-grow flex-col {messageHeaders.length > 0 ? 'mt-5' : ''} space-y-5">
				{#each messageHeaders as messageHeader}
					<div class="flex flex-grow place-content-start space-x-5">
						<Input
							name="headerName_{messageHeader.id}"
							type={InputType.Text}
							placeholder="Header name"
							className="flex flex-grow"
							bind:value={messageHeader.name}
						/>
						<Input
							name="headerValue_{messageHeader.id}"
							type={InputType.Text}
							placeholder="Header value"
							className="flex flex-grow"
							bind:value={messageHeader.value}
						/>
						<Button
							name="removeHeader_{messageHeader.id}"
							text=""
							theme={ButtonTheme.Error}
							on:click={() => removeHeader(messageHeader)}
						>
							<Icon slot="icon" src={AiOutlineMinus} className="fill-white" />
						</Button>
					</div>
				{/each}
			</div>
		</div>
		<div class="flex place-content-between space-x-5">
			<Input
				name="subject"
				type={InputType.Text}
				placeholder="Subject"
				className="flex flex-grow"
				bind:value={messageSubject}
			>
				<span slot="label">Subject</span>
			</Input>
		</div>
		<div class="flex place-content-between space-x-5">
			<Textarea name="body" placeholder="" className="flex flex-grow" bind:value={messageBody}>
				<span slot="label">Body</span>
			</Textarea>
		</div>
	</div>
</div>

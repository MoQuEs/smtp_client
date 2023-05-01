<script lang="ts">
	import { sendMail } from '$api/tauri';
	import Icon from 'svelte-icons-pack';

	import { smtp_message } from '$stores/smtp_message';
	import t from '$i18n/translate';

	import RiBusinessMailSendLine from 'svelte-icons-pack/ri/RiBusinessMailSendLine';
	import RiBusinessMailSettingsLine from 'svelte-icons-pack/ri/RiBusinessMailSettingsLine';
	import RiBusinessMailAddLine from 'svelte-icons-pack/ri/RiBusinessMailAddLine';
	import FaSolidCogs from 'svelte-icons-pack/fa/FaSolidCogs';

	import Button from '$components/form/Button.svelte';
	import Tabs from '$components/tab/Tabs.svelte';
	import TabList from '$components/tab/TabList.svelte';
	import Tab from '$components/tab/Tab.svelte';
	import TabPanel from '$components/tab/TabPanel.svelte';
	import Separator from '$components/Separator.svelte';
	import SMTPConfigurationPanel from '$components/smtp/SMTPConfigurationPanel.svelte';
	import SMTPMessage from '$components/smtp/SMTPMessage.svelte';
	import Tooltip from '$components/tooltip/Tooltip.svelte';
	import LogoWithText from '$components/logo/LogoWithText.svelte';
</script>

<Tabs>
	<div class="flex flex-row">
		<LogoWithText />

		<TabList className="flex-grow justify-center">
			<Tooltip title={t('menu.send')}>
				<Tab className="flex flex-col items-center">
					<Icon src={RiBusinessMailSendLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={t('menu.configurations')}>
				<Tab>
					<Icon src={RiBusinessMailSettingsLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<!-- <Tooltip title={t('menu.messages')}>
				<Tab>
					<Icon src={RiBusinessMailAddLine} size="26" color="white" />
				</Tab>
			</Tooltip> -->

			<!-- <Tooltip title={t('menu.settings')}>
				<Tab>
					<Icon src={FaSolidCogs} size="26" color="white" />
				</Tab>
			</Tooltip> -->
		</TabList>
	</div>

	<Separator />

	<TabPanel>
		<div class="flex flex-col">
			<Button name="sendMail" text="Send Mail" className="mb-5" />

			<SMTPMessage bind:message={$smtp_message} />
		</div>
	</TabPanel>

	<TabPanel>
		<SMTPConfigurationPanel />
	</TabPanel>

	<!-- <TabPanel>
		<SMTPMessage bind:message={$smtp_message} />
	</TabPanel> -->

	<!-- <TabPanel>
		<h2>TODO: Settings</h2>
	</TabPanel> -->
</Tabs>

<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import t from '$lib/i18n/translate';

	import { RiBusinessMailSendLine } from 'svelte-icons-pack/ri';
	import { RiBusinessMailSettingsLine } from 'svelte-icons-pack/ri';
	import { RiBusinessMailAddLine } from 'svelte-icons-pack/ri';
	import { ImCogs } from "svelte-icons-pack/im";

	import Tabs from '$lib/components/tab/Tabs.svelte';
	import TabList from '$lib/components/tab/TabList.svelte';
	import Tab from '$lib/components/tab/Tab.svelte';
	import TabPanel from '$lib/components/tab/TabPanel.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';
	import SettingsPanel from '$lib/components/settings/SettingsPanel.svelte';
	import SMTPConfigurationPanel from '$lib/components/smtp/SMTPConfigurationPanel.svelte';
	import SMTPMessage from '$lib/components/smtp/SMTPMessage.svelte';
	import SMTPMessagePanel from '$lib/components/smtp/SMTPMessagePanel.svelte';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import LogoWithText from '$lib/components/logo/LogoWithText.svelte';
	import SMTPSendMail from '$lib/components/smtp/SMTPSendMail.svelte';
</script>

<Tabs>
	<div class="flex flex-row">
		<LogoWithText />

		<TabList className="flex-grow justify-end">
			<Tooltip title={$t('menu.send')}>
				<Tab>
					<Icon src={RiBusinessMailSendLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.configurations')}>
				<Tab>
					<Icon src={RiBusinessMailSettingsLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.messages')}>
				<Tab>
					<Icon src={RiBusinessMailAddLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.settings')}>
				<Tab>
					<Icon src={ImCogs} size="26" color="white" />
				</Tab>
			</Tooltip>
		</TabList>
	</div>

	<Separator />

	<TabPanel>
		<div class="flex flex-col space-y-5">
			<SMTPSendMail />

			<Separator size={SeparatorSize.XS} />

			<SMTPMessage />
		</div>
	</TabPanel>

	<TabPanel>
		<SMTPConfigurationPanel />
	</TabPanel>

	<TabPanel>
		<SMTPMessagePanel />
	</TabPanel>

	<TabPanel>
		<SettingsPanel />
	</TabPanel>
</Tabs>

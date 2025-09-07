<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import t from '$lib/i18n/translate';

	import {
		RiBusinessMailSettingsLine,
		RiBusinessMailAddLine,
		RiBusinessMailLockLine
	} from 'svelte-icons-pack/ri';
	import { BsSend } from 'svelte-icons-pack/bs';
	import { BiSolidCog } from 'svelte-icons-pack/bi';
	import { ImTable } from 'svelte-icons-pack/im';
	import { IoAttachOutline } from 'svelte-icons-pack/io';


	import Tabs from '$lib/components/tab/Tabs.svelte';
	import TabList from '$lib/components/tab/TabList.svelte';
	import Tab from '$lib/components/tab/Tab.svelte';
	import TabPanel from '$lib/components/tab/TabPanel.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';
	import SettingsPanel from '$lib/components/panels/settings/SettingsPanel.svelte';
	import ConfigurationPanel from '$lib/components/panels/configuration/ConfigurationPanel.svelte';
	import Message from '$lib/components/panels/message/Message.svelte';
	import MessagePanel from '$lib/components/panels/message/MessagePanel.svelte';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import LogoWithText from '$lib/components/logo/LogoWithText.svelte';
	import SendMailPanel from '$lib/components/panels/send_mail/SendMailPanel.svelte';
	import AttachmentPanel from '$lib/components/panels/attachment/AttachmentPanel.svelte';
	import PersonalizationPanel from '$lib/components/panels/personalization/PersonalizationPanel.svelte';
	import SMIMEPanel from '$lib/components/panels/smime/SMIMEPanel.svelte';
	import { fillIconClass, strokeIconClass } from '$lib/stores/theme';
</script>

<Tabs>
	<div class="flex flex-row justify-between flex-wrap mb-1">
		<LogoWithText />

		<TabList>
			<Tooltip title={$t('menu.send')}>
				<Tab>
					<Icon src={BsSend} size="26" className={$fillIconClass} />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.configurations')}>
				<Tab>
					<Icon src={RiBusinessMailSettingsLine} size="26" className={$fillIconClass} />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.messages')}>
				<Tab>
					<Icon src={RiBusinessMailAddLine} size="26" className={$fillIconClass} />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.smime')}>
				<Tab>
					<Icon src={RiBusinessMailLockLine} size="26" className={$fillIconClass} />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.attachments')}>
				<Tab>
					<Icon src={IoAttachOutline} size="26" className={$strokeIconClass} />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.personalizations')}>
				<Tab>
					<Icon src={ImTable} size="26" className={$fillIconClass} />
				</Tab>
			</Tooltip>

			<Tooltip title={$t('menu.settings')}>
				<Tab>
					<Icon src={BiSolidCog} size="26" className={$fillIconClass} />
				</Tab>
			</Tooltip>
		</TabList>
	</div>

	<Separator />

	<TabPanel>
		<div class="flex flex-col space-y-5">
			<SendMailPanel />

			<Separator size={SeparatorSize.XS} />

			<Message />
		</div>
	</TabPanel>

	<TabPanel>
		<ConfigurationPanel />
	</TabPanel>

	<TabPanel>
		<MessagePanel />
	</TabPanel>

	<TabPanel>
		<SMIMEPanel />
	</TabPanel>

	<TabPanel>
		<AttachmentPanel />
	</TabPanel>

	<TabPanel>
		<PersonalizationPanel />
	</TabPanel>

	<TabPanel>
		<SettingsPanel />
	</TabPanel>
</Tabs>
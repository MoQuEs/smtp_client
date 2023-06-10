<script lang="ts">
	import Icon from 'svelte-icons-pack';

	import t from '$i18n/translate';

	import RiBusinessMailSendLine from 'svelte-icons-pack/ri/RiBusinessMailSendLine';
	import RiBusinessMailSettingsLine from 'svelte-icons-pack/ri/RiBusinessMailSettingsLine';
	import RiBusinessMailAddLine from 'svelte-icons-pack/ri/RiBusinessMailAddLine';
	import FaSolidCogs from 'svelte-icons-pack/fa/FaSolidCogs';

	import Tabs from '$components/tab/Tabs.svelte';
	import TabList from '$components/tab/TabList.svelte';
	import Tab from '$components/tab/Tab.svelte';
	import TabPanel from '$components/tab/TabPanel.svelte';
	import Separator, { SeparatorSize } from '$components/Separator.svelte';
	import SettingsPanel from '$components/settings/SettingsPanel.svelte';
	import SMTPConfigurationPanel from '$components/smtp/SMTPConfigurationPanel.svelte';
	import SMTPMessage from '$components/smtp/SMTPMessage.svelte';
	import SMTPMessagePanel from '$components/smtp/SMTPMessagePanel.svelte';
	import Tooltip from '$components/tooltip/Tooltip.svelte';
	import LogoWithText from '$components/logo/LogoWithText.svelte';
	import SMTPSendMail from '$components/smtp/SMTPSendMail.svelte';
	import Button, { ButtonPaddingSize, ButtonTheme } from '$src/lib/components/form/Button.svelte';
</script>

<Tabs>
	<div class="flex flex-row">
		<LogoWithText />

		<TabList className="flex-grow justify-center">
			<Tooltip title={t('menu.send')}>
				<Tab>
					<Icon src={RiBusinessMailSendLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={t('menu.configurations')}>
				<Tab>
					<Icon src={RiBusinessMailSettingsLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={t('menu.messages')}>
				<Tab>
					<Icon src={RiBusinessMailAddLine} size="26" color="white" />
				</Tab>
			</Tooltip>

			<Tooltip title={t('menu.settings')}>
				<Tab>
					<Icon src={FaSolidCogs} size="26" color="white" />
				</Tab>
			</Tooltip>
		</TabList>
	</div>

	<Separator />

	<TabPanel>
		<div class="flex flex-col space-y-5">
			<!-- <div class="flex flex-row space-x-5">BUTTON COLOR TEST:</div>
			<div class="flex flex-row space-x-5">
				<Button text="Primary" theme={ButtonTheme.Primary} padding={ButtonPaddingSize.SM} />
				<Button text="Secondary" theme={ButtonTheme.Secondary} padding={ButtonPaddingSize.SM} />
			</div>
			<div class="flex flex-row space-x-5">
				<Button text="Success" theme={ButtonTheme.Success} padding={ButtonPaddingSize.SM} />
				<Button text="Error" theme={ButtonTheme.Error} padding={ButtonPaddingSize.SM} />
				<Button text="Warning" theme={ButtonTheme.Warning} padding={ButtonPaddingSize.SM} />
				<Button text="Info" theme={ButtonTheme.Info} padding={ButtonPaddingSize.SM} />
			</div>
			<div class="flex flex-row space-x-5">
				<Button text="LightGray" theme={ButtonTheme.LightGray} padding={ButtonPaddingSize.SM} />
				<Button text="Gray" theme={ButtonTheme.Gray} padding={ButtonPaddingSize.SM} />
				<Button text="DarkGray" theme={ButtonTheme.DarkGray} padding={ButtonPaddingSize.SM} />
				<Button text="DarkestGray" theme={ButtonTheme.DarkestGray} padding={ButtonPaddingSize.SM} />
			</div> -->

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

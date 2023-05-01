<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import t from '$i18n/translate';
	import { allConfigurations } from '$stores/smtp_configuration';
	import SMTPConfiguration from './SMTPConfiguration.svelte';
	import Separator, { SeparatorSize } from '$components/Separator.svelte';
	import Input from '$components/form/Input.svelte';
	import Button, { ButtonTheme } from '$components/form/Button.svelte';

	import Dropdown from '$components/dropdown/Dropdown.svelte';
	import DropdownHeader from '$components/dropdown/DropdownHeader.svelte';
	import DropdownItem from '$components/dropdown/DropdownItem.svelte';
	import DropdownSeparator from '$components/dropdown/DropdownSeparator.svelte';
	import Tooltip from '$components/tooltip/Tooltip.svelte';

	import BiRepost from 'svelte-icons-pack/bi/BiRepost';
	import AiOutlineMinus from 'svelte-icons-pack/ai/AiOutlineMinus';
	import AiOutlineDownload from 'svelte-icons-pack/ai/AiOutlineDownload';
	import AiOutlineUpload from 'svelte-icons-pack/ai/AiOutlineUpload';
</script>

<div class="flex flex-col">
	<Dropdown text="Saved configurations">
		<DropdownItem>
			<div class="flex flex-row space-x-5">
				<Input className="flex-grow" />
				<Tooltip title="Save&nbsp;configuration">
					<Button theme={ButtonTheme.Success} text="">
						<Icon src={AiOutlineUpload} size="22" color="white" slot="icon" />
					</Button>
				</Tooltip>
			</div>
		</DropdownItem>

		<DropdownSeparator />

		{#each $allConfigurations as confuguration, i}
			<DropdownItem>
				<div class="flex flex-row space-x-2">
					<span class="flex flex-grow text-base self-center">{confuguration.name}</span>
					<Tooltip title="Load">
						<Button theme={ButtonTheme.Success} text="">
							<Icon src={AiOutlineDownload} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
					<Tooltip title="Repleace">
						<Button theme={ButtonTheme.Success} text="">
							<Icon src={BiRepost} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
					<Tooltip title="Delete">
						<Button text="">
							<Icon src={AiOutlineMinus} size="22" color="white" slot="icon" />
						</Button>
					</Tooltip>
				</div>
			</DropdownItem>
		{/each}
	</Dropdown>

	<Separator size={SeparatorSize.XS} />

	<SMTPConfiguration />
</div>

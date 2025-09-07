<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import { AiOutlineMinus } from 'svelte-icons-pack/ai';

	import { RiSystemFilter2Line } from 'svelte-icons-pack/ri';
	import { RiSystemFilter2Fill } from 'svelte-icons-pack/ri';

	import t from '$lib/i18n/translate';
	import {
		allAttachments,
		filterAttachment,
		removeAttachment
	} from '$lib/stores/attachments';
	import Input from '$lib/components/form/Input.svelte';
	import { fillIconClass } from '$lib/stores/theme';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import Button, { ButtonTheme, ButtonPaddingSize } from '$lib/components/form/Button.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';


	let filter: boolean = $state(false);

	let rows = $derived($allAttachments
		.filter((value) => {
			return (
				!filter ||
				value.name.indexOf($filterAttachment) !== -1
			);
		})
		.sort((c1, c2) => c1.name.localeCompare(c2.name)));
</script>

<div class="flex flex-col">
	<Input
		className="flex-grow"
		placeholder={$t('attachment.filter')}
		bind:value={$filterAttachment}
		iconAfter={filter ? RiSystemFilter2Fill : RiSystemFilter2Line}
		iconAfterInteractive={true}
		iconAfterClass={$fillIconClass}
		on:click_after={() => (filter = !filter)}
		iconAfterTooltip={filter ? $t('turn_off_filter') : $t('turn_on_filter')}
	/>

	<Separator size={SeparatorSize.XS} />

	<div class="flex-grow overflow-auto scrollbar">
		<table>
			<thead>
			<tr>
				<th class="text-left p-2 border-b">{$t('attachment.name')}</th>
				<th class="text-left p-2 border-b">{$t('attachment.file_name')}</th>
				<th class="text-left p-2 border-b">{$t('attachment.mime')}</th>
				<th class="text-left p-2 border-b">{$t('attachment.size')}</th>
				<th class="p-2 border-b">{$t('attachment.options')}</th>
			</tr>
			</thead>
			<tbody>
			{#if rows.length === 0}
				<tr>
					<td class="p-2" colspan="5">{$t('attachment.no_attachments')}</td>
				</tr>
			{:else}
				{#each rows as row, index}
					<tr class="hover:bg-gray-100 dark:hover:bg-gray-700">
						<td class="p-2 border-b">{row.name}</td>
						<td class="p-2 border-b">{row.attachment.name}</td>
						<td class="p-2 border-b">{row.attachment.mime}</td>
						<td class="p-2 border-b">{(row.attachment.size / 1024).toFixed(2)} KB</td>

						<td class="p-2 border-b">
							<div class="flex flex-row space-x-2 justify-end">
								<Tooltip title={$t('remove')}>
									<Button
										text=""
										theme={ButtonTheme.Error}
										padding={ButtonPaddingSize.SM}
										on:click={() => removeAttachment(row)}
									>
										<Icon src={AiOutlineMinus} size="22" className="icon-fill-white" slot="icon" />
									</Button>
								</Tooltip>
							</div>
						</td>
					</tr>
				{/each}
			{/if}
			</tbody>
		</table>
	</div>
</div>

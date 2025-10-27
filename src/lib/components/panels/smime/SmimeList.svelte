<script lang="ts">
	import { Icon } from 'svelte-icons-pack';

	import { AiOutlineMinus } from 'svelte-icons-pack/ai';

	import t from '$lib/i18n/translate';
	import {
		allSmimes,
		filterSmime,
		removeSmime
	} from '$lib/stores/smime';
	import Input from '$lib/components/form/Input.svelte';
	import Tooltip from '$lib/components/tooltip/Tooltip.svelte';
	import Button, { ButtonTheme, ButtonPaddingSize } from '$lib/components/form/Button.svelte';
	import Separator, { SeparatorSize } from '$lib/components/Separator.svelte';

	let rows = $derived($allSmimes
		.filter((value) => {
			return (
				value.name.toLowerCase().indexOf($filterSmime.toLowerCase()) !== -1
				|| value.from.toLowerCase() == $filterSmime.toLowerCase()
			);
		})
		.sort((c1, c2) => c1.name.localeCompare(c2.name)));
</script>

<div class="flex flex-col">
	<Input
		className="flex-grow"
		placeholder={$t('smime.filter')}
		bind:value={$filterSmime}
	/>

	<Separator size={SeparatorSize.XS} />

	<div class="flex-grow overflow-auto scrollbar">
		<table>
			<thead>
			<tr>
				<th class="text-left p-2 border-b">{$t('smime.name')}</th>
				<th class="text-left p-2 border-b">{$t('smime.from')}</th>
				<th class="text-left p-2 border-b">{$t('smime.email')}</th>
				<th class="p-2 border-b">{$t('smime.options')}</th>
			</tr>
			</thead>
			<tbody>
			{#if rows.length === 0}
				<tr>
					<td class="p-2" colspan="5">{$t('smime.no_smimes')}</td>
				</tr>
			{:else}
				{#each rows as row, index}
					<tr class="hover:bg-gray-100 dark:hover:bg-gray-700">
						<td class="p-2 border-b">{row.name}</td>
						<td class="p-2 border-b">{row.from}</td>
						<td class="p-2 border-b">{row.smime.email}</td>

						<td class="p-2 border-b">
							<div class="flex flex-row space-x-2 justify-center">
								<Tooltip title={$t('remove')}>
									<Button
										text=""
										theme={ButtonTheme.Error}
										padding={ButtonPaddingSize.SM}
										on:click={() => removeSmime(row)}
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

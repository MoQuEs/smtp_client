<script lang="ts">
	import Button, { ButtonTheme } from '$lib/components/form/Button.svelte';
	import { theme } from '$lib/stores/theme';
	import { SettingsTheme } from '$lib/../generated/tauri';

	import { Icon } from 'svelte-icons-pack';
	import { AiFillCaretUp } from 'svelte-icons-pack/ai';
	import { AiFillCaretDown } from 'svelte-icons-pack/ai';

	let {
		text = '',
		className = ''
	}: {
		text: string,
		className: string
	} = $props();

	let filterDropdownIsOpen = $state(false);
	let iconColor = $derived($theme == SettingsTheme.Dark ? 'white' : 'black');
</script>

<div class="flex justify-center {className}">
	<div class="relative flex-grow">
		<Button
			name="dropdown"
			{text}
			theme={ButtonTheme.Gray}
			className="w-full"
			on:click={() => (filterDropdownIsOpen = !filterDropdownIsOpen)}
		>
			<Icon
				src={filterDropdownIsOpen ? AiFillCaretUp : AiFillCaretDown}
				slot="iconAfter"
				color={iconColor}
			/>
		</Button>

		<ul
			class="w-full max-h-96 overflow-auto absolute text-base z-10 float-left py-2 list-none text-left
			rounded shadow-lg mt-1 border-1 flex-grow m-0 bg-clip-padding scrollbar
			border-gray-200 bg-gray-100
			dark:border-gray-600 dark:bg-gray-800
			{filterDropdownIsOpen ? '' : 'hidden'}"
		>
			<slot />
		</ul>
	</div>
</div>

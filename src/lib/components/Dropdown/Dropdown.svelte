<script lang="ts">
	import Button, { ButtonTheme, ButtonType } from '$components/form/Button.svelte';

	import Icon from 'svelte-icons-pack/Icon.svelte';
	import AiFillCaretUp from 'svelte-icons-pack/ai/AiFillCaretUp';
	import AiFillCaretDown from 'svelte-icons-pack/ai/AiFillCaretDown';

	export let text: string;
	export let className: string = '';
	let filterDropdownIsOpen = false;
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
				color="white"
			/>
		</Button>

		<ul
			class="w-full max-h-96 overflow-auto absolute text-base z-10 float-left py-2 list-none text-left rounded shadow-lg mt-1 border-1 border-gray-600 flex-grow m-0 bg-clip-padding bg-gray-800 scrollbar
			{filterDropdownIsOpen ? '' : 'hidden'}"
		>
			<slot />
		</ul>
	</div>
</div>

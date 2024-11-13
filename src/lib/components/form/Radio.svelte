<script lang="ts">
	import { RandomId } from '../../utils/random';

	export let name: string = RandomId();

	export let checked: boolean = false;
	export let value: string = '';
	export let disabled: boolean = false;
	export let className: string = '';

	const id = RandomId();

	const handleChange = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		checked = e.currentTarget.checked;
	};
</script>

<div class="relative z-0 flex items-center cursor-pointer {className}">
	<input
		{id}
		{name}
		type="radio"
		{value}
		{disabled}
		class="w-4 h-4 rounded focus:ring-2
		ring-offset-gray-200 focus:ring-offset-gray-200 bg-gray-300 border-gray-400
		dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600
		{disabled ? 'cursor-not-allowed' : ''}"
		on:change={handleChange}
		{checked}
	/>
	<label
		for={id}
		class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-200
	{disabled ? 'cursor-not-allowed' : ''}"
	>
		<slot />
	</label>
</div>

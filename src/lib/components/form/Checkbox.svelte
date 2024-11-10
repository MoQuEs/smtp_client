<script lang="ts">
	import { RandomId } from '../../utils/random';

	export let name = RandomId();

	export let checked = false;
	export let value = '';
	export let disabled = false;
	export let className = '';

	const id = RandomId();

	const handleChange = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		checked = e.currentTarget.checked;
	};
</script>

<div class="relative z-0 flex items-center cursor-pointer {className}">
	<input
		{id}
		{name}
		type="checkbox"
		{value}
		{disabled}
		class="w-4 h-4 rounded ring-offset-gray-800 focus:ring-offset-gray-800 focus:ring-2 bg-gray-700 border-gray-600
		{disabled ? 'cursor-not-allowed' : ''}"
		on:change={handleChange}
		{checked}
	/>
	<label
		for={id}
		class="ml-2 text-sm font-medium text-gray-200
	{disabled ? 'cursor-not-allowed' : ''}"
	>
		<slot />
	</label>
</div>

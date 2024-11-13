<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { RandomId } from '../../utils/random';

	export let name = RandomId();

	export let rows = 4;
	export let value = '';
	export let placeholder = '';
	export let disabled = false;
	export let readonly = false;
	export let className = '';

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleInput = (e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) => {
		value = e.currentTarget.value;
		dispatch('input');
	};
</script>

<div class="flex flex-col {className}">
	{#if $$slots.label}
		<label for={id} class="block mb-2 text-sm font-medium text-black dark:text-white">
			<slot name="label" />
		</label>
	{/if}
	<textarea
		{id}
		{name}
		{rows}
		class="block p-2.5 w-full text-sm rounded border
		bg-gray-300 border-gray-400 placeholder-gray-600 text-black focus:ring-blue-500 focus:border-blue-500
		dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white
		{disabled ? 'cursor-not-allowed' : ''}"
		{placeholder}
		{disabled}
		{readonly}
		on:input={handleInput}
		on:change={handleInput}
		autocomplete="off">{value}</textarea
	>
</div>

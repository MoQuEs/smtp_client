<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { RandomId } from '$src/lib/components/Random';

	export let name: string;

	export let rows: number = 4;
	export let value: string = '';
	export let placeholder: string = '';
	export let disabled: boolean = false;
	export let readonly: boolean = false;
	export let className: string = '';

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleInput = (e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) => {
		value = e.currentTarget.value;
		dispatch('input');
	};
</script>

<div class="flex flex-col {className}">
	{#if $$slots.label}
		<label for={id} class="block mb-2 text-sm font-medium text-white"><slot name="label" /></label>
	{/if}
	<textarea
		{id}
		{name}
		{rows}
		class="block p-2.5 w-full text-sm rounded-lg border bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500
		{disabled ? 'cursor-not-allowed' : ''}"
		{placeholder}
		{disabled}
		{readonly}
		on:input={handleInput}
		on:change={handleInput}
		autocomplete="off">{value}</textarea
	>
</div>

<script context="module" lang="ts">
	export type SelectDispatch = {
		text: string;
		value: string | number;
	};
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { RandomId } from '$utils/random';
	import type { NEvent } from '$utils/types';

	export let name: string = RandomId();
	export let value: string | number = '';
	export let disabled: boolean = false;
	export let className: string = '';
	export let options: SelectDispatch[];

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleSelect = (e: NEvent<HTMLSelectElement>) => {
		value = e.currentTarget.value;
		dispatch('select', options.filter((option) => option.value === value)[0]);
	};
</script>

<div class="flex flex-col {className}">
	{#if $$slots.label}
		<label for={id} class="flex flex-grow mb-2 text-sm font-medium text-white">
			<slot name="label" />
		</label>
	{/if}
	<select
		{id}
		{name}
		bind:value
		class="bborder text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
		{disabled}
		on:select={handleSelect}
		on:change={handleSelect}
	>
		{#each options as option, i}
			<option value={option.value} selected={option.value === value}>
				{option.text}
			</option>
		{/each}
	</select>
</div>

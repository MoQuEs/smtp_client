<script context="module" lang="ts">
	import { RandomId } from '$utils/random';

	export class SelectDispatch<T> {
		text: string;
		value: T;

		constructor(text: string, value: T) {
			this.text = text;
			this.value = value;
		}
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { SNEvent } from '$utils/types';

	export let name: string = RandomId();
	export let disabled: boolean = false;
	export let className: string = '';
	export let options: SelectDispatch<any>[] = [];
	export let selected: SelectDispatch<any> | undefined =
		options.length > 0 ? options[0] : undefined;

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleSelect = (e: SNEvent<HTMLSelectElement>) => {
		selected = options.filter((_, index) => index === parseInt(e.currentTarget.value))[0];
		dispatch('select', selected);
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
		class="border text-sm rounded block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
		{disabled}
		on:select={handleSelect}
		on:change={handleSelect}
	>
		{#each options as option, i}
			<option value={i}>
				{option.text}
			</option>
		{/each}
	</select>
</div>

<script context="module" lang="ts">
	import { RandomId } from '../../utils/random';

	export class SelectDispatch<T> {
		text: string;
		value: T;
		selected: boolean;

		constructor(text: string, value: T, selected: boolean = false) {
			this.text = text;
			this.value = value;
			this.selected = selected;
		}
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { SNEvent } from '../../utils/types';

	export let name: string = RandomId();
	export let disabled: boolean = false;
	export let className: string = '';
	export let options: SelectDispatch<any>[] = [];
	export let selected: SelectDispatch<any> | undefined = undefined;
	if (options.length > 0) {
		if (options.filter((option) => option.selected).length > 0) {
			selected = options.filter((option) => option.selected)[0];
		} else {
			selected = options[0];
		}
	}

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleSelect = (e: SNEvent<HTMLSelectElement>) => {
		for (let i = 0; i < options.length; i++) {
			options[i].selected = false;
			if (i === parseInt(e.currentTarget.value)) {
				options[i].selected = true;
				selected = options[i];
			}
		}

		dispatch('select', selected);
	};
</script>

<div class="flex flex-col {className}">
	{#if $$slots.label}
		<label for={id} class="flex flex-grow mb-2 text-sm font-medium text-black dark:text-white">
			<slot name="label" />
		</label>
	{/if}
	<select
		{id}
		{name}
		class="border text-sm rounded block w-full p-2.5
		bg-gray-300 border-gray-400 placeholder-gray-600 text-black focus:ring-blue-500 focus:border-blue-500
		dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white"
		{disabled}
		on:select={handleSelect}
		on:change={handleSelect}
	>
		{#each options as option, i}
			<option value={i} selected={option.selected}>
				{option.text}
			</option>
		{/each}
	</select>
</div>

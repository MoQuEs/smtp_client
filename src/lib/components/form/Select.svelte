<script context="module" lang="ts">
	export enum InputType {
		Text = 'text',
		Hidden = 'hidden',
		Password = 'password',
		Email = 'email',
		Number = 'number',
		Tel = 'tel',
		Url = 'url'
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { RandomId } from '$components/Random';
	import type { NEvent } from '$types/Event';

	export let name: string;

	export let type: InputType = InputType.Text;
	export let value: string | number = '';
	export let disabled: boolean = false;
	export let className: string = '';

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleSelect = (e: NEvent<HTMLSelectElement>) => {
		value = type === InputType.Number ? Number(e.currentTarget.value) : e.currentTarget.value;
		dispatch('select');
	};

	const handleDummy = (e: Event) => {};
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
		class="bborder text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
		{disabled}
		on:select={handleSelect}
		on:change={handleSelect}
	>
		<option selected>Choose a country</option>
		<option value="US">United States</option>
		<option value="CA">Canada</option>
		<option value="FR">France</option>
		<option value="DE">Germany</option>
	</select>
</div>

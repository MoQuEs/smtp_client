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
	import { Icon } from 'svelte-icons-pack';
	import { AiOutlineEyeInvisible } from 'svelte-icons-pack/ai';
	import { AiOutlineEye } from 'svelte-icons-pack/ai';
	import t from '../../i18n/translate';
	import { RandomId } from '../../utils/random';
	import Tooltip from '../../components/tooltip/Tooltip.svelte';

	export let name: string = RandomId();

	export let type: InputType = InputType.Text;
	export let value: string | number | Number = '';
	export let placeholder = '';
	export let autocomplete: 'on' | 'off' = 'off';
	export let disabled = false;
	export let readonly = false;
	export let className = '';

	export let iconBefore: any = undefined;
	export let iconBeforeColor = 'white';
	export let iconBeforeTooltip = '';
	export let iconBeforeInteractive = false;

	export let iconAfter: any = undefined;
	export let iconAfterColor = 'white';
	export let iconAfterTooltip = '';
	export let iconAfterInteractive = false;

	let passwordShowed = false;
	let isPasswordinput = () =>
		type === InputType.Password || (type === InputType.Text && passwordShowed == true);

	const dispatch = createEventDispatcher();
	const id = RandomId();

	const handleInput = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		value = type === InputType.Number ? Number(e.currentTarget.value) : e.currentTarget.value;
		dispatch('input', value);
	};

	const handleClickBefore = () => {
		dispatch('click_before');
	};

	const handleClickAfter = () => {
		if (isPasswordinput()) {
			passwordShowed = !passwordShowed;
		}

		dispatch('click_after');
	};

	const handleDummy = () => {
	};
</script>

<div class="flex flex-col {className}">
	{#if $$slots.label}
		<label for="input-group-1" class="flex flex-grow mb-2 text-sm font-medium text-black dark:text-white">
			<slot name="label" />
		</label>
	{/if}
	<div class="flex flex-grow relative">
		{#if iconBefore}
			<div
				class="absolute inset-y-0 left-0 flex items-center pl-3
				pointer-events-none {iconBeforeInteractive ? 'cursor-pointer' : ''}"
				on:click={handleClickBefore}
				on:keydown={handleDummy}
				on:keyup={handleDummy}
				on:keypress={handleDummy}
				role="button" tabindex="0"
			>
				{#if iconBeforeTooltip !== ''}
					<Tooltip title={iconBeforeTooltip}>
						<Icon src={iconBefore} size="22" color={iconBeforeColor} />
					</Tooltip>
				{:else}
					<Icon src={iconBefore} size="22" color={iconBeforeColor} />
				{/if}
			</div>
		{/if}
		<input
			type={isPasswordinput() ? (passwordShowed ? InputType.Text : InputType.Password) : type}
			{name}
			{id}
			{value}
			on:input={handleInput}
			on:change={handleInput}
			{placeholder}
			class="border text-sm rounded block w-full
			{iconBefore ? 'pl-10' : ''}
			{iconAfter ? 'pr-10' : ''}
			{disabled ? 'cursor-not-allowed' : ''}
			p-2.5
			bg-gray-100 border-gray-200 placeholder-gray-700 text-black focus:ring-blue-500 focus:border-blue-500
			dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white"
			{disabled}
			{readonly}
			{autocomplete}
		/>
		{#if iconAfter || isPasswordinput()}
			<div
				class="absolute inset-y-0 right-0 flex items-center pr-3
				 {iconAfterInteractive || isPasswordinput() ? 'cursor-pointer' : ''}"
				on:click={handleClickAfter}
				on:keydown={handleDummy}
				on:keyup={handleDummy}
				on:keypress={handleDummy}
				role="button" tabindex="0"
			>
				{#if isPasswordinput()}
					<Tooltip
						title={passwordShowed
							? $t('components.form.input.hide_password')
							: $t('components.form.input.show_password')}
					>
						<Icon
							src={passwordShowed ? AiOutlineEye : AiOutlineEyeInvisible}
							size="22"
							color={iconAfterColor}
						/>
					</Tooltip>
				{:else if iconAfterTooltip !== ''}
					<Tooltip title={iconAfterTooltip}>
						<Icon src={iconAfter} size="22" color={iconAfterColor} />
					</Tooltip>
				{:else}
					<Icon src={iconAfter} size="22" color={iconAfterColor} />
				{/if}
			</div>
		{/if}
	</div>
</div>

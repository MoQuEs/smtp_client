<script context="module" lang="ts">
	export enum ButtonType {
		Submit = 'submit',
		Reset = 'reset',
		Button = 'button'
	}

	export enum ButtonSize {
		XS,
		SM,
		MD,
		LG,
		XL
	}

	export enum ButtonMode {
		Normal,
		Disabled,
		Loding
	}

	export enum ButtonTheme {
		Info,
		Success,
		Warning,
		Error,
		Primary,
		Secondary,
		LightGray,
		Gray,
		DarkGray,
		DarkestGray
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Icon from 'svelte-icons-pack';
	import CgSpinnerTwoAlt from 'svelte-icons-pack/cg/CgSpinnerTwoAlt';

	export let name: string;
	export let text: string;

	export let type: ButtonType = ButtonType.Button;
	export let theme: ButtonTheme = ButtonTheme.Primary;
	export let size: ButtonSize = ButtonSize.MD;
	export let mode: ButtonMode = ButtonMode.Normal;
	export let className: string = '';

	let textSizeClass = 'text-base';
	let iconSizeClass = 'text-base';
	switch (size) {
		case ButtonSize.XS:
			textSizeClass = 'text-xs';
			iconSizeClass = 'text-xs';
			break;

		case ButtonSize.SM:
			textSizeClass = 'text-sm';
			iconSizeClass = 'text-sm';
			break;

		case ButtonSize.MD:
			textSizeClass = 'text-base';
			iconSizeClass = 'text-base';
			break;

		case ButtonSize.LG:
			textSizeClass = 'text-lg';
			iconSizeClass = 'text-lg';
			break;

		case ButtonSize.XL:
			textSizeClass = 'text-xl';
			iconSizeClass = 'text-xl';
			break;
	}

	let cursor = 'cursor-pointer';
	switch (mode) {
		case ButtonMode.Normal:
			cursor = 'cursor-pointer';
			break;
		case ButtonMode.Loding:
			cursor = 'cursor-wait';
			break;
		case ButtonMode.Disabled:
			cursor = 'cursor-not-allowed';
			break;
	}

	let buttonClass = 'bg-primary-500';
	switch (theme) {
		case ButtonTheme.Primary:
			buttonClass = 'bg-primary-500';
			break;
		case ButtonTheme.Secondary:
			buttonClass = 'bg-secondary-500';
			break;
		case ButtonTheme.Info:
			buttonClass = 'bg-info-500';
			break;
		case ButtonTheme.Success:
			buttonClass = 'bg-success-500';
			break;
		case ButtonTheme.Warning:
			buttonClass = 'bg-warning-500';
			break;
		case ButtonTheme.Error:
			buttonClass = 'bg-error-500';
			break;
		case ButtonTheme.LightGray:
			buttonClass = 'bg-gray-600';
			break;
		case ButtonTheme.Gray:
			buttonClass = 'bg-gray-700';
			break;
		case ButtonTheme.DarkGray:
			buttonClass = 'bg-gray-800';
			break;
		case ButtonTheme.DarkestGray:
			buttonClass = 'bg-gray-900';
			break;
	}

	const dispatch = createEventDispatcher();

	let click = () => {
		mode === ButtonMode.Loding || mode === ButtonMode.Disabled ? false : dispatch('click');
	};
</script>

<button
	{name}
	{type}
	class="relative {textSizeClass} leading-none
    whitespace-nowrap font-bold btn-work active:brightness-75
    hover:brightness-125 text-white {buttonClass} rounded {cursor}
    {className}"
	on:click={click}
>
	{#if mode === ButtonMode.Loding || mode === ButtonMode.Disabled}
		<div
			class="absolute w-full h-full bg-gray-900 opacity-70 flex place-content-center
		{mode === ButtonMode.Disabled ? 'cursor-not-allowed' : ''}
		{mode === ButtonMode.Loding ? 'cursor-wait' : ''}"
		>
			{#if mode === ButtonMode.Loding}
				<Icon
					src={CgSpinnerTwoAlt}
					className="{iconSizeClass} fill-white animate-spin self-center"
				/>
			{/if}
		</div>
	{/if}
	<div class="flex flex-row flex-grow place-content-center align-middle mx-5 my-3">
		{#if $$slots.icon}
			<div class="{text != '' ? 'mr-2' : ''} {iconSizeClass}">
				<slot name="icon" />
			</div>
		{/if}
		{#if text != ''}
			<div>
				{text}
			</div>
		{/if}
		{#if $$slots.iconAfter}
			<div class="{text != '' ? 'ml-2' : ''} {iconSizeClass}">
				<slot name="iconAfter" />
			</div>
		{/if}
	</div>
</button>

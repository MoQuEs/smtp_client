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

	export enum ButtonPaddingSize {
		XS,
		SM,
		MD,
		LG,
		XL
	}

	export enum ButtonMode {
		Normal,
		Disabled,
		Loading
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
	import { RandomId } from '$utils/random';

	export let text: string;
	export let name: string = RandomId();

	export let type: ButtonType = ButtonType.Button;
	export let theme: ButtonTheme = ButtonTheme.Primary;
	export let size: ButtonSize = ButtonSize.MD;
	export let padding: ButtonPaddingSize = ButtonPaddingSize.LG;
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

	let paddingSize = 'mx-5 my-3';
	switch (padding) {
		case ButtonPaddingSize.XS:
			paddingSize = 'mx-2 my-1';
			break;

		case ButtonPaddingSize.SM:
			paddingSize = 'mx-3 my-2';
			break;

		case ButtonPaddingSize.MD:
			paddingSize = 'mx-4 my-2';
			break;

		case ButtonPaddingSize.LG:
			paddingSize = 'mx-5 my-3';
			break;

		case ButtonPaddingSize.XL:
			paddingSize = 'mx-6 my-4';
			break;
	}

	let cursor = 'cursor-pointer';
	switch (mode) {
		case ButtonMode.Normal:
			cursor = 'cursor-pointer';
			break;
		case ButtonMode.Loading:
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
		mode === ButtonMode.Loading || mode === ButtonMode.Disabled ? false : dispatch('click');
	};
</script>

<button
	{name}
	{type}
	class="relative flex flex-row flex-grow {textSizeClass}
    whitespace-nowrap font-bold active:brightness-75
    hover:brightness-125 text-white items-center rounded {buttonClass}  {cursor}
    {className}"
	on:click={click}
>
	{#if mode === ButtonMode.Loading || mode === ButtonMode.Disabled}
		<div
			class="absolute w-full h-full bg-gray-900 opacity-70 flex flex-grow place-content-center
		{mode === ButtonMode.Disabled ? 'cursor-not-allowed' : ''}
		{mode === ButtonMode.Loading ? 'cursor-wait' : ''}"
		>
			{#if mode === ButtonMode.Loading}
				<Icon
					src={CgSpinnerTwoAlt}
					className="{iconSizeClass} fill-white animate-spin self-center"
				/>
			{/if}
		</div>
	{/if}
	<div class="flex flex-grow place-content-center align-middle items-center {paddingSize}">
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

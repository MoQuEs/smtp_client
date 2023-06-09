<script context="module" lang="ts">
	export enum ToastType {
		Primary,
		Secondary,
		Info,
		Success,
		Warning,
		Error,
		CloseAll
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';

	import Icon from 'svelte-icons-pack';
	import AiFillInfoCircle from 'svelte-icons-pack/ai/AiFillInfoCircle';
	import AiFillWarning from 'svelte-icons-pack/ai/AiFillWarning';
	import AiFillCheckCircle from 'svelte-icons-pack/ai/AiFillCheckCircle';
	import AiFillCloseCircle from 'svelte-icons-pack/ai/AiFillCloseCircle';
	import AiOutlineClose from 'svelte-icons-pack/ai/AiOutlineClose';
	import AiOutlineCheck from 'svelte-icons-pack/ai/AiOutlineCheck';

	export let type: ToastType;
	export let title: string;
	export let subTitle: string = '';
	export let text: string = '';

	let icon = AiFillInfoCircle;
	let bg: string = 'bg-info-800';
	let border: string = 'border-info-500';
	switch (type) {
		case ToastType.Info:
			bg = 'bg-info-800';
			border = 'border-info-500';
			icon = AiFillInfoCircle;
			break;
		case ToastType.Success:
			bg = 'bg-success-800';
			border = 'border-success-500';
			icon = AiFillCheckCircle;
			break;
		case ToastType.Warning:
			bg = 'bg-warning-800';
			border = 'border-warning-500';
			icon = AiFillWarning;
			break;
		case ToastType.Error:
			bg = 'bg-error-800';
			border = 'border-error-500';
			icon = AiFillCloseCircle;
			break;
		case ToastType.Primary:
			bg = 'bg-primary-800';
			border = 'border-primary-500';
			icon = AiFillInfoCircle;
			break;
		case ToastType.Secondary:
			bg = 'bg-secondary-800';
			border = 'border-secondary-500';
			icon = AiFillInfoCircle;
			break;
		case ToastType.CloseAll:
			bg = 'bg-gray-800';
			border = 'border-gray-500';
			icon = AiFillInfoCircle;
			break;
	}

	const dispatch = createEventDispatcher();

	const closeToast = () => dispatch('dismiss');
	const dummy = () => {};
</script>

<div
	class="{bg} opacity-90 shadow-lg mx-auto w-60 max-w-full text-sm bg-clip-padding rounded-lg"
	transition:fade
>
	<div
		class="{bg} flex justify-between items-center py-2 px-3 bg-clip-padding border-b {border} rounded-t-lg
		{text == '' ? 'rounded-b-lg' : ''}"
	>
		<p class="font-bold text-white flex items-center">
			<Icon src={icon} size="18" className="mr-1 fill-white" />
			{title}
		</p>
		<div class="flex items-center">
			{#if subTitle != '' && subTitle != undefined}
				<p class="text-white opacity-90 text-xs mr-1">{subTitle}</p>
			{/if}
			<div
				class="flex items-center"
				on:click={closeToast}
				on:keydown={dummy}
				on:keyup={dummy}
				on:keypress={dummy}
			>
				<Icon
					src={type != ToastType.CloseAll ? AiOutlineClose : AiOutlineCheck}
					size="20"
					className="cursor-pointer fill-white"
				/>
			</div>
		</div>
	</div>

	{#if text != ''}
		<div class="p-3 {bg} rounded-b-lg break-words text-white">
			{text}
		</div>
	{/if}
</div>

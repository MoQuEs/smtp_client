<script context="module" lang="ts">
	export enum BadgeSize {
		XS,
		SM,
		MD,
		LG,
		XL
	}

	export enum BadgeColor {
		Info = 'info',
		Success = 'success',
		Warning = 'warning',
		Error = 'error',
		Primary = 'primary',
		Secondary = 'secondary'
	}

	export enum BadgeTheme {
		FullColor = 'full_color',
		Normal = 'normal'
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let text: string;

	export let size: BadgeSize = BadgeSize.MD;
	export let color: BadgeColor = BadgeColor.Primary;
	export let theme: BadgeTheme = BadgeTheme.FullColor;

	export let interactive: boolean = false;

	export let className: string = '';

	let sizeClass = 'text-base';
	switch (size) {
		case BadgeSize.XS:
			sizeClass = 'text-xs';
			break;

		case BadgeSize.SM:
			sizeClass = 'text-sm';
			break;

		case BadgeSize.MD:
			sizeClass = 'text-base';
			break;

		case BadgeSize.LG:
			sizeClass = 'text-lg';
			break;

		case BadgeSize.XL:
			sizeClass = 'text-xl';
			break;
	}

	let colorClass = 'bg-primary-500';
	let textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-primary-900';
	let hoverColorClass = 'bg-primary-400';
	let hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-primary-800';
	let activeColorClass = 'bg-primary-600';
	let activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-primary-900';
	switch (color) {
		case BadgeColor.Primary:
			colorClass = 'bg-primary-500';
			textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-primary-900';
			hoverColorClass = 'bg-primary-400';
			hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-primary-800';
			activeColorClass = 'bg-primary-600';
			activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-primary-900';
			break;

		case BadgeColor.Secondary:
			colorClass = 'bg-secondary-500';
			textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-secondary-900';
			hoverColorClass = 'bg-secondary-400';
			hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-secondary-800';
			activeColorClass = 'bg-secondary-600';
			activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-secondary-900';
			break;

		case BadgeColor.Info:
			colorClass = 'bg-info-500';
			textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-info-900';
			hoverColorClass = 'bg-info-400';
			hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-info-800';
			activeColorClass = 'bg-info-600';
			activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-info-900';
			break;

		case BadgeColor.Success:
			colorClass = 'bg-success-500';
			textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-success-900';
			hoverColorClass = 'bg-success-400';
			hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-success-800';
			activeColorClass = 'bg-success-600';
			activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-success-900';
			break;

		case BadgeColor.Warning:
			colorClass = 'bg-warning-500';
			textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-warning-900';
			hoverColorClass = 'bg-warning-400';
			hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-warning-800';
			activeColorClass = 'bg-warning-600';
			activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-warning-900';
			break;

		case BadgeColor.Error:
			colorClass = 'bg-error-500';
			textColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-error-900';
			hoverColorClass = 'bg-error-400';
			hoverTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-error-800';
			activeColorClass = 'bg-error-600';
			activeTextColorClass = theme == BadgeTheme.Normal ? 'text-white' : 'text-error-900';
			break;
	}

	let spanColorClass = colorClass;
	let spanTextColorClass = textColorClass;

	const dispatch = createEventDispatcher();

	let click = () => {
		if (interactive) {
			dispatch('click');
		}
	};

	let hover = false;
	let active = false;
	let changeColor = (e: Event) => {
		if (interactive) {
			if (e.type == 'mouseover') {
				hover = true;
			} else if (e.type == 'mouseleave') {
				hover = false;
				active = false;
			} else if (e.type == 'mousedown') {
				active = true;
			} else if (e.type == 'mouseup') {
				if (active) {
					click();
				}
				active = false;
			}

			if (active) {
				spanColorClass = activeColorClass;
				spanTextColorClass = activeTextColorClass;
			} else if (hover) {
				spanColorClass = hoverColorClass;
				spanTextColorClass = hoverTextColorClass;
			} else {
				spanColorClass = colorClass;
				spanTextColorClass = textColorClass;
			}
		}
	};
</script>

<span
	class="{sizeClass} {interactive
		? 'cursor-pointer'
		: ''} inline-flex py-1 px-2.5 leading-none whitespace-nowrap align-baseline font-bold {spanColorClass} {spanTextColorClass} rounded {className}"
	on:mouseover={changeColor}
	on:focus={changeColor}
	on:mouseleave={changeColor}
	on:mousedown={changeColor}
	on:mouseup={changeColor}
>
	{text}
</span>

<script context="module" lang="ts">
	export enum SeparatorSize {
		XS,
		SM,
		MD,
		LG,
		XL
	}

	export enum SeparatorMarginSize {
		XS,
		SM,
		MD,
		LG,
		XL
	}

	export enum SeparatorColor {
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
	export let size: SeparatorSize = SeparatorSize.SM;
	export let sizeMargin: SeparatorMarginSize = SeparatorMarginSize.SM;
	export let color: SeparatorColor = SeparatorColor.LightGray;

	export let className: string = '';

	let sizeClass = 'border-2';
	switch (size) {
		case SeparatorSize.XS:
			sizeClass = 'border-1';
			break;

		case SeparatorSize.SM:
			sizeClass = 'border-2';
			break;

		case SeparatorSize.MD:
			sizeClass = 'border-3';
			break;

		case SeparatorSize.LG:
			sizeClass = 'border-4';
			break;

		case SeparatorSize.XL:
			sizeClass = 'border-5';
			break;
	}

	let sizeMarginClass = 'my-3';
	switch (sizeMargin) {
		case SeparatorMarginSize.XS:
			sizeMarginClass = 'my-2';
			break;

		case SeparatorMarginSize.SM:
			sizeMarginClass = 'my-4';
			break;

		case SeparatorMarginSize.MD:
			sizeMarginClass = 'my-6';
			break;

		case SeparatorMarginSize.LG:
			sizeMarginClass = 'my-8';
			break;

		case SeparatorMarginSize.XL:
			sizeMarginClass = 'my-10';
			break;
	}

	let colorClass = 'border-primary-500';
	switch (color) {
		case SeparatorColor.Primary:
			colorClass = 'border-primary-500';
			break;
		case SeparatorColor.Secondary:
			colorClass = 'border-secondary-500';
			break;
		case SeparatorColor.Info:
			colorClass = 'border-info-500';
			break;
		case SeparatorColor.Success:
			colorClass = 'border-success-500';
			break;
		case SeparatorColor.Warning:
			colorClass = 'border-warning-500';
			break;
		case SeparatorColor.Error:
			colorClass = 'border-error-500';
			break;
		case SeparatorColor.LightGray:
			colorClass = 'border-gray-600';
			break;
		case SeparatorColor.Gray:
			colorClass = 'border-gray-700';
			break;
		case SeparatorColor.DarkGray:
			colorClass = 'border-gray-800';
			break;
		case SeparatorColor.DarkestGray:
			colorClass = 'border-gray-900';
			break;
	}
</script>

<hr
	class="h-0 border border-solid opacity-50 {sizeClass} {sizeMarginClass} {colorClass} {className}"
/>

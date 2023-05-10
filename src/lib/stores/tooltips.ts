import { writable, type Writable } from 'svelte/store';

const lastStabilized: TooltipPosition = { x: 0, y: 0 };

export const tooltipTestPosition: Writable<TooltipPosition> = writable({ x: 0, y: 0 });

export const tooltip: Writable<Tooltip> = writable({
	title: '',
	show: false,
	position: { x: 0, y: 0 }
});

export declare type Tooltip = {
	title: string;
	show: boolean;
	position: TooltipPosition;
};

export declare type TooltipPosition = {
	x: number;
	y: number;
};

export const showTooltip = (show: boolean = true) => {
	tooltip.update((tooltip) => {
		tooltip.title = tooltip.title;
		tooltip.show = show;
		return tooltip;
	});
};

export const titleTooltip = (title: string) => {
	tooltip.update((tooltip) => {
		tooltip.title = title;
		return tooltip;
	});
};

const positionNormalize = (position: TooltipPosition, down: boolean = true): TooltipPosition => {
	return {
		x: position.x + 10,
		y: position.y + (down ? 15 : -30)
	};
};

export const positionTooltip = (position: TooltipPosition) => {
	let down = true;

	tooltipTestPosition.set(positionNormalize(position, down));
	let elem = document.getElementById('tooltip-container-test');
	if (elem === null) {
		return;
	}

	let bounding = elem.getBoundingClientRect();
	let windowWidth = window.innerWidth || document.documentElement.clientWidth;
	let windowHeight = window.innerHeight || document.documentElement.clientHeight;

	if (bounding.right > windowWidth) {
		let toRemove = Math.floor(windowWidth - bounding.right);
		let newX = position.x + (toRemove - 1);
		if (newX > lastStabilized.x - 5 && newX < lastStabilized.x + 5) {
			position.x = lastStabilized.x;
		} else {
			lastStabilized.x = newX;
			position.x = newX;
		}
	}

	if (bounding.bottom > windowHeight) {
		down = false;
	}

	tooltip.update((tooltip) => {
		tooltip.position = positionNormalize(position, down);
		return tooltip;
	});
};

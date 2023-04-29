<script lang="ts">
	import { titleTooltip, showTooltip, positionTooltip } from '$stores/tooltips';
	import type { CEvent } from '$types/Event';

	export let title: string = '';

	function mouseOver(event: CEvent<MouseEvent, HTMLDivElement>) {
		titleTooltip(title);
		mouseMove(event);
		showTooltip(true);
	}

	function mouseMove(event: CEvent<MouseEvent, HTMLDivElement>) {
		positionTooltip({ x: event.pageX, y: event.pageY });
	}

	function mouseLeave() {
		showTooltip(false);
	}

	function focus(_: CEvent<FocusEvent, HTMLDivElement>) {}
</script>

<div on:mouseover={mouseOver} on:focus={focus} on:mouseleave={mouseLeave} on:mousemove={mouseMove}>
	<slot />
</div>

<script lang="ts">
	import { titleTooltip, showTooltip, positionTooltip } from '$stores/tooltips';
	import type { SMEvent, SFEvent } from '$utils/types';

	export let title = '';
	export let className = '';

	function mouseOver(event: SMEvent<HTMLDivElement>) {
		titleTooltip(title);
		mouseMove(event);
		showTooltip(true);
	}

	function mouseMove(event: SMEvent<HTMLDivElement>) {
		positionTooltip({ x: event.pageX, y: event.pageY });
	}

	function mouseLeave() {
		showTooltip(false);
	}

	function focus() {}
</script>

<div
	class="inline-flex {className}"
	on:mouseover={mouseOver}
	on:focus={focus}
	on:mouseleave={mouseLeave}
	on:mousemove={mouseMove}
>
	<slot />
</div>

<script lang="ts">
	export let className: string = '';
</script>

<div class="flex flex-row flex-wrap space-x-2 {className}">
	<slot />
</div>

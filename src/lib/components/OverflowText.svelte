<script lang="ts">
	export let className: string = '';
</script>

<span class="min-w-0 text-ellipsis overflow-hidden whitespace-nowrap {className}">
	<slot />
</span>

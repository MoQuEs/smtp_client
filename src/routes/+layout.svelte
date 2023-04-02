<script lang="ts">
	import '../app.scss';

	import Toasts from '$elements/Toasts.svelte';
</script>

<Toasts />

<main
	class="flex flex-col flex-grow-1 max-h-screen min-h-screen w-full overflow-y-auto scrollbar bg-gray-900 text-gray-200 px-3 py-3"
>
	<slot />
</main>

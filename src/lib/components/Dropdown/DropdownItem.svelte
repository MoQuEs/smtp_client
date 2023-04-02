<script lang="ts">
	export let alowHover: boolean = true;
</script>

<li
	class="text-sm text-gray-300 py-2 px-4 font-normal block w-full whitespace-nowrap bg-transparent {alowHover
		? 'hover:bg-gray-700 hover:text-white focus:text-white focus:bg-gray-700'
		: ''}"
>
	<slot />
</li>

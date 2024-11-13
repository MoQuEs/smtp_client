<script lang="ts">
	export let alowHover: boolean = true;
	export let className: string = '';
</script>

<li
	class="text-sm text-black dark:text-white py-2 px-4 font-normal w-full whitespace-nowrap bg-transparent
	{className}
	{alowHover ? 'hover:bg-gray-200 focus:bg-gray-300 ' +
	 ' dark:hover:bg-gray-700 dark:focus:bg-gray-700' : ''}"
>
	<slot />
</li>

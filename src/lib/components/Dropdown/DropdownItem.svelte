<script lang="ts">
	export let alowHover: boolean = true;
	export let className: string = '';
</script>

<li
	class="text-sm text-gray-700 dark:text-gray-300 py-2 px-4 font-normal w-full whitespace-nowrap bg-transparent
	{className}
	{alowHover ? 'hover:bg-gray-300 hover:text-black focus:text-black focus:bg-gray-300 ' +
	 ' dark:hover:bg-gray-700 dark:hover:text-white dark:focus:text-white dark:focus:bg-gray-700' : ''}"
>
	<slot />
</li>

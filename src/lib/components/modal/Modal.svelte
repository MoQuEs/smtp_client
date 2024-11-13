<script lang="ts">
	import { Icon } from 'svelte-icons-pack';
	import { AiOutlineClose } from 'svelte-icons-pack/ai';

	let {
		children,
		header,
		showModal = $bindable(false)
	}: {
		children: () => any,
		header: () => any,
		showModal?: boolean,
	} = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div
	class="fixed top-0 left-0 w-screen h-screen p-4 overflow-hidden backdrop-blur-sm backdrop-brightness-50 m-0 flex items-center justify-center z-50"
	class:hidden={!showModal}
	style="margin: 0 !important;"
>
	<div class="rounded-lg flex flex-shrink flex-col shadow bg-gray-300 dark:bg-gray-700">
		<div class="flex items-center justify-between p-4 border-b rounded-t border-gray-400 dark:border-gray-600">
			<h3 class="text-xl font-medium text-white">
				{@render header()}
			</h3>
			<div
				class="flex items-center"
				onclick={() => showModal = false}
			>
				<Icon
					src={AiOutlineClose}
					size="20"
					className="cursor-pointer fill-white"
				/>
			</div>
		</div>

		<div class="p-4 space-y-4">
			{@render children()}
		</div>
	</div>
</div>

<script context="module" lang="ts">
	export class ModalRef {
		[x: string]: any;

		private modal: any = null;
		private closeCallback: null | Function = null;

		constructor(modal: any) {
			this.modal = modal;
		}

		close(artifacts?: any) {
			this.modal.$destroy();
			if (this.closeCallback !== null) {
				this.closeCallback(artifacts);
			}
		}

		onClose(callback: Function) {
			this.closeCallback = callback;
		}
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Icon } from 'svelte-icons-pack';
	import { AiOutlineCheck, AiOutlineClose } from 'svelte-icons-pack/ai';

	const dispatch = createEventDispatcher();

	export let component: any = null;
	export let componentProps: any = {};

	function handleClick(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (target.id === 'dark-overlay') {
			close();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			close();
		}
	}

	const close = () => dispatch('close');
	const dummy = () => {
	};
</script>

<svelte:window on:keydown={handleKeydown}></svelte:window>

<div
	class="fixed top-0 left-0 right-0 z-50 hidden w-full p-4 overflow-x-hidden overflow-y-auto md:inset-0 h-[calc(100%-1rem)] max-h-full"
>
	<div class="relative w-full max-w-7xl max-h-full">
		<div class="relative rounded-lg shadow bg-gray-700">
			<div class="flex items-center justify-between p-5 border-b rounded-t border-gray-600">
				<h3 class="text-xl font-medium text-white">Extra Large modal</h3>
				<div
					class="flex items-center"
					on:click={close}
					on:keydown={dummy}
					on:keyup={dummy}
					on:keypress={dummy}
				>
					<Icon
						src={AiOutlineClose}
						size="20"
						className="cursor-pointer fill-white"
					/>
				</div>
			</div>
			<div class="p-6 space-y-6">
				<svelte:component this={component} {...componentProps} />
			</div>
		</div>
	</div>
</div>

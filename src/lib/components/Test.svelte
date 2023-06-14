<script lang="ts">
	import t from '$i18n/translate';
	import { callTauri } from '$api/tauri';
	import Button from '$components/form/Button.svelte';

	const call = () => {
		callTauri('test', { name: 'test' }).then((res) => {
			console.log(res);
		});
	};
</script>

<div class="flex flex-row space-x-5">
	<Button text="test" on:click={call} />
</div>

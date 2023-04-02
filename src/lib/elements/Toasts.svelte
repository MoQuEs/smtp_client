<script lang="ts">
	import Toast, { ToastType } from '$components/Toast.svelte';
	import t from '$i18n/t';
	import { dismissToast, dismissAllToasts, toasts } from '$stores/toasts';
</script>

{#if $toasts.length > 0}
	<section class="fixed flex right-0 flex-col max-h-screen gap-2 flex-wrap-reverse z-10 p-3">
		<Toast type={ToastType.CloseAll} title={t('toats.close_all')} on:dismiss={dismissAllToasts} />

		{#each $toasts as toast (toast.id)}
			<Toast
				type={toast.type}
				title={toast.title}
				subTitle={toast.subTitle}
				text={toast.text}
				on:dismiss={() => dismissToast(toast.id)}
			/>
		{/each}
	</section>
{/if}

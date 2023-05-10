<script lang="ts">
	import Toast, { ToastType } from '$components/toast/Toast.svelte';
	import t from '$i18n/translate';
	import { dismissToast, dismissAllToasts, toasts } from '$stores/toasts';
</script>

{#if $toasts.length > 0}
	<section class="fixed flex right-0 flex-col max-h-screen gap-2 flex-wrap-reverse z-999 p-3">
		<Toast type={ToastType.CloseAll} title={t('toast.close_all')} on:dismiss={dismissAllToasts} />

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

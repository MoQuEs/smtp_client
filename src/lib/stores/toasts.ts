import { writable, type Writable } from 'svelte/store';
import type { ToastType } from '$lib/components/toast/Toast.svelte';
import { RandomId } from '$lib/utils/random';

export const toasts: Writable<ToastComponentShowed[]> = writable([]);

export declare type ToastComponent = {
	type: ToastType;
	title: string;
	subTitle?: string;
	text?: string;
	autoCloseAfterS?: number;
};

type ToastComponentShowed = ToastComponent & {
	id: number | string;
	// @ts-ignore
	timeOut: NodeJS.Timeout;
};

export const addToast = (toast: ToastComponent) => {
	const addedToast: ToastComponentShowed = {
		...{
			id: RandomId(),
			subTitle: '',
			text: '',
			timeOut: setTimeout(() => dismissToast(addedToast.id), toast.autoCloseAfterS ?? 10 * 1000)
		},
		...toast
	};

	toasts.update((all) => [addedToast, ...all]);
};

export const dismissToast = (id: number | string) => {
	toasts.update((all) =>
		all.filter((t) => {
			if (t.id === id) {
				clearTimeout(t.timeOut);
				return false;
			}

			return true;
		})
	);
};

export const dismissAllToasts = () => {
	toasts.update((all) =>
		all.filter((t) => {
			clearTimeout(t.timeOut);
			return false;
		})
	);
};

import Modal, { ModalRef, type ModalOptions } from '$lib/components/modal/Modal.svelte';

let modalRef: ModalRef = null;

export function openModal(component: any, componentProps?: any) {
	if (modalRef) {
		console.error('A modal is already open.');
		return;
	}

	const props: any = { component };
	if (componentProps) {
		props.componentProps = componentProps;
	}

	const modal = new Modal({
		target: document.body,
		props
	});

	modal.$on('close', (event: CustomEvent<any>) => {
		closeActiveModal(event.detail);
	});

	modalRef = new ModalRef(modal);

	return modalRef;
}

export function closeActiveModal(artifacts?: any) {
	if (!modalRef) {
		return;
	}

	modalRef.close(artifacts);
	modalRef = null;
}

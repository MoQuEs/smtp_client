import { invoke, type InvokeArgs } from '@tauri-apps/api/tauri';
import v8n from 'v8n';
import { validate, validateOrReject } from 'class-validator';
import t from '$i18n/t';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/Toast.svelte';
import type {
	TauriResponse,
	ServerRequest as SR,
	MessageRequest as MR,
	MessageHeaderRequest as MHR
} from '$generated/tauri';

export interface ChechParams {
	//chechParams(): boolean;
}

class ServerRequest implements SR {
	address: string;
	port: number;
	use_auth: boolean = false;
	auth_user: string = '';
	auth_password: string = '';
	use_ssl: boolean = false;
	ssl_verify: boolean = true;

	constructor(
		address: string,
		port: number,
		use_auth: boolean = false,
		auth_user: string = '',
		auth_password: string = '',
		use_ssl: boolean = false,
		ssl_verify: boolean = true
	) {
		this.address = address;
		this.port = port;
		this.use_auth = use_auth;
		this.auth_user = auth_user;
		this.auth_password = auth_password;
		this.use_ssl = use_ssl;
		this.ssl_verify = ssl_verify;
	}

	setAuth(use_auth: boolean = false, auth_user: string = '', auth_password: string = '') {
		this.use_auth = use_auth;
		this.auth_user = auth_user;
		this.auth_password = auth_password;
	}

	setSSL(use_ssl: boolean = false, ssl_verify: boolean = true) {
		this.use_ssl = use_ssl;
		this.ssl_verify = ssl_verify;
	}
}

class MessageRequest implements MR {
	to_name: string;
	to_email: string;
	from_name: string;
	from_email: string;
	replay_to_name: string = '';
	replay_to_email: string = '';
	headers: MessageHeaderRequest[] = [];
	subject: string = '';
	body: string = '';

	constructor(
		to_name: string,
		to_email: string,
		from_name: string,
		from_email: string,
		replay_to_name: string = '',
		replay_to_email: string = '',
		headers: MessageHeaderRequest[] = [],
		subject: string = '',
		body: string = ''
	) {
		this.to_name = to_name;
		this.to_email = to_email;
		this.from_name = from_name;
		this.from_email = from_email;
		this.replay_to_name = replay_to_name;
		this.replay_to_email = replay_to_email;
		this.headers = headers;
		this.subject = subject;
		this.body = body;
	}

	setReplayTo(replay_to_name: string = '', replay_to_email: string = '') {
		this.replay_to_name = replay_to_name;
		this.replay_to_email = replay_to_email;
	}

	setHeaders(headers: MessageHeaderRequest[] = []) {
		this.headers = headers;
	}

	addHeader(header: MessageHeaderRequest) {
		this.headers.push(header);
	}

	addHeaderRaw(header_name: string, header_value: string) {
		this.addHeader(new MessageHeaderRequest(header_name, header_value));
	}

	setSubject(subject: string = '') {
		this.subject = subject;
	}

	setBody(body: string = '') {
		this.body = body;
	}
}

class MessageHeaderRequest implements MHR {
	header: string;
	value: string;

	constructor(header: string, value: string) {
		this.header = header;
		this.value = value;
	}
}

function isTauriResponse<T>(res: unknown): res is TauriResponse<T> {
	return (res as TauriResponse<T>) !== undefined;
}

export const sendMail = (server: ServerRequest, message: MessageRequest) => {
	invoke('send_mail', { server: server, message: message })
		.then((res) => {
			if (isTauriResponse(res)) {
				addToast({
					type: res.success ? ToastType.Success : ToastType.Error,
					title: t('ERROR'),
					text: res.message
				});
				return;
			}
			addToast({
				type: ToastType.Error,
				title: t('ERROR')
			});
		})
		.catch((e) =>
			addToast({
				type: ToastType.Error,
				title: t('ERROR'),
				text: e
			})
		);
};

export { ServerRequest, MessageRequest, MessageHeaderRequest };

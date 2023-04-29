import { invoke, type InvokeArgs } from '@tauri-apps/api/tauri';
import t from '$i18n/translate';
import { addToast } from '$stores/toasts';
import { ToastType } from '$components/toast/Toast.svelte';
import type * as tauri from '$generated/tauri';

export class NamedSMTPConfiguration implements tauri.NamedSMTPConfiguration {
	public name: string;
	public configuration: SMTPConfiguration;

	constructor(name: string, configuration: SMTPConfiguration = new SMTPConfiguration()) {
		this.name = name;
		this.configuration = configuration;
	}
}

export class SMTPConfiguration implements tauri.SMTPConfiguration {
	public address: SMTPConfigurationAddress;
	public auth: SMTPConfigurationAuth;
	public require_ssl: boolean;
	public verify_certificates: boolean;

	constructor(
		address: SMTPConfigurationAddress = new SMTPConfigurationAddress(),
		auth: SMTPConfigurationAuth = new SMTPConfigurationAuth(),
		require_ssl: boolean = false,
		verify_certificates: boolean = false
	) {
		this.address = address;
		this.auth = auth;
		this.require_ssl = require_ssl;
		this.verify_certificates = verify_certificates;
	}
}

export class SMTPConfigurationAddress implements tauri.SMTPConfigurationAddress {
	public address: string;
	public port: number;

	constructor(address: string = '', port: number = 25) {
		this.address = address;
		this.port = port;
	}
}

export class SMTPConfigurationAuth implements tauri.SMTPConfigurationAuth {
	public use_auth: boolean;
	public user: string;
	public password: string;

	constructor(use_auth: boolean = false, user: string = '', password: string = '') {
		this.use_auth = use_auth;
		this.user = user;
		this.password = password;
	}
}

export class NamedSMTPMessage implements tauri.NamedSMTPMessage {
	public name: string;
	public message: SMTPMessage;

	constructor(name: string, message: SMTPMessage = new SMTPMessage()) {
		this.name = name;
		this.message = message;
	}
}

export class SMTPMessage implements tauri.SMTPMessage {
	public to: SMTPMessageAddress;
	public from: SMTPMessageAddress;
	public reply_to: SMTPMessageAddress;
	public cc: SMTPMessageAddress;
	public bcc: SMTPMessageAddress;
	public headers: SMTPMessageHeader[];
	public subject: string;
	public body: SMTPMessageBody;

	constructor(
		to: SMTPMessageAddress = new SMTPMessageAddress(),
		from: SMTPMessageAddress = new SMTPMessageAddress(),
		reply_to: SMTPMessageAddress = new SMTPMessageAddress(),
		cc: SMTPMessageAddress = new SMTPMessageAddress(),
		bcc: SMTPMessageAddress = new SMTPMessageAddress(),
		headers: SMTPMessageHeader[] = [],
		subject: string = '',
		body: SMTPMessageBody = new SMTPMessageBody()
	) {
		this.to = to;
		this.from = from;
		this.reply_to = reply_to;
		this.cc = cc;
		this.bcc = bcc;
		this.headers = headers;
		this.subject = subject;
		this.body = body;
	}
}

export class SMTPMessageAddress implements tauri.SMTPMessageAddress {
	public name: string | undefined;
	public email: string;

	constructor(name: string | undefined = '', email: string = '') {
		this.name = name;
		this.email = email;
	}
}

export class SMTPMessageHeader implements tauri.SMTPMessageHeader {
	public name: string;
	public value: string;

	constructor(name: string, value: string) {
		this.name = name;
		this.value = value;
	}
}

export class SMTPMessageBody implements tauri.SMTPMessageBody {
	public html: string;
	public text: string;

	constructor(html: string = '', text: string = '') {
		this.html = html;
		this.text = text;
	}
}

export type Callback<T> = (response_data: tauri.TauriResponse<T>) => Promise<void>;

export const sendMail = async (
	server: SMTPConfiguration,
	message: SMTPMessage,
	callback: Callback<null> | undefined = undefined
) => {
	callTauri<null>(
		'send_mail_command',
		{ server: server, message: message },
		async (response_data) => {
			if (response_data.success) {
				addToast({
					type: ToastType.Success,
					title: t('api.send_mail.success')
				});
			} else {
				addToast({
					type: ToastType.Error,
					title: t('api.send_mail.error')
				});
			}
		}
	);
};

async function callTauri<T>(
	function_name: string,
	data: InvokeArgs,
	callback: Callback<T> | undefined = undefined
) {
	invoke<tauri.TauriResponse<T>>(function_name, data)
		.then(async (res) => {
			if (isTauriResponse(res)) {
				if (callback !== undefined) {
					callback(res);
				}
				return;
			}
			addToast({
				type: ToastType.Error,
				title: t('api.data.error'),
				text: typeof res
			});
		})
		.catch((e) =>
			addToast({
				type: ToastType.Error,
				title: t('api.error'),
				text: e
			})
		);
}

function isTauriResponse<T>(res: unknown): res is tauri.TauriResponse<T> {
	return (res as tauri.TauriResponse<T>) !== undefined;
}

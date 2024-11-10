import type * as tauri from '$lib/../generated/tauri';

export type TauriResponse<T> = tauri.TauriResponse<T>;

export type NamedSMTPConfigurations = NamedSMTPConfiguration[];

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
		require_ssl = false,
		verify_certificates = false
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

	constructor(address = '', port = 25) {
		this.address = address;
		this.port = port;
	}
}

export class SMTPConfigurationAuth implements tauri.SMTPConfigurationAuth {
	public use_auth: boolean;
	public user: string;
	public password: string;

	constructor(use_auth = false, user = '', password = '') {
		this.use_auth = use_auth;
		this.user = user;
		this.password = password;
	}
}

export type NamedSMTPMessages = NamedSMTPMessage[];

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
		subject = '',
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

	constructor(name: string | undefined = '', email = '') {
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

	constructor(html = '', text = '') {
		this.html = html;
		this.text = text;
	}
}

export class Secret<T> implements tauri.Secret<T> {
	public name: string;
	public value: T;

	constructor(name: string, value: T) {
		this.name = name;
		this.value = value;
	}
}

export type SettingsTheme = tauri.SettingsTheme;

export type SettingsLanguage = tauri.SettingsLanguage;

export class Settings implements tauri.Settings {
	public theme: SettingsTheme;
	public language: SettingsLanguage;

	constructor(theme: SettingsTheme, language: SettingsLanguage) {
		this.theme = theme;
		this.language = language;
	}
}

export class ImportExportSettings implements tauri.ImportExportSettings {
	public password: string;
	public smtp_configurations: boolean;
	public smtp_messages: boolean;
	public settings: boolean;

	constructor(
		password: string,
		smtp_configurations: boolean,
		smtp_messages: boolean,
		settings: boolean
	) {
		this.password = password;
		this.smtp_configurations = smtp_configurations;
		this.smtp_messages = smtp_messages;
		this.settings = settings;
	}
}

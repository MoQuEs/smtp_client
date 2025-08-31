import type * as tauri from '$lib/../generated/tauri';

export type TauriResponse<T> = tauri.TauriResponse<T>;

export type NamedConfigurations = NamedConfiguration[];

export class NamedConfiguration implements tauri.NamedConfiguration {
	public name: string;
	public configuration: Configuration;

	constructor(name: string, configuration: Configuration = new Configuration()) {
		this.name = name;
		this.configuration = configuration;
	}
}

export class Configuration implements tauri.Configuration {
	public address: ConfigurationAddress;
	public auth: ConfigurationAuth;
	public require_ssl: boolean;
	public verify_certificates: boolean;

	constructor(
		address: ConfigurationAddress = new ConfigurationAddress(),
		auth: ConfigurationAuth = new ConfigurationAuth(),
		require_ssl = false,
		verify_certificates = false
	) {
		this.address = address;
		this.auth = auth;
		this.require_ssl = require_ssl;
		this.verify_certificates = verify_certificates;
	}
}

export class ConfigurationAddress implements tauri.ConfigurationAddress {
	public address: string;
	public port: number;

	constructor(address = '', port = 25) {
		this.address = address;
		this.port = port;
	}
}

export class ConfigurationAuth implements tauri.ConfigurationAuth {
	public use_auth: boolean;
	public user: string;
	public password: string;

	constructor(use_auth = false, user = '', password = '') {
		this.use_auth = use_auth;
		this.user = user;
		this.password = password;
	}
}

export type NamedMessages = NamedMessage[];

export class NamedMessage implements tauri.NamedMessage {
	public name: string;
	public message: Message;

	constructor(name: string, message: Message = new Message()) {
		this.name = name;
		this.message = message;
	}
}

export class Message implements tauri.Message {
	public to: MessageAddress;
	public from: MessageAddress;
	public reply_to: MessageAddress;
	public cc: MessageAddress;
	public bcc: MessageAddress;
	public headers: MessageHeader[];
	public subject: string;
	public body: MessageBody;

	constructor(
		to: MessageAddress = new MessageAddress(),
		from: MessageAddress = new MessageAddress(),
		reply_to: MessageAddress = new MessageAddress(),
		cc: MessageAddress = new MessageAddress(),
		bcc: MessageAddress = new MessageAddress(),
		headers: MessageHeader[] = [],
		subject = '',
		body: MessageBody = new MessageBody()
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

export class MessageAddress implements tauri.MessageAddress {
	public name: string | undefined;
	public email: string;

	constructor(name: string | undefined = '', email = '') {
		this.name = name;
		this.email = email;
	}
}

export class MessageHeader implements tauri.MessageHeader {
	public name: string;
	public value: string;

	constructor(name: string, value: string) {
		this.name = name;
		this.value = value;
	}
}

export class MessageBody implements tauri.MessageBody {
	public html: string;
	public text: string;
	public convert_html_to_text: boolean;

	constructor(html = '', text = '', convert_html_to_text = true) {
		this.html = html;
		this.text = text;
		this.convert_html_to_text = convert_html_to_text;
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
	public configurations: boolean;
	public messages: boolean;
	public settings: boolean;

	constructor(
		password: string,
		configurations: boolean,
		messages: boolean,
		settings: boolean
	) {
		this.password = password;
		this.configurations = configurations;
		this.messages = messages;
		this.settings = settings;
	}
}

export type NamedAttachments = NamedAttachment[];

export class Attachment implements tauri.Attachment {
	public path: string;
	public mime: string;
	public binary: number[];

	constructor(path = '', mime = '', binary: number[] = []) {
		this.path = path;
		this.mime = mime;
		this.binary = binary;
	}
}

export class NamedAttachment implements tauri.NamedAttachment {
	public name: string;
	public attachment: Attachment;

	constructor(name: string, attachment: Attachment = new Attachment()) {
		this.name = name;
		this.attachment = attachment;
	}
}

export class ToSaveAttachment implements tauri.ToSaveAttachment {
	public name: string;

	constructor(name: string) {
		this.name = name;
	}
}

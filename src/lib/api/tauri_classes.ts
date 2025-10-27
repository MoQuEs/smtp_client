import * as tauri from '$lib/../generated/tauri';

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
	public name: string;
	public extension: string;
	public mime: string;
	public size: number;
	public binary: number[];

	constructor(
		path = '',
		name = '',
		extension = '',
		mime = '',
		binary: number[] = []
	) {
		this.path = path;
		this.name = name;
		this.extension = extension;
		this.mime = mime;
		this.size = binary.length;
		this.binary = binary;
	}
}

export class NamedAttachment implements tauri.NamedAttachment {
	public name: string;
	public from: AddAttachmentFrom;
	public attachment: Attachment;

	constructor(name: string, from: AddAttachmentFrom, attachment: Attachment) {
		this.name = name;
		this.from = from;
		this.attachment = attachment;
	}
}

export type AddAttachmentFrom = tauri.AddAttachmentFrom;
export const AddAttachmentFrom = tauri.AddAttachmentFrom;

export const addAttachmentFromFromString = (s: string): AddAttachmentFrom => {
	switch (s) {
		case tauri.AddAttachmentFrom.File:
			return tauri.AddAttachmentFrom.File;
		case tauri.AddAttachmentFrom.Url:
			return tauri.AddAttachmentFrom.Url;
		default:
			throw new Error('Unrecognized Attachment type');
	}
};

export class AddAttachment implements tauri.AddAttachment {
	public name: string;
	public from: AddAttachmentFrom;
	public url: string;

	constructor(name: string, from: AddAttachmentFrom, url: string = '') {
		this.name = name;
		this.from = from;
		this.url = url;
	}
}

export type NamedSmimes = NamedSmime[];

export class Smime implements tauri.Smime {
	public email: string;
	public key_name: string;
	public key_data: number[];
	public cert_name: string;
	public cert_data: number[];
	public has_ca_cert: boolean;
	public ca_cert_name?: string;
	public ca_cert_data?: number[];

	constructor(
		email = '',
		key_name = '',
		key_data: number[] = [],
		cert_name = '',
		cert_data: number[] = [],
		has_ca_cert = false,
		ca_cert_name?: string,
		ca_cert_data?: number[]
	) {
		this.email = email;
		this.key_name = key_name;
		this.key_data = key_data;
		this.cert_name = cert_name;
		this.cert_data = cert_data;
		this.has_ca_cert = has_ca_cert;
		this.ca_cert_name = ca_cert_name;
		this.ca_cert_data = ca_cert_data;
	}
}

export class NamedSmime implements tauri.NamedSmime {
	public name: string;
	public from: AddSmimeFrom;
	public smime: Smime;

	constructor(name: string, from: AddSmimeFrom, smime: Smime) {
		this.name = name;
		this.from = from;
		this.smime = smime;
	}
}

export type AddSmimeFrom = tauri.AddSmimeFrom;
export const AddSmimeFrom = tauri.AddSmimeFrom;

export const addSmimeFromFromString = (s: string): AddSmimeFrom => {
	switch (s) {
		case tauri.AddSmimeFrom.PKCS12:
			return tauri.AddSmimeFrom.PKCS12;
		case tauri.AddSmimeFrom.Separate:
			return tauri.AddSmimeFrom.Separate;
		default:
			throw new Error('Unrecognized Smime type');
	}
};

export class AddSmime implements tauri.AddSmime {
	public name: string;
	public from: AddSmimeFrom;
	public email: string;
	public password: string;

	constructor(
		name: string,
		from: AddSmimeFrom,
		email: string,
		password: string
	) {
		this.name = name;
		this.from = from;
		this.email = email;
		this.password = password;
	}
}

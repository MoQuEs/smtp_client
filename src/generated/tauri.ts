/*
 Generated by typeshare 1.5.0
*/

export interface TauriResponse<T> {
	success: boolean;
	message?: string;
	data?: T;
}

export interface SMTPConfigurationAddress {
	address: string;
	port: number;
}

export interface SMTPConfigurationAuth {
	use_auth: boolean;
	user: string;
	password: string;
}

export interface SMTPConfiguration {
	address: SMTPConfigurationAddress;
	auth: SMTPConfigurationAuth;
	require_ssl: boolean;
	verify_certificates: boolean;
}

export interface NamedSMTPConfiguration {
	name: string;
	configuration: SMTPConfiguration;
}

export interface SMTPMessageAddress {
	name?: string;
	email: string;
}

export interface SMTPMessageHeader {
	name: string;
	value: string;
}

export interface SMTPMessageBody {
	html: string;
	text: string;
}

export interface SMTPMessage {
	to: SMTPMessageAddress;
	from: SMTPMessageAddress;
	reply_to: SMTPMessageAddress;
	cc: SMTPMessageAddress;
	bcc: SMTPMessageAddress;
	headers: SMTPMessageHeader[];
	subject: string;
	body: SMTPMessageBody;
}

export interface NamedSMTPMessage {
	name: string;
	message: SMTPMessage;
}

export interface Secret<T> {
	name: string;
	value: T;
}

export enum SettingsTheme {
	Dark = "Dark",
}

export enum SettingsLanguage {
	English = "English",
}

export interface Settings {
	theme: SettingsTheme;
	language: SettingsLanguage;
}


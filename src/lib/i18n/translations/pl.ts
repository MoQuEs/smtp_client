export default {
	title: {
		full: 'SMTPclient',
		first_part: 'SMTP',
		second_part: 'client'
	},
	ERROR: 'ERROR',
	SUCCESS: 'SUKCES',
	custom: 'Własce',
	show: 'Pokaż',
	hide: 'Ukryj',
	save: 'Zapisz',
	load: 'Załaduj',
	repleace: 'Zamień',
	delete: 'Usuń',
	remove: 'Usuń',
	filter: 'Filtruj',
	turn_on_filter: 'Włącz filter',
	turn_off_filter: 'Wyłącz filter',
	name_exists_error: 'Nazwa już istnieje',
	name_cant_be_empty_error: 'Nazwa nie może być pusta',
	imports_exports: 'Import/Export',
	import: 'Import',
	export: 'Export',
	logo: {
		alt: 'Logo',
		text: 'Logo text'
	},
	components: {
		form: {
			input: {
				email: 'Email',
				show_password: 'Pokaż hasło',
				hide_password: 'Ukryj hasło'
			}
		}
	},
	toast: {
		loading: 'Ładowanie...',
		close_all: 'Zamknij wszystkie',
		close_in: 'Zamknij za {{seconds}}s'
	},
	menu: {
		send: 'Wyślij',
		configurations: 'Konfiguracje',
		messages: 'Wiadomości',
		settings: 'Ustawienia'
	},
	smtp: {
		send_mail: 'Wyślij wiadomość',
		configuration: {
			name: 'Konfiguracja',
			unsaved: 'Niezapisana konfiguracja',
			no_configurations_saved: 'Brak zapisanych konfiguracji',
			address: 'Adres',
			port: 'Port',
			use_auth: 'Użyj autoryzacji',
			user: 'Użytkownik',
			password: 'Hasło',
			require_ssl: 'Wymagaj SSL',
			verify_certificates: 'Weryfikuj certyfikaty',
			configuration_name: 'Nazwa konfiguracji',
			configuration_filter: 'Filtr konfiguracji',
			saved: 'Zapisano konfigurację',
			save_error: 'Nie można zapisać konfiguracji',
			repleace: 'Zamieniono konfigurację',
			repleace_error: 'Nie można zamienić konfiguracji',
			remove: 'Usunięto konfigurację',
			remove_error: 'Nie można usunąć konfiguracji',
			load: 'Załadowano konfigurację',
			load_error: 'Nie można załadować konfiguracji'
		},
		message: {
			name: 'Wiadomość',
			unsaved: 'Niezapisana wiadomość',
			no_messages_saved: 'Brak zapisanych wiadomości',
			to: {
				name: 'To nazwa',
				email: 'To email'
			},
			from: {
				name: 'From nazwa',
				email: 'From email'
			},
			reply_to: {
				name: 'Replay to nazwa',
				email: 'Replay to email'
			},
			header: {
				add: 'Dodaj nagłówek',
				name: 'Nazwa',
				value: 'Wartość'
			},
			subject: 'Temat',
			body: {
				html: 'HTML',
				convert_html_to_text: 'Konwertuj HTML na tekst',
				text: 'TEXT'
			},
			message_name: 'Nazwa wiadomości',
			message_filter: 'Filtr wiadomości',
			saved: 'Zapisano wiadomość',
			save_error: 'Nie można zapisać wiadomości',
			repleace: 'Zamieniono wiadomość',
			repleace_error: 'Nie można zamienić wiadomości',
			remove: 'Usunięto wiadomość',
			remove_error: 'Nie można usunąć wiadomości',
			load: 'Załadowano wiadomość',
			load_error: 'Nie można załadować wiadomości'
		}
	},
	settings: {
		save_success: 'Zapisano ustawienia',
		save_error: 'Nie można zapisać ustawień',
		load_error: 'Nie można załadować ustawień',
		locale_error: 'Nie można zmienić języka',
		language: 'Język',
		languages: {
			en: 'English',
			pl: 'Polski'
		},
		theme: 'Styl',
		themes: {
			dark: 'Dark'
		}
	},
	api: {
		error: 'Błąd podczas wywoływania api',
		data: {
			error: 'Błąd podczas pobierania danych z api'
		},
		send_mail: {
			success: 'Wysłano email',
			error: 'Błąd podczas wysyłania emaila'
		}
	}
};

export default {
	title: {
		full: 'SMTPclient',
		first_part: 'SMTP',
		second_part: 'client'
	},
	ERROR: 'ERROR',
	SUCCESS: 'SUCCESS',
	custom: 'Custom',
	show: 'Show',
	hide: 'Hide',
	save: 'Save',
	load: 'Load',
	repleace: 'Repleace',
	delete: 'Delete',
	remove: 'Remove',
	filter: 'Filter',
	turn_on_filter: 'Turn on filter',
	turn_off_filter: 'Turn off filter',
	name_exists_error: 'Name alredy exists',
	name_cant_be_empty_error: "Name can't be empty",
	imports_exports: 'Imports/Exports',
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
				show_password: 'Show password',
				hide_password: 'Hide password'
			}
		}
	},
	toast: {
		loading: 'Loading...',
		close_all: 'Close all',
		close_in: 'Close in {{seconds}}s'
	},
	menu: {
		send: 'Send',
		configurations: 'Configurations',
		messages: 'Messages',
		settings: 'Settings'
	},
	smtp: {
		send_mail: 'Send Mail',
		configuration: {
			name: 'Configuration',
			unsaved: 'Unsaved configuration',
			no_configurations_saved: 'No configurations saved',
			address: 'Address',
			port: 'Port',
			use_auth: 'Use auth',
			user: 'User',
			password: 'Password',
			require_ssl: 'Require SSL',
			verify_certificates: 'Verify Certificates',
			configuration_name: 'Configuration name',
			configuration_filter: 'Configuration filter',
			saved: 'Saved configuration',
			save_error: "Can't save configuration",
			repleace: 'Repleaced configuration',
			repleace_error: "Can't repleace configuration",
			remove: 'Removed configuration',
			remove_error: "Can't remove configuration",
			load: 'Loaded configuration',
			load_error: "Can't load configuration"
		},
		message: {
			name: 'Message',
			unsaved: 'Unsaved mesage',
			select_label: '',
			select_label_with_name: '',
			no_messages_saved: 'No messages saved',
			to: {
				name: 'To name',
				email: 'To email'
			},
			from: {
				name: 'From name',
				email: 'From email'
			},
			reply_to: {
				name: 'Replay to name',
				email: 'Replay to email'
			},
			header: {
				add: 'Add custom header',
				name: 'Name',
				value: 'Value'
			},
			subject: 'Subject',
			body: {
				html: 'HTML',
				convert_html_to_text: 'Get TEXT version from HTML',
				text: 'TEXT'
			},
			message_name: 'Message name',
			message_filter: 'Message filter',
			saved: 'Saved message',
			save_error: "Can't save message",
			repleace: 'Repleaced message',
			repleace_error: "Can't repleace message",
			remove: 'Removed message',
			remove_error: "Can't remove message",
			load: 'Loaded message',
			load_error: "Can't load message"
		}
	},
	settings: {
		save_success: 'Settings saved',
		save_error: "Can't save settings",
		load_error: "Can't load settings",
		locale_error: "Can't load language",
		language: 'Language',
		languages: {
			en: 'English',
			pl: 'Polski'
		},
		theme: 'Theme',
		themes: {
			dark: 'Dark'
		}
	},
	api: {
		error: 'Error while calling api',
		data: {
			error: 'Error while calling api'
		},
		send_mail: {
			success: 'Email was sent',
			error: 'Cant sent email'
		}
	}
};

import fs from 'fs';
import { exec } from './exec.js';

let date = new Date();
let day = date.getDate();
let month = date.getMonth() + 1;
let year = date.getFullYear();

day = day.padStart(2, '0');
month = month.padStart(2, '0');

const currentDate = `${day}-${month}-${year}`;

if (typeof process.argv[2] !== 'string' || process.argv[2] === '') {
	console.error('Version number needs to be provided');
	process.exit(1);
}

const version = process.argv[2];

new Promise(async () => {
	// change version in tauri.conf.json
	let tauriConfFile = './src-tauri/tauri.conf.json';
	fs.readFile(tauriConfFile, 'utf8', function(err, data) {
		if (err) {
			console.error(`Error while loading file: ${tauriConfFile}`);
			console.error(err);
			process.exit(1);
		}

		data = data.replace(/"version": "[0-9]+\.[0-9]+\.[0-9]+"/g, `"version": "${version}"`);
		fs.writeFile(tauriConfFile, data, 'utf8', function(err) {
			if (err) {
				console.error(`Error while write file: ${tauriConfFile}`);
				console.error(err);
				process.exit(1);
			}
		});
	});

	// change version in cargo
	await exec(`cd src-tauri & cargo bump ${version}`);

	// change version in npm
	try {
		await exec(`npm version --allow-same-version=true --commit-hooks=false --git-tag-version=false ${version}`);
	} catch (e) {
		console.error(e);
	}

	// changelog
	let changelogFile = './CHANGELOG.md';
	fs.readFile(changelogFile, 'utf8', function(err, data) {
		if (err) {
			console.error(`Error while loading file: ${changelogFile}`);
			console.error(err);
			process.exit(1);
		}

		if (data.indexOf(`[${version}]`) !== -1) {
			return;
		}

		data = data.replace(
			/## \[Unreleased\] - Unreleased/g,
			`## [Unreleased] - Unreleased\r\n\r\n\r\n## [${version}] - ${currentDate}`
		);

		data = data.replace(
			/\[unreleased\]: https:\/\/github.com\/MoQuEs\/smtp_client\/compare\/v([0-9]+\.[0-9]+\.[0-9]+)\.\.\.HEAD/g,
			`[unreleased]: https://github.com/MoQuEs/smtp_client/compare/v${version}...HEAD\r\n[${version}]: https://github.com/MoQuEs/smtp_client/compare/v$1...v${version}`
		);

		fs.writeFile(changelogFile, data, 'utf8', function(err) {
			if (err) {
				console.error(`Error while write file: ${changelogFile}`);
				console.error(err);
				process.exit(1);
			}
		});
	});
});

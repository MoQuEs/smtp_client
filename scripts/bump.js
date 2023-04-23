import fs from 'fs';
import { exec } from './exec.js';

exec('git rev-parse --abbrev-ref HEAD')
	.then(async (stdout) => {
		let matches = RegExp('^release/v([0-9]+.[0-9]+.[0-9]+)$', 'i').exec(stdout.trim());
		if (matches === null) {
			return;
		}

		let version = matches[1];
		version = '9.9.9';
		return version;
	})
	.then(async (version) => {
		await exec(`git stash -u`);
		return version;
	})
	.then((version) => {
		return new Promise((resolve, reject) => {
			let file = './src-tauri/tauri.conf.json';
			fs.readFile(file, 'utf8', function (err, data) {
				if (err) {
					console.error(`Error while loading file: ${file}`);
					console.error(err);
					reject(error, stderr);
					process.exit(1);
				}

				var result = data.replace(/"version": "[0-9]+.[0-9]+.[0-9]+"/g, `"version": "${version}"`);
				fs.writeFile(file, result, 'utf8', function (err) {
					if (err) {
						console.error(`Error while write file: ${file}`);
						console.error(err);
						reject(error, stderr);
						process.exit(1);
					}

					resolve(version);
				});
			});
		});
	})
	.then(async (version) => {
		await exec(`cargo bump ${version}`);
		return version;
	})
	.then(async (version) => {
		await exec(`npm version --commit-hooks false --git-tag-version false ${version}`);
		return version;
	})
	.then(async (version) => {
		await exec(`git add package.json`);
		return version;
	})
	.then(async (version) => {
		await exec(`git add package-lock.json`);
		return version;
	})
	.then(async (version) => {
		await exec(`git add ./src-tauri/Cargo.toml`);
		return version;
	})
	.then(async (version) => {
		await exec(`git add Cargo.lock`);
		return version;
	})
	.then(async (version) => {
		await exec(`git add ./src-tauri/tauri.conf.json`);
		return version;
	})
	.catch((a, b, c) => {
		console.error(a, b, c);
	});

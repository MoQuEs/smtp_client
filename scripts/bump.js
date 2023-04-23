import fs from 'fs';
import { exec } from './exec.js';

exec('git rev-parse --abbrev-ref HEAD').then(async (stdout) => {
	// get version
	let matches = RegExp('^release/v([0-9]+.[0-9]+.[0-9]+)$', 'gi').exec(stdout.trim());
	if (matches === null) {
		console.log("Can't find version from branch");
		process.exit(0);
	}

	let version = matches[1];

	// stash data
	await exec(`git stash -u -m "release_v${version}"`);

	// change version in tauri.conf.json
	let file = './src-tauri/tauri.conf.json';
	fs.readFile(file, 'utf8', function (err, data) {
		if (err) {
			console.error(`Error while loading file: ${file}`);
			console.error(err);
			process.exit(1);
		}

		var result = data.replace(/"version": "[0-9]+.[0-9]+.[0-9]+"/g, `"version": "${version}"`);
		fs.writeFile(file, result, 'utf8', function (err) {
			if (err) {
				console.error(`Error while write file: ${file}`);
				console.error(err);
				process.exit(1);
			}
		});
	});

	// change version in cargo
	await exec(`cargo bump ${version}`);

	// change version in npm
	await exec(
		`npm version --allow-same-version --commit-hooks false --git-tag-version false ${version}`
	);

	// add files to commit
	await exec(`git add package.json`);
	await exec(`git add package-lock.json`);
	await exec(`git add src-tauri/Cargo.toml`);
	await exec(`git add Cargo.lock`);
	await exec(`git add src-tauri/tauri.conf.json`);

	// check if needs to make commit
	await exec(`git status`).then(async (stdout) => {
		let matches = RegExp('modified:', 'gi').exec(stdout.trim());
		if (matches !== null) {
			// if needs to make commit make commit
			await exec(`git commit -m "auto commit release/v${version}"`);
		}
	});

	// check if stashed anny files
	await exec(`git stash list`).then(async (stdout) => {
		let matches = RegExp(`stash@{([0-9]+)}: On .*: release_v${version}`, 'gi').exec(stdout.trim());
		if (matches !== null) {
			// if found stash from this release pop it
			await exec(`git stash pop stash@{${matches[1]}}`);
		}
	});
});

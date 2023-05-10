import { exec } from './exec.js';

new Promise((resolve, reject) => {}).then(async () => {
	await exec('cargo fmt');
	await exec('npx prettier --ignore-unknown --write src');
	await exec('npx prettier --ignore-unknown --write scripts');

	resolve();
});

import { exec } from './exec.js';

new Promise((resolve, reject) => {
	resolve();
}).then(async () => {
	await exec('cargo fmt');
	await exec('npx prettier --ignore-unknown --write src');
	await exec('npx prettier --ignore-unknown --write scripts');
});

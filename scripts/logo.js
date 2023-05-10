import { exec } from './exec.js';
import fs from 'fs';

if (typeof process.argv[2] !== 'string' || process.argv[2] === '') {
	console.error('Logo file name to generate icons was not provided');
	process.exit(1);
}

const logoFilename = process.argv[2];

[`${logoFilename}.svg`, `${logoFilename}.png`, `${logoFilename}_text.svg`].forEach((file) => {
	if (!fs.existsSync(`logo\\${file}`)) {
		console.error(`Logo file logo\\${file} not exist`);
		process.exit(1);
	}
});

fs.copyFile(`logo\\${logoFilename}.svg`, `src\\assets\\logo.svg`, (err) => {
	if (err) {
		console.log(`Can't copy logo\\${logoFilename}.svg to src\\assets\\logo.svg`);
		console.error(err);
		process.exit(1);
	}
});

fs.copyFile(`logo\\${logoFilename}_text.svg`, `src\\assets\\logo_text.svg`, (err) => {
	if (err) {
		console.log(`Can't copy logo\\${logoFilename}_text.svg to src\\assets\\logo_text.svg`);
		console.error(err);
		process.exit(1);
	}
});

exec(`npm run tauri icon logo\\${logoFilename}.png`).then(() => {
	[
		[`src-tauri\\icons\\128x128.png`, `src\\assets\\favicon.png`],
		[`src-tauri\\icons\\128x128.png`, `static\\favicon.png`]
	].forEach((file) => {
		fs.copyFile(`${file[0]}`, `${file[1]}`, (err) => {
			if (err) {
				console.error(`Can't copy ${file[0]} to ${file[1]}`);
				console.error(err);
				process.exit(1);
			}
		});
	});
});

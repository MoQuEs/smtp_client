import { exec } from './exec.js';
import fs from 'fs';

new Promise((resolve, reject) => {
	['logo.png', 'logo.svg'].forEach((element) => {
		fs.copyFile(`static\\logo\\${element}`, `src\\assets\\logo\\${element}`, (err) => {
			if (err) {
				console.log(`Can't copy static\\logo\\${element} to src\\assets\\logo\\${element}`);
				console.error(err);
				reject();
			}
			resolve();
		});
	});
})
	.then(() => {
		exec('npm run tauri icon .\\static\\logo\\logo.png').then(() => {
			[
				['src-tauri\\icons\\128x128.png', 'static\\favicon.png'],
				['src-tauri\\icons\\128x128.png', 'src\\assets\\favicon.png']
			].forEach((elements) => {
				fs.copyFile(elements[0], elements[1], (err) => {
					if (err) {
						console.error(`Can't copy ${elements[0]} to ${elements[1]}`);
						console.error(err);
						process.exit(1);
					}
				});
			});
		});
	})
	.catch(() => process.exit(1));

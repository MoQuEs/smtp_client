import { exec as cmd_exec } from 'child_process';

export const exec = (cmd, exitOnError = true) => {
	return new Promise((resolve, reject) => {
		console.log(`exec: ${cmd}`);
		cmd_exec(cmd, (error, stdout, stderr) => {
			if (error !== null) {
				console.error(error, stderr);
				reject(error, stderr);
				if (exitOnError) {
					process.exit(1);
				}
				return;
			}

			resolve(stdout);
		});
	});
};

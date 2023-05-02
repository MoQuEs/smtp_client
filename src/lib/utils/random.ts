export let RandomId = (length: number = 10) => {
	let possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';

	function pickRandom() {
		return possible[Math.floor(Math.random() * possible.length)];
	}

	return Array.apply(null, Array(length)).map(pickRandom).join('');
};

export let RandomNumber = (min: number, max: number) =>
	Math.floor(Math.random() * (max - min) + min);

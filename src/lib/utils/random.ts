export const RandomId = (length = 10) => {
	const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';

	function pickRandom() {
		return possible[Math.floor(Math.random() * possible.length)];
	}

	return Array.from({ length }, pickRandom).join('');
};

export const RandomNumber = (min: number, max: number) =>
	Math.floor(Math.random() * (max - min) + min);

export let RandomId = () => Math.floor(Math.random() * 10000).toString();
export let RandomNumber = (min: number, max: number) =>
	Math.floor(Math.random() * (max - min) + min);

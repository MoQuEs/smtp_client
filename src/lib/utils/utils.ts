export const clone = <T = Object>(toClone: T): T => {
	if (typeof structuredClone === 'function') {
		return structuredClone(toClone);
	}

	return JSON.parse(JSON.stringify(toClone)) as T;
};

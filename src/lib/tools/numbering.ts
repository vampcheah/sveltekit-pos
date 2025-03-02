export const formatNumber = (number: number) => {
	return number
		? number.toLocaleString('en-US', {
				minimumFractionDigits: 2,
				maximumFractionDigits: 2
			})
		: 0;
};

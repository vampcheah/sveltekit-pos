import { project } from '$lib/index';

export const formatNumber = (number: number) => {
	return number
		? number.toLocaleString('en-US', {
				minimumFractionDigits: 2,
				maximumFractionDigits: 2
			})
		: 0;
};

export const numberWithCurrency = (number: number) => {
	return number.toLocaleString('en-US', {
		style: 'currency',
		currency: project.currency
	});
};

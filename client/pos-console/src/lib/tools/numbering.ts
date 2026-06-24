import { project } from '$lib/index';

export const formatNumber = (number: number): string => {
	return (number ?? 0).toLocaleString('en-US', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 2
	});
};

export const numberWithCurrency = (number: number) => {
	return (number ?? 0).toLocaleString('en-US', {
		style: 'currency',
		currency: project.currency
	});
};

export const isTrue = (value: unknown) => {
	return value === true || value === 'true' || value === 1 || value === '1';
};

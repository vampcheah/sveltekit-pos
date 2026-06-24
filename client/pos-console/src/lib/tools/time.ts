export const getCurrentTime = (offsetInHours: number = 8): Date => {
	const localDate = new Date();
	localDate.setHours(localDate.getHours() + offsetInHours);

	return localDate;
};

// a function which can get current time in different formats
export const formatDateTime = (date: string, format: string): string => {
	const d = new Date(date);
	const map: { [key: string]: string } = {
		DD: String(d.getUTCDate()).padStart(2, '0'),
		MM: String(d.getUTCMonth() + 1).padStart(2, '0'),
		YYYY: String(d.getUTCFullYear()),
		HH: String(d.getUTCHours()).padStart(2, '0'),
		mm: String(d.getUTCMinutes()).padStart(2, '0'),
		ss: String(d.getUTCSeconds()).padStart(2, '0')
	};

	return format.replace(/DD|MM|YYYY|HH|mm|ss/g, (matched) => map[matched]);
};

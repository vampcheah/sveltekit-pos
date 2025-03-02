export function getCurrentTime(offsetInHours: number = 8): string {
	const localDate = new Date();
	localDate.setHours(localDate.getHours() + offsetInHours);

	return localDate.toISOString().replace('T', ' ').substring(0, 19).replace(/-/g, '/');
}

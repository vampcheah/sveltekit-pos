// Pure formatting helpers (currency, numbers, dates, relative time, initials).
// No app or runtime dependencies — safe to import anywhere (server or client).

/**
 * Coerce a loose date input into a `Date`. Returns `null` for nullish or
 * unparseable values so callers can render a graceful fallback.
 */
function toDate(date: Date | string | number | null | undefined): Date | null {
	if (date === null || date === undefined) return null;
	const d = date instanceof Date ? date : new Date(date);
	return Number.isNaN(d.getTime()) ? null : d;
}

/** Format a numeric amount as localized currency (default USD, en-US). */
export function formatCurrency(amount: number, currency = 'USD', locale = 'en-US'): string {
	return new Intl.NumberFormat(locale, { style: 'currency', currency }).format(amount);
}

/** Format a number with locale-aware grouping; pass `options` to customize. */
export function formatNumber(value: number, options?: Intl.NumberFormatOptions): string {
	return new Intl.NumberFormat('en-US', options).format(value);
}

/**
 * Format a date as a localized date string. Returns an empty string for
 * nullish/invalid input. Defaults to medium date (e.g. "Jun 9, 2026").
 */
export function formatDate(
	date: Date | string | number | null | undefined,
	opts?: Intl.DateTimeFormatOptions
): string {
	const d = toDate(date);
	if (!d) return '';
	return new Intl.DateTimeFormat(
		'en-US',
		opts ?? { year: 'numeric', month: 'short', day: 'numeric' }
	).format(d);
}

/** Format a date including the time of day (e.g. "Jun 9, 2026, 3:04 PM"). */
export function formatDateTime(date: Date | string | number | null | undefined): string {
	const d = toDate(date);
	if (!d) return '';
	return new Intl.DateTimeFormat('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: 'numeric',
		minute: '2-digit'
	}).format(d);
}

const RELATIVE_UNITS: { unit: Intl.RelativeTimeFormatUnit; ms: number }[] = [
	{ unit: 'year', ms: 1000 * 60 * 60 * 24 * 365 },
	{ unit: 'month', ms: 1000 * 60 * 60 * 24 * 30 },
	{ unit: 'week', ms: 1000 * 60 * 60 * 24 * 7 },
	{ unit: 'day', ms: 1000 * 60 * 60 * 24 },
	{ unit: 'hour', ms: 1000 * 60 * 60 },
	{ unit: 'minute', ms: 1000 * 60 },
	{ unit: 'second', ms: 1000 }
];

/**
 * Human-readable relative time, e.g. "3 hours ago" or "in 2 days".
 * Returns an empty string for invalid input.
 */
export function formatRelativeTime(date: Date | string | number): string {
	const d = toDate(date);
	if (!d) return '';
	const diffMs = d.getTime() - Date.now();
	const abs = Math.abs(diffMs);
	const rtf = new Intl.RelativeTimeFormat('en-US', { numeric: 'auto' });
	for (const { unit, ms } of RELATIVE_UNITS) {
		if (abs >= ms || unit === 'second') {
			return rtf.format(Math.round(diffMs / ms), unit);
		}
	}
	return rtf.format(0, 'second');
}

/** Convert a date to a `YYYY-MM-DD` string (local time). */
export function toISODate(date: Date | string | number): string {
	const d = toDate(date);
	if (!d) return '';
	const year = d.getFullYear();
	const month = String(d.getMonth() + 1).padStart(2, '0');
	const day = String(d.getDate()).padStart(2, '0');
	return `${year}-${month}-${day}`;
}

/**
 * Derive up-to-two uppercase initials from a name.
 * "Jane Doe" -> "JD", "madonna" -> "M", "" -> "".
 */
export function initials(name: string): string {
	const parts = name.trim().split(/\s+/).filter(Boolean);
	if (parts.length === 0) return '';
	if (parts.length === 1) return parts[0]!.charAt(0).toUpperCase();
	return (parts[0]!.charAt(0) + parts[parts.length - 1]!.charAt(0)).toUpperCase();
}

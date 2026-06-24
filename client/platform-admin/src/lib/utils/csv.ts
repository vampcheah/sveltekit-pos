// Browser-side helpers for triggering file downloads and exporting tabular
// data as CSV. Guarded for SSR — calls are no-ops without a `document`.

/**
 * Trigger a browser download of `content` as `filename`.
 * No-op during SSR (no `document`).
 */
export function downloadBlob(content: BlobPart, filename: string, mime = 'text/plain'): void {
	if (typeof document === 'undefined') return;
	const blob = content instanceof Blob ? content : new Blob([content], { type: mime });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement('a');
	anchor.href = url;
	anchor.download = filename;
	anchor.style.display = 'none';
	document.body.appendChild(anchor);
	anchor.click();
	document.body.removeChild(anchor);
	// Release the object URL once the download has been kicked off.
	URL.revokeObjectURL(url);
}

/** Escape a single CSV cell, quoting when it contains special characters. */
function escapeCell(value: unknown): string {
	if (value === null || value === undefined) return '';
	const str = String(value);
	if (/[",\n\r]/.test(str)) {
		return `"${str.replace(/"/g, '""')}"`;
	}
	return str;
}

/**
 * Export an array of objects to a downloadable CSV file.
 *
 * - When `columns` is provided, only those keys are exported, in order, using
 *   the supplied headers.
 * - Otherwise every key found in the first row is used, with the key as header.
 */
export function exportToCsv<T extends object>(
	rows: T[],
	filename: string,
	columns?: { key: keyof T; header: string }[]
): void {
	const cols: { key: keyof T; header: string }[] =
		columns ??
		(rows.length > 0
			? (Object.keys(rows[0]!) as (keyof T)[]).map((key) => ({ key, header: String(key) }))
			: []);

	const headerLine = cols.map((c) => escapeCell(c.header)).join(',');
	const bodyLines = rows.map((row) => cols.map((c) => escapeCell(row[c.key])).join(','));
	// Prepend a BOM so Excel detects UTF-8 correctly.
	const csv = '﻿' + [headerLine, ...bodyLines].join('\r\n');

	downloadBlob(csv, filename, 'text/csv;charset=utf-8;');
}

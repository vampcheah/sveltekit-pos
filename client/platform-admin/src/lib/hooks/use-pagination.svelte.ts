// Headless pagination hook. Owns page/pageSize/total state and derives all the
// values a paginated UI needs, including a windowed page list (with `-1` as the
// ellipsis marker) suitable for rendering numbered pagination controls.

/** Selectable page sizes for a "rows per page" control. */
export const PAGE_SIZE_OPTIONS: readonly number[] = [10, 20, 50, 100];

export interface PaginationOptions {
	pageSize?: number;
	total?: number;
	initialPage?: number;
}

export interface Pagination {
	page: number;
	pageSize: number;
	total: number;
	readonly totalPages: number;
	readonly start: number;
	readonly end: number;
	readonly hasPrev: boolean;
	readonly hasNext: boolean;
	/** Windowed page numbers to render; `-1` marks an ellipsis gap. */
	readonly pages: number[];
	setPage(p: number): void;
	next(): void;
	prev(): void;
	setTotal(n: number): void;
	setPageSize(n: number): void;
}

/**
 * Build a compact, windowed list of page numbers around `current`.
 * Always shows the first and last page; uses `-1` to mark elided ranges.
 * e.g. for 10 pages on page 5 → [1, -1, 4, 5, 6, -1, 10].
 */
function buildPages(current: number, totalPages: number, siblings = 1): number[] {
	if (totalPages <= 1) return totalPages === 1 ? [1] : [];

	// How many page numbers we'd show if nothing were collapsed:
	// first + last + current + 2*siblings + 2 ellipsis slots.
	const totalSlots = siblings * 2 + 5;
	if (totalPages <= totalSlots) {
		return Array.from({ length: totalPages }, (_, i) => i + 1);
	}

	const leftSibling = Math.max(current - siblings, 1);
	const rightSibling = Math.min(current + siblings, totalPages);
	const showLeftDots = leftSibling > 2;
	const showRightDots = rightSibling < totalPages - 1;

	const first = 1;
	const last = totalPages;

	if (!showLeftDots && showRightDots) {
		const leftCount = siblings * 2 + 3;
		const left = Array.from({ length: leftCount }, (_, i) => i + 1);
		return [...left, -1, last];
	}

	if (showLeftDots && !showRightDots) {
		const rightCount = siblings * 2 + 3;
		const right = Array.from({ length: rightCount }, (_, i) => totalPages - rightCount + 1 + i);
		return [first, -1, ...right];
	}

	const middle = Array.from({ length: rightSibling - leftSibling + 1 }, (_, i) => leftSibling + i);
	return [first, -1, ...middle, -1, last];
}

/**
 * Create a pagination controller. All fields are reactive; `page`, `pageSize`
 * and `total` are assignable, while derived fields are read-only.
 */
export function usePagination(opts: PaginationOptions = {}): Pagination {
	let page = $state(Math.max(1, opts.initialPage ?? 1));
	let pageSize = $state(opts.pageSize ?? PAGE_SIZE_OPTIONS[0]!);
	let total = $state(opts.total ?? 0);

	const totalPages = $derived(Math.max(1, Math.ceil(total / pageSize)));
	// Keep the active page within bounds as total/pageSize change.
	const clampedPage = $derived(Math.min(Math.max(page, 1), totalPages));
	const start = $derived(total === 0 ? 0 : (clampedPage - 1) * pageSize + 1);
	const end = $derived(Math.min(clampedPage * pageSize, total));
	const hasPrev = $derived(clampedPage > 1);
	const hasNext = $derived(clampedPage < totalPages);
	const pages = $derived(buildPages(clampedPage, totalPages));

	function setPage(p: number): void {
		page = Math.min(Math.max(Math.trunc(p), 1), totalPages);
	}

	function next(): void {
		if (hasNext) page = clampedPage + 1;
	}

	function prev(): void {
		if (hasPrev) page = clampedPage - 1;
	}

	function setTotal(n: number): void {
		total = Math.max(0, Math.trunc(n));
		// Re-clamp so we never sit past the new last page.
		page = Math.min(Math.max(page, 1), Math.max(1, Math.ceil(total / pageSize)));
	}

	function setPageSize(n: number): void {
		pageSize = Math.max(1, Math.trunc(n));
		// Reset to the first page on a page-size change for predictable UX.
		page = 1;
	}

	return {
		get page() {
			return clampedPage;
		},
		set page(p: number) {
			setPage(p);
		},
		get pageSize() {
			return pageSize;
		},
		set pageSize(n: number) {
			setPageSize(n);
		},
		get total() {
			return total;
		},
		set total(n: number) {
			setTotal(n);
		},
		get totalPages() {
			return totalPages;
		},
		get start() {
			return start;
		},
		get end() {
			return end;
		},
		get hasPrev() {
			return hasPrev;
		},
		get hasNext() {
			return hasNext;
		},
		get pages() {
			return pages;
		},
		setPage,
		next,
		prev,
		setTotal,
		setPageSize
	};
}

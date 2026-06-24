<!--
  DataTable — a generic, client-side data table.

  Features (all client-side, no backend):
    • text search across columns flagged `searchable`
    • click-to-sort on columns flagged `sortable` (asc → desc → asc)
    • pagination via the `usePagination` hook + the ui/pagination primitives
    • optional row selection (checkbox column; `selected` is bindable)
    • a loading skeleton state and an EmptyState when there is no data
    • snippet hooks for custom cells (`cell`), per-row `actions`, and a `toolbar`

  The `Column<T>` interface is exported from this file and re-exported as a
  type from the shared barrel.
-->
<script lang="ts" module>
	export interface Column<T> {
		key: string;
		header: string;
		sortable?: boolean;
		searchable?: boolean;
		class?: string;
		align?: 'left' | 'right' | 'center';
		/** Custom value extractor used for display, search and sort. */
		render?: (row: T) => string | number;
	}
</script>

<script lang="ts" generics="T extends { id: string | number }">
	import type { Snippet } from 'svelte';
	import ArrowUp from '@lucide/svelte/icons/arrow-up';
	import ArrowDown from '@lucide/svelte/icons/arrow-down';
	import ChevronsUpDown from '@lucide/svelte/icons/chevrons-up-down';
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import Inbox from '@lucide/svelte/icons/inbox';
	import * as Table from '$lib/components/ui/table';
	import * as Pagination from '$lib/components/ui/pagination';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { buttonVariants } from '$lib/components/ui/button';
	import { usePagination } from '$lib/hooks/use-pagination.svelte';
	import { cn } from '$lib/utils';
	import { config } from '$lib/config';
	import EmptyState from './EmptyState.svelte';
	import SearchInput from './SearchInput.svelte';

	interface Props {
		data: T[];
		columns: Column<T>[];
		searchable?: boolean;
		selectable?: boolean;
		loading?: boolean;
		pageSize?: number;
		emptyTitle?: string;
		emptyDescription?: string;
		/** Currently selected row ids (bindable). */
		selected?: (string | number)[];
		/** Render a custom cell; receives the row and its column. */
		cell?: Snippet<[T, Column<T>]>;
		/** Trailing per-row actions column. */
		actions?: Snippet<[T]>;
		/** Extra controls rendered in the toolbar (left of search). */
		toolbar?: Snippet;
	}

	let {
		data,
		columns,
		searchable = false,
		selectable = false,
		loading = false,
		pageSize = config.ui.pageSize,
		emptyTitle = 'No results',
		emptyDescription = 'There is nothing to show here yet.',
		selected = $bindable([]),
		cell,
		actions,
		toolbar
	}: Props = $props();

	let search = $state('');
	let sortKey = $state<string | null>(null);
	let sortDir = $state<'asc' | 'desc'>('asc');

	// `pageSize` seeds the paginator; keep it in sync if the prop later changes.
	// svelte-ignore state_referenced_locally
	const pagination = usePagination({ pageSize });
	$effect(() => {
		pagination.setPageSize(pageSize);
	});

	/** Read a column's comparable/display value for a row. */
	function valueOf(row: T, column: Column<T>): string | number {
		if (column.render) return column.render(row);
		const raw = (row as Record<string, unknown>)[column.key];
		return raw == null ? '' : (raw as string | number);
	}

	// --- Filtering ---------------------------------------------------------
	const searchableColumns = $derived(columns.filter((c) => c.searchable));

	const filtered = $derived.by(() => {
		const query = search.trim().toLowerCase();
		if (!query || searchableColumns.length === 0) return data;
		return data.filter((row) =>
			searchableColumns.some((c) => String(valueOf(row, c)).toLowerCase().includes(query))
		);
	});

	// --- Sorting -----------------------------------------------------------
	const sorted = $derived.by(() => {
		if (!sortKey) return filtered;
		const column = columns.find((c) => c.key === sortKey);
		if (!column) return filtered;
		const factor = sortDir === 'asc' ? 1 : -1;
		return [...filtered].sort((a, b) => {
			const av = valueOf(a, column);
			const bv = valueOf(b, column);
			if (typeof av === 'number' && typeof bv === 'number') return (av - bv) * factor;
			return String(av).localeCompare(String(bv), undefined, { numeric: true }) * factor;
		});
	});

	// Keep the paginator's total in sync with the filtered/sorted row count.
	$effect(() => {
		pagination.setTotal(sorted.length);
	});

	// Reset to the first page whenever the search query changes.
	$effect(() => {
		// Bare reference registers `search` as a reactive dependency of this effect.
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		search;
		pagination.setPage(1);
	});

	// --- Pagination slice --------------------------------------------------
	// Derive bounds from page/pageSize directly so behaviour is independent of
	// the hook's `start`/`end` index convention.
	const sliceStart = $derived((pagination.page - 1) * pagination.pageSize);
	const sliceEnd = $derived(Math.min(sliceStart + pagination.pageSize, sorted.length));
	const pageRows = $derived(sorted.slice(sliceStart, sliceEnd));

	const columnCount = $derived(columns.length + (selectable ? 1 : 0) + (actions ? 1 : 0));

	const allOnPageSelected = $derived(
		pageRows.length > 0 && pageRows.every((row) => selected.includes(row.id))
	);
	const someOnPageSelected = $derived(
		pageRows.some((row) => selected.includes(row.id)) && !allOnPageSelected
	);

	function toggleSort(column: Column<T>) {
		if (!column.sortable) return;
		if (sortKey === column.key) {
			sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		} else {
			sortKey = column.key;
			sortDir = 'asc';
		}
		pagination.setPage(1);
	}

	function toggleRow(row: T, checked: boolean) {
		if (checked) {
			if (!selected.includes(row.id)) selected = [...selected, row.id];
		} else {
			selected = selected.filter((id) => id !== row.id);
		}
	}

	function toggleAllOnPage(checked: boolean) {
		const pageIds = pageRows.map((r) => r.id);
		if (checked) {
			const merged = new Set([...selected, ...pageIds]);
			selected = [...merged];
		} else {
			selected = selected.filter((id) => !pageIds.includes(id));
		}
	}

	function alignClass(align?: 'left' | 'right' | 'center') {
		return align === 'right' ? 'text-right' : align === 'center' ? 'text-center' : 'text-left';
	}
</script>

<div class="space-y-4">
	{#if searchable || toolbar}
		<div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
			{#if toolbar}
				<div class="flex flex-wrap items-center gap-2">{@render toolbar()}</div>
			{:else}
				<div></div>
			{/if}
			{#if searchable}
				<div class="w-full sm:w-64">
					<SearchInput bind:value={search} placeholder="Search..." />
				</div>
			{/if}
		</div>
	{/if}

	<div class="bg-card overflow-hidden rounded-lg border border-border">
		<div class="overflow-x-auto">
			<Table.Root>
				<Table.Header>
					<Table.Row class="bg-primary/80 hover:[&>th]:!bg-primary/80">
						{#if selectable}
							<Table.Head class="w-10">
								<Checkbox
									checked={allOnPageSelected}
									indeterminate={someOnPageSelected}
									onCheckedChange={(v) => toggleAllOnPage(v === true)}
									aria-label="Select all rows on this page"
								/>
							</Table.Head>
						{/if}
						{#each columns as column (column.key)}
							<Table.Head
								class={cn(
									'text-primary-foreground/80 text-xs font-medium tracking-wide uppercase',
									alignClass(column.align),
									column.class
								)}
							>
								{#if column.sortable}
									<button
										type="button"
										onclick={() => toggleSort(column)}
										class={cn(
											'text-primary-foreground/80 hover:text-primary-foreground focus-visible:ring-primary-foreground/50 -mx-1 inline-flex items-center gap-1 rounded px-1 py-0.5 font-medium transition-colors focus-visible:outline-none focus-visible:ring-2',
											column.align === 'right' && 'flex-row-reverse'
										)}
										aria-label={`Sort by ${column.header}`}
									>
										{column.header}
										{#if sortKey === column.key}
											{#if sortDir === 'asc'}
												<ArrowUp class="text-primary-foreground size-3.5" aria-hidden="true" />
											{:else}
												<ArrowDown class="text-primary-foreground size-3.5" aria-hidden="true" />
											{/if}
										{:else}
											<ChevronsUpDown
												class="text-primary-foreground/60 size-3.5"
												aria-hidden="true"
											/>
										{/if}
									</button>
								{:else}
									{column.header}
								{/if}
							</Table.Head>
						{/each}
						{#if actions}
							<Table.Head class="w-12 text-right">
								<span class="sr-only">Actions</span>
							</Table.Head>
						{/if}
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#if loading}
						{#each Array(Math.min(pageSize, 5)) as _, i (i)}
							<Table.Row>
								{#if selectable}
									<Table.Cell><Skeleton class="size-4 rounded" /></Table.Cell>
								{/if}
								{#each columns as column (column.key)}
									<Table.Cell class={cn(alignClass(column.align), column.class)}>
										<Skeleton class="h-4 w-24" />
									</Table.Cell>
								{/each}
								{#if actions}
									<Table.Cell class="text-right"><Skeleton class="ms-auto h-4 w-8" /></Table.Cell>
								{/if}
							</Table.Row>
						{/each}
					{:else if pageRows.length === 0}
						<Table.Row class="hover:bg-transparent">
							<Table.Cell colspan={columnCount} class="p-0">
								<EmptyState
									icon={Inbox}
									title={search ? 'No matches' : emptyTitle}
									description={search ? 'Try adjusting your search terms.' : emptyDescription}
									class="rounded-none border-0"
								/>
							</Table.Cell>
						</Table.Row>
					{:else}
						{#each pageRows as row (row.id)}
							<Table.Row data-state={selected.includes(row.id) ? 'selected' : undefined}>
								{#if selectable}
									<Table.Cell class="w-10">
										<Checkbox
											checked={selected.includes(row.id)}
											onCheckedChange={(v) => toggleRow(row, v === true)}
											aria-label="Select row"
										/>
									</Table.Cell>
								{/if}
								{#each columns as column (column.key)}
									<Table.Cell class={cn(alignClass(column.align), column.class)}>
										{#if cell}
											{@render cell(row, column)}
										{:else}
											{valueOf(row, column)}
										{/if}
									</Table.Cell>
								{/each}
								{#if actions}
									<Table.Cell class="text-right">
										{@render actions(row)}
									</Table.Cell>
								{/if}
							</Table.Row>
						{/each}
					{/if}
				</Table.Body>
			</Table.Root>
		</div>
	</div>

	{#if !loading && sorted.length > 0}
		<div class="flex flex-col items-center justify-between gap-3 sm:flex-row">
			<p class="text-sm whitespace-nowrap text-muted-foreground">
				{#if selectable && selected.length > 0}
					{selected.length} selected ·
				{/if}
				Showing
				<span class="font-medium text-foreground">{sorted.length === 0 ? 0 : sliceStart + 1}</span
				>–<span class="font-medium text-foreground">{sliceEnd}</span>
				of <span class="font-medium text-foreground">{sorted.length}</span>
			</p>

			{#if pagination.totalPages > 1}
				<Pagination.Root
					count={sorted.length}
					perPage={pagination.pageSize}
					page={pagination.page}
					onPageChange={(p) => pagination.setPage(p)}
					class="mx-0 w-auto justify-end"
				>
					<Pagination.Content>
						<Pagination.Item>
							<button
								type="button"
								onclick={() => pagination.prev()}
								disabled={!pagination.hasPrev}
								aria-label="Go to previous page"
								class={cn(buttonVariants({ variant: 'ghost', size: 'icon' }))}
							>
								<ChevronLeft class="size-4" aria-hidden="true" />
							</button>
						</Pagination.Item>

						{#each pagination.pages as p, i (i)}
							<Pagination.Item>
								{#if p === -1}
									<Pagination.Ellipsis />
								{:else}
									<button
										type="button"
										onclick={() => pagination.setPage(p)}
										aria-label={`Go to page ${p}`}
										aria-current={p === pagination.page ? 'page' : undefined}
										class={cn(
											buttonVariants({
												variant: p === pagination.page ? 'outline' : 'ghost',
												size: 'icon'
											})
										)}
									>
										{p}
									</button>
								{/if}
							</Pagination.Item>
						{/each}

						<Pagination.Item>
							<button
								type="button"
								onclick={() => pagination.next()}
								disabled={!pagination.hasNext}
								aria-label="Go to next page"
								class={cn(buttonVariants({ variant: 'ghost', size: 'icon' }))}
							>
								<ChevronRight class="size-4" aria-hidden="true" />
							</button>
						</Pagination.Item>
					</Pagination.Content>
				</Pagination.Root>
			{/if}
		</div>
	{/if}
</div>

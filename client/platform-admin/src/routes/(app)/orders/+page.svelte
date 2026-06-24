<!-- 订单管理 — GET /orders、POST /orders/:id/refund。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { PageContainer, PageHeader, DataTable, StatusBadge, ConfirmDialog, type Column, type BadgeTone } from '$lib/components/shared';
	import { Button } from '$lib/components/ui/button';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface Order {
		id: number;
		order_no: string;
		kind: string;
		store_id: number;
		cashier_id: number | null;
		total: string;
		status: string;
		created_at: string;
	}

	let rows = $state<Order[]>([]);
	let loading = $state(true);
	let refundOpen = $state(false);
	let refundTarget = $state<Order | null>(null);

	const columns = $derived<Column<Order>[]>([
		{ key: 'order_no', header: t('orders.no'), sortable: true, searchable: true },
		{ key: 'kind', header: t('orders.kind'), render: (r) => (r.kind === 'refund' ? t('orders.refund') : t('orders.sale')) },
		{ key: 'total', header: t('orders.amount'), align: 'right', sortable: true, render: (r) => `RM ${r.total}` },
		{ key: 'status', header: t('common.status') },
		{ key: 'created_at', header: t('orders.time'), sortable: true, render: (r) => new Date(r.created_at).toLocaleString() }
	]);

	const tone = (s: string): BadgeTone => (s === 'paid' ? 'brand' : s === 'refunded' ? 'outline' : 'neutral');

	async function load() {
		loading = true;
		try {
			rows = await api.get<Order[]>('/orders');
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);

	function askRefund(o: Order) {
		if (o.kind !== 'sale' || o.status !== 'paid') return;
		refundTarget = o;
		refundOpen = true;
	}
	async function doRefund() {
		if (!refundTarget) return;
		try {
			await api.post(`/orders/${refundTarget.id}/refund`, { reason_code: 'admin' });
			toast.success(t('orders.refunded', { no: refundTarget.order_no }));
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('orders.refundFailed'));
			throw e;
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('orders.title')} description={t('orders.desc')} />
	<DataTable data={rows} {columns} {loading} searchable emptyTitle={t('orders.empty')}>
		{#snippet cell(row, col)}
			{#if col.key === 'status'}
				<StatusBadge tone={tone(row.status)}>{row.status}</StatusBadge>
			{:else if col.render}
				{col.render(row)}
			{:else}
				{row[col.key as keyof Order]}
			{/if}
		{/snippet}
		{#snippet actions(row)}
			{#if row.kind === 'sale' && row.status === 'paid'}
				<Button variant="outline" size="sm" onclick={() => askRefund(row)}>{t('orders.refund')}</Button>
			{/if}
		{/snippet}
	</DataTable>
</PageContainer>

<ConfirmDialog
	bind:open={refundOpen}
	title={t('orders.refundTitle')}
	description={t('orders.refundDesc', { no: refundTarget?.order_no ?? '', total: refundTarget?.total ?? '' })}
	confirmText={t('orders.refund')}
	cancelText={t('common.cancel')}
	variant="destructive"
	onConfirm={doRefund}
/>

<!-- 收益报表 — /reports/*（营收/毛利/储值负债）。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Receipt from '@lucide/svelte/icons/receipt';
	import TrendingUp from '@lucide/svelte/icons/trending-up';
	import Wallet from '@lucide/svelte/icons/wallet';
	import { PageContainer, PageHeader, StatCard, DataTable, type Column } from '$lib/components/shared';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface RevenueRow {
		id: string;
		bucket: string;
		sales: string;
		refunds: string;
		net: string;
		orders: number;
	}

	let loading = $state(true);
	let margin = $state({ revenue: '0', cogs: '0', margin: '0' });
	let liability = $state({ total_stored_value: '0', members_with_balance: 0 });
	let revenue = $state<RevenueRow[]>([]);
	let period = $state<'week' | 'month' | 'year'>('month');

	const columns = $derived<Column<RevenueRow>[]>([
		{ key: 'bucket', header: t('reports.period'), render: (r) => new Date(r.bucket).toLocaleDateString() },
		{ key: 'sales', header: t('reports.sales'), align: 'right', render: (r) => `RM ${r.sales}` },
		{ key: 'refunds', header: t('reports.refunds'), align: 'right', render: (r) => `RM ${r.refunds}` },
		{ key: 'net', header: t('reports.net'), align: 'right', render: (r) => `RM ${r.net}` },
		{ key: 'orders', header: t('reports.ordersCount'), align: 'right' }
	]);

	async function load() {
		loading = true;
		try {
			const [m, l, rev] = await Promise.all([
				api.get<typeof margin>('/reports/margin'),
				api.get<typeof liability>('/reports/member-liability'),
				api.get<Omit<RevenueRow, 'id'>[]>(`/reports/revenue?period=${period}`)
			]);
			margin = m;
			liability = l;
			revenue = rev.map((r, i) => ({ ...r, id: String(i) }));
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);
</script>

<PageContainer>
	<PageHeader title={t('reports.title')} description={t('reports.desc')} />

	<div class="grid gap-4 sm:grid-cols-3">
		<StatCard title={t('reports.revenue')} value="RM {margin.revenue}" icon={Receipt} />
		<StatCard title={t('reports.margin')} value="RM {margin.margin}" icon={TrendingUp} hint={t('reports.marginHint')} />
		<StatCard title={t('reports.liability')} value="RM {liability.total_stored_value}" icon={Wallet} hint={t('reports.liabilityHint', { n: liability.members_with_balance })} />
	</div>

	<div class="mt-6">
		<DataTable data={revenue} {columns} {loading} emptyTitle={t('reports.empty')} />
	</div>
</PageContainer>

<!-- 看板 — /reports/dashboard（今日 KPI + Top 商品）。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import ShoppingBag from '@lucide/svelte/icons/shopping-bag';
	import DollarSign from '@lucide/svelte/icons/dollar-sign';
	import Receipt from '@lucide/svelte/icons/receipt';
	import { PageContainer, PageHeader, StatCard } from '$lib/components/shared';
	import * as Card from '$lib/components/ui/card';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface Dashboard {
		today_orders: number;
		today_revenue: string;
		avg_ticket: string;
		top_products: { name: string; qty: string }[];
	}

	let d = $state<Dashboard>({ today_orders: 0, today_revenue: '0', avg_ticket: '0', top_products: [] });

	onMount(async () => {
		try {
			d = await api.get<Dashboard>('/reports/dashboard');
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		}
	});
</script>

<PageContainer>
	<PageHeader title={t('dashboard.title')} description={t('dashboard.desc')} />

	<div class="grid gap-4 sm:grid-cols-3">
		<StatCard title={t('dashboard.todayOrders')} value={d.today_orders} icon={ShoppingBag} />
		<StatCard title={t('dashboard.todayRevenue')} value="RM {d.today_revenue}" icon={DollarSign} />
		<StatCard title={t('dashboard.avgTicket')} value="RM {Number(d.avg_ticket).toFixed(2)}" icon={Receipt} />
	</div>

	<Card.Root class="mt-6">
		<Card.Header><Card.Title>{t('dashboard.topToday')}</Card.Title></Card.Header>
		<Card.Content>
			{#if d.top_products.length === 0}
				<p class="text-sm text-muted-foreground">{t('dashboard.noSales')}</p>
			{:else}
				<ul class="divide-y">
					{#each d.top_products as p}
						<li class="flex justify-between py-2 text-sm"><span>{p.name}</span><span class="font-medium">{p.qty}</span></li>
					{/each}
				</ul>
			{/if}
		</Card.Content>
	</Card.Root>
</PageContainer>

<!-- 商品管理 — GET/POST/PATCH/DELETE /products。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Plus from '@lucide/svelte/icons/plus';
	import {
		PageContainer, PageHeader, DataTable, StatusBadge, ConfirmDialog,
		type Column, type BadgeTone
	} from '$lib/components/shared';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Sheet from '$lib/components/ui/sheet';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface Product {
		id: number;
		sku: string;
		name: string;
		price: string;
		cost: string;
		is_weighted: boolean;
		tax_rate: string;
		status: string;
	}

	let rows = $state<Product[]>([]);
	let loading = $state(true);
	let open = $state(false);
	let saving = $state(false);
	let editId = $state<number | null>(null);
	let form = $state({ sku: '', name: '', price: '', cost: '', tax_rate: '0.06' });
	let confirmOpen = $state(false);
	let deleteTarget = $state<Product | null>(null);

	const columns = $derived<Column<Product>[]>([
		{ key: 'sku', header: t('products.sku'), sortable: true, searchable: true },
		{ key: 'name', header: t('products.name'), sortable: true, searchable: true },
		{ key: 'price', header: t('products.price'), align: 'right', sortable: true, render: (r) => `RM ${r.price}` },
		{ key: 'cost', header: t('products.cost'), align: 'right', render: (r) => `RM ${r.cost}` },
		{ key: 'tax_rate', header: t('products.taxRate'), align: 'right', render: (r) => `${(Number(r.tax_rate) * 100).toFixed(0)}%` },
		{ key: 'status', header: t('common.status') }
	]);

	const tone = (s: string): BadgeTone => (s === 'active' ? 'brand' : 'outline');

	async function load() {
		loading = true;
		try {
			rows = await api.get<Product[]>('/products');
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);

	function openCreate() {
		editId = null;
		form = { sku: '', name: '', price: '', cost: '', tax_rate: '0.06' };
		open = true;
	}
	function openEdit(p: Product) {
		editId = p.id;
		form = { sku: p.sku, name: p.name, price: p.price, cost: p.cost, tax_rate: p.tax_rate };
		open = true;
	}
	async function save() {
		saving = true;
		try {
			if (editId == null) {
				await api.post('/products', { sku: form.sku, name: form.name, price: form.price, cost: form.cost || '0', tax_rate: form.tax_rate || '0' });
				toast.success(t('products.created'));
			} else {
				await api.patch(`/products/${editId}`, { name: form.name, price: form.price, cost: form.cost, tax_rate: form.tax_rate });
				toast.success(t('products.saved'));
			}
			open = false;
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.save'));
		} finally {
			saving = false;
		}
	}
	function askRemove(p: Product) { deleteTarget = p; confirmOpen = true; }
	async function doRemove() {
		if (!deleteTarget) return;
		try {
			await api.del(`/products/${deleteTarget.id}`);
			toast.success(t('products.deleted'));
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
			throw e;
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('products.title')} description={t('products.desc')}>
		{#snippet actions()}
			<Button onclick={openCreate}><Plus class="mr-1 size-4" />{t('products.new')}</Button>
		{/snippet}
	</PageHeader>

	<DataTable data={rows} {columns} {loading} searchable emptyTitle={t('products.empty')}>
		{#snippet cell(row, col)}
			{#if col.key === 'status'}
				<StatusBadge tone={tone(row.status)}>{row.status}</StatusBadge>
			{:else if col.render}
				{col.render(row)}
			{:else}
				{row[col.key as keyof Product]}
			{/if}
		{/snippet}
		{#snippet actions(row)}
			<Button variant="outline" size="sm" onclick={() => openEdit(row)}>{t('common.edit')}</Button>
			<Button variant="outline" size="sm" onclick={() => askRemove(row)}>{t('common.delete')}</Button>
		{/snippet}
	</DataTable>
</PageContainer>

<Sheet.Root bind:open>
	<Sheet.Content class="flex flex-col gap-4">
		<Sheet.Header><Sheet.Title>{editId == null ? t('products.createTitle') : t('products.editTitle')}</Sheet.Title></Sheet.Header>
		<div class="space-y-3 px-1">
			<div class="space-y-1"><Label>{t('products.sku')}</Label><Input bind:value={form.sku} placeholder="SKU001" disabled={editId != null} /></div>
			<div class="space-y-1"><Label>{t('products.name')}</Label><Input bind:value={form.name} placeholder="可乐" /></div>
			<div class="space-y-1"><Label>{t('products.price')}</Label><Input bind:value={form.price} placeholder="3.50" /></div>
			<div class="space-y-1"><Label>{t('products.cost')}</Label><Input bind:value={form.cost} placeholder="2.00" /></div>
			<div class="space-y-1"><Label>{t('products.taxRate')} (0.06=6%)</Label><Input bind:value={form.tax_rate} /></div>
		</div>
		<Sheet.Footer>
			<Button onclick={save} disabled={saving || !form.sku || !form.name || !form.price}>
				{saving ? t('common.saving') : t('common.save')}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

<ConfirmDialog
	bind:open={confirmOpen}
	title={t('products.deleteTitle')}
	description={t('products.deleteDesc', { name: deleteTarget?.name ?? '' })}
	confirmText={t('common.delete')}
	cancelText={t('common.cancel')}
	variant="destructive"
	onConfirm={doRemove}
/>

<!-- 会员管理 — GET /members + 充值/积分 + 新建。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Plus from '@lucide/svelte/icons/plus';
	import { PageContainer, PageHeader, DataTable, type Column } from '$lib/components/shared';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Sheet from '$lib/components/ui/sheet';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface Member {
		id: number;
		name: string;
		phone: string | null;
		tier: string;
		points: number;
		balance: string;
		status: string;
	}

	let rows = $state<Member[]>([]);
	let loading = $state(true);
	let mode = $state<'create' | 'topup' | 'points' | null>(null);
	let target = $state<Member | null>(null);
	let saving = $state(false);
	let createForm = $state({ name: '', phone: '' });
	let amount = $state('');

	const columns = $derived<Column<Member>[]>([
		{ key: 'name', header: t('members.name'), sortable: true, searchable: true },
		{ key: 'phone', header: t('members.phone'), searchable: true },
		{ key: 'tier', header: t('members.tier') },
		{ key: 'points', header: t('members.points'), align: 'right', sortable: true },
		{ key: 'balance', header: t('members.balance'), align: 'right', sortable: true, render: (r) => `RM ${r.balance}` },
		{ key: 'status', header: t('common.status') }
	]);

	async function load() {
		loading = true;
		try {
			rows = await api.get<Member[]>('/members');
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);

	function openTopup(m: Member) { target = m; amount = ''; mode = 'topup'; }
	function openPoints(m: Member) { target = m; amount = ''; mode = 'points'; }
	function openCreate() { createForm = { name: '', phone: '' }; mode = 'create'; }

	async function submit() {
		saving = true;
		try {
			if (mode === 'create') {
				await api.post('/members', { name: createForm.name, phone: createForm.phone || null });
				toast.success(t('members.created'));
			} else if (mode === 'topup' && target) {
				await api.post(`/members/${target.id}/topup`, { amount });
				toast.success(t('members.toppedUp', { amount }));
			} else if (mode === 'points' && target) {
				await api.post(`/members/${target.id}/points`, { points_delta: Number(amount) });
				toast.success(t('members.pointsDone'));
			}
			mode = null;
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
		} finally {
			saving = false;
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('members.title')} description={t('members.desc')}>
		{#snippet actions()}
			<Button onclick={openCreate}><Plus class="mr-1 size-4" />{t('members.new')}</Button>
		{/snippet}
	</PageHeader>

	<DataTable data={rows} {columns} {loading} searchable emptyTitle={t('members.empty')}>
		{#snippet actions(row)}
			<Button variant="outline" size="sm" onclick={() => openTopup(row)}>{t('members.topup')}</Button>
			<Button variant="outline" size="sm" onclick={() => openPoints(row)}>{t('members.points2')}</Button>
		{/snippet}
	</DataTable>
</PageContainer>

<Sheet.Root open={mode !== null} onOpenChange={(v) => { if (!v) mode = null; }}>
	<Sheet.Content class="flex flex-col gap-4">
		<Sheet.Header>
			<Sheet.Title>
				{mode === 'create' ? t('members.createTitle') : mode === 'topup' ? t('members.topupTitle', { name: target?.name ?? '' }) : t('members.pointsTitle', { name: target?.name ?? '' })}
			</Sheet.Title>
		</Sheet.Header>
		<div class="space-y-3 px-1">
			{#if mode === 'create'}
				<div class="space-y-1"><Label>{t('members.name')}</Label><Input bind:value={createForm.name} /></div>
				<div class="space-y-1"><Label>{t('members.phone')}</Label><Input bind:value={createForm.phone} /></div>
			{:else if mode === 'topup'}
				<div class="space-y-1"><Label>{t('members.topupAmount')}</Label><Input bind:value={amount} placeholder="100.00" /></div>
			{:else if mode === 'points'}
				<div class="space-y-1"><Label>{t('members.pointsDelta')}</Label><Input bind:value={amount} placeholder="50 / -20" /></div>
			{/if}
		</div>
		<Sheet.Footer>
			<Button onclick={submit} disabled={saving || (mode === 'create' ? !createForm.name : !amount)}>
				{saving ? t('common.processing') : t('common.confirm')}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

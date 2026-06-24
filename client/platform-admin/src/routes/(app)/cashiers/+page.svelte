<!-- 收银员管理 — GET /cashiers + 新建/重置PIN/停用启用。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Plus from '@lucide/svelte/icons/plus';
	import { PageContainer, PageHeader, DataTable, StatusBadge, ConfirmDialog, type Column, type BadgeTone } from '$lib/components/shared';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Sheet from '$lib/components/ui/sheet';
	import * as Dialog from '$lib/components/ui/dialog';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface Cashier {
		id: number;
		username: string;
		full_name: string | null;
		is_supervisor: boolean;
		status: string;
		last_login_at: string | null;
	}
	interface Store { id: number; name: string }

	let rows = $state<Cashier[]>([]);
	let stores = $state<Store[]>([]);
	let loading = $state(true);
	let open = $state(false);
	let saving = $state(false);
	let form = $state({ username: '', pin: '', full_name: '', is_supervisor: false, store_id: 0 });
	let pinOpen = $state(false);
	let pinTarget = $state<Cashier | null>(null);
	let newPin = $state('');
	let toggleOpen = $state(false);
	let toggleTarget = $state<Cashier | null>(null);
	const toggleDisabling = $derived(toggleTarget?.status === 'active');

	const columns = $derived<Column<Cashier>[]>([
		{ key: 'username', header: t('cashiers.username'), sortable: true, searchable: true },
		{ key: 'full_name', header: t('cashiers.fullName'), searchable: true },
		{ key: 'is_supervisor', header: t('cashiers.supervisor'), render: (r) => (r.is_supervisor ? t('common.confirm') : t('common.none')) },
		{ key: 'status', header: t('common.status') },
		{ key: 'last_login_at', header: t('cashiers.lastLogin'), render: (r) => (r.last_login_at ? new Date(r.last_login_at).toLocaleString() : t('common.none')) }
	]);
	const tone = (s: string): BadgeTone => (s === 'active' ? 'brand' : 'outline');

	async function load() {
		loading = true;
		try {
			[rows, stores] = await Promise.all([api.get<Cashier[]>('/cashiers'), api.get<Store[]>('/stores')]);
			if (!form.store_id && stores[0]) form.store_id = stores[0].id;
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);

	function openCreate() {
		form = { username: '', pin: '', full_name: '', is_supervisor: false, store_id: stores[0]?.id ?? 0 };
		open = true;
	}
	async function create() {
		saving = true;
		try {
			await api.post('/cashiers', {
				username: form.username, pin: form.pin, full_name: form.full_name || null,
				is_supervisor: form.is_supervisor, store_ids: [form.store_id], home_store_id: form.store_id
			});
			toast.success(t('cashiers.created'));
			open = false;
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.save'));
		} finally {
			saving = false;
		}
	}
	function askResetPin(c: Cashier) { pinTarget = c; newPin = ''; pinOpen = true; }
	async function doResetPin() {
		if (!pinTarget || !newPin) return;
		try {
			await api.post(`/cashiers/${pinTarget.id}/reset-pin`, { new_pin: newPin });
			toast.success(t('cashiers.pinReset'));
			pinOpen = false;
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
		}
	}
	function onToggle(c: Cashier) { toggleTarget = c; toggleOpen = true; }
	async function doToggle() {
		if (!toggleTarget) return;
		const disabling = toggleTarget.status === 'active';
		try {
			await api.post(`/cashiers/${toggleTarget.id}/${disabling ? 'disable' : 'enable'}`);
			toast.success(disabling ? t('cashiers.disabled') : t('cashiers.enabled'));
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
			throw e;
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('cashiers.title')} description={t('cashiers.desc')}>
		{#snippet actions()}
			<Button onclick={openCreate}><Plus class="mr-1 size-4" />{t('cashiers.new')}</Button>
		{/snippet}
	</PageHeader>

	<DataTable data={rows} {columns} {loading} searchable emptyTitle={t('cashiers.empty')}>
		{#snippet cell(row, col)}
			{#if col.key === 'status'}
				<StatusBadge tone={tone(row.status)}>{row.status}</StatusBadge>
			{:else if col.render}
				{col.render(row)}
			{:else}
				{row[col.key as keyof Cashier]}
			{/if}
		{/snippet}
		{#snippet actions(row)}
			<Button variant="outline" size="sm" onclick={() => askResetPin(row)}>{t('cashiers.resetPin')}</Button>
			<Button variant="outline" size="sm" onclick={() => onToggle(row)}>{row.status === 'active' ? t('common.disable') : t('common.enable')}</Button>
		{/snippet}
	</DataTable>
</PageContainer>

<Sheet.Root bind:open>
	<Sheet.Content class="flex flex-col gap-4">
		<Sheet.Header><Sheet.Title>{t('cashiers.createTitle')}</Sheet.Title></Sheet.Header>
		<div class="space-y-3 px-1">
			<div class="space-y-1"><Label>{t('cashiers.username')}</Label><Input bind:value={form.username} /></div>
			<div class="space-y-1"><Label>{t('cashiers.pin')}</Label><Input bind:value={form.pin} /></div>
			<div class="space-y-1"><Label>{t('cashiers.fullName')}</Label><Input bind:value={form.full_name} /></div>
			<div class="space-y-1">
				<Label>{t('cashiers.homeStore')}</Label>
				<select bind:value={form.store_id} class="h-9 w-full rounded-md border border-input bg-background px-3 text-sm">
					{#each stores as s}<option value={s.id}>{s.name}</option>{/each}
				</select>
			</div>
			<label class="flex items-center gap-2 text-sm">
				<Checkbox bind:checked={form.is_supervisor} /> {t('cashiers.supervisorHint')}
			</label>
		</div>
		<Sheet.Footer>
			<Button onclick={create} disabled={saving || !form.username || !form.pin || !form.store_id}>
				{saving ? t('common.saving') : t('common.save')}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

<Dialog.Root bind:open={pinOpen}>
	<Dialog.Content class="sm:max-w-sm">
		<Dialog.Header><Dialog.Title>{t('cashiers.resetPinTitle', { name: pinTarget?.username ?? '' })}</Dialog.Title></Dialog.Header>
		<div class="space-y-2 py-2">
			<Label>{t('cashiers.newPin')}</Label>
			<Input bind:value={newPin} type="password" placeholder="••••••" />
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => (pinOpen = false)}>{t('common.cancel')}</Button>
			<Button onclick={doResetPin} disabled={!newPin}>{t('cashiers.confirmReset')}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ConfirmDialog
	bind:open={toggleOpen}
	title={toggleDisabling ? t('cashiers.disableTitle') : t('cashiers.enableTitle')}
	description={toggleDisabling
		? t('cashiers.disableDesc', { name: toggleTarget?.username ?? '' })
		: t('cashiers.enableDesc', { name: toggleTarget?.username ?? '' })}
	confirmText={toggleDisabling ? t('common.disable') : t('common.enable')}
	cancelText={t('common.cancel')}
	variant={toggleDisabling ? 'destructive' : 'default'}
	onConfirm={doToggle}
/>

<!-- 管理员管理 — GET/POST /admins + 角色分配 + 停用/启用 + 重置密码。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Plus from '@lucide/svelte/icons/plus';
	import { PageContainer, PageHeader, DataTable, StatusBadge, ConfirmDialog, type Column, type BadgeTone } from '$lib/components/shared';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Sheet from '$lib/components/ui/sheet';
	import * as Dialog from '$lib/components/ui/dialog';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { auth } from '$lib/auth';
	import { t } from '$lib/i18n';

	interface Admin {
		id: number;
		username: string;
		full_name: string | null;
		email: string | null;
		role_id: number | null;
		status: string;
	}
	interface Role { id: number; code: string; name: string }

	const isSelf = (a: Admin) => String(a.id) === auth.user?.id;

	let rows = $state<Admin[]>([]);
	let roles = $state<Role[]>([]);
	let loading = $state(true);
	let open = $state(false);
	let saving = $state(false);
	let form = $state({ username: '', password: '', full_name: '', email: '', role_id: 0 });
	let pwOpen = $state(false);
	let pwTarget = $state<Admin | null>(null);
	let newPassword = $state('');
	let toggleOpen = $state(false);
	let toggleTarget = $state<Admin | null>(null);
	const toggleDisabling = $derived(toggleTarget?.status === 'active');

	const roleName = (id: number | null) => roles.find((r) => r.id === id)?.name ?? t('common.none');
	const columns = $derived<Column<Admin>[]>([
		{ key: 'username', header: t('admins.username'), sortable: true, searchable: true },
		{ key: 'full_name', header: t('admins.fullName'), searchable: true },
		{ key: 'email', header: t('admins.email') },
		{ key: 'role_id', header: t('admins.role'), render: (r) => roleName(r.role_id) },
		{ key: 'status', header: t('common.status') }
	]);
	const tone = (s: string): BadgeTone => (s === 'active' ? 'brand' : 'outline');

	async function load() {
		loading = true;
		try {
			[rows, roles] = await Promise.all([api.get<Admin[]>('/admins'), api.get<Role[]>('/roles')]);
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);

	function openCreate() {
		form = { username: '', password: '', full_name: '', email: '', role_id: roles[0]?.id ?? 0 };
		open = true;
	}
	async function create() {
		saving = true;
		try {
			await api.post('/admins', {
				username: form.username, password: form.password, full_name: form.full_name || null,
				email: form.email || null, role_id: form.role_id || null
			});
			toast.success(t('admins.created'));
			open = false;
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.save'));
		} finally {
			saving = false;
		}
	}
	async function setRole(a: Admin, role_id: number) {
		try {
			await api.patch(`/admins/${a.id}`, { role_id });
			toast.success(t('admins.roleChanged'));
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
		}
	}
	function onToggle(a: Admin) { toggleTarget = a; toggleOpen = true; }
	async function doToggle() {
		if (!toggleTarget) return;
		const disabling = toggleTarget.status === 'active';
		try {
			await api.post(`/admins/${toggleTarget.id}/${disabling ? 'disable' : 'enable'}`);
			toast.success(disabling ? t('admins.disabled') : t('admins.enabled'));
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
			throw e;
		}
	}
	function askResetPassword(a: Admin) { pwTarget = a; newPassword = ''; pwOpen = true; }
	async function doResetPassword() {
		if (!pwTarget || newPassword.length < 8) return;
		try {
			await api.post(`/admins/${pwTarget.id}/reset-password`, { new_password: newPassword });
			toast.success(t('admins.pwReset'));
			pwOpen = false;
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.op'));
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('admins.title')} description={t('admins.desc')}>
		{#snippet actions()}
			<Button onclick={openCreate}><Plus class="mr-1 size-4" />{t('admins.new')}</Button>
		{/snippet}
	</PageHeader>

	<DataTable data={rows} {columns} {loading} searchable emptyTitle={t('admins.empty')}>
		{#snippet cell(row, col)}
			{#if col.key === 'status'}
				<StatusBadge tone={tone(row.status)}>{row.status}</StatusBadge>
			{:else if col.key === 'role_id'}
				{#if isSelf(row)}
					<span class="text-sm text-muted-foreground">{roleName(row.role_id)}（{t('admins.self')}）</span>
				{:else}
					<select value={row.role_id ?? 0} onchange={(e) => setRole(row, Number(e.currentTarget.value))}
						class="h-8 rounded-md border border-input bg-background px-2 text-sm">
						<option value={0}>{t('common.none')}</option>
						{#each roles as r}<option value={r.id}>{r.name}</option>{/each}
					</select>
				{/if}
			{:else if col.render}
				{col.render(row)}
			{:else}
				{row[col.key as keyof Admin]}
			{/if}
		{/snippet}
		{#snippet actions(row)}
			{#if isSelf(row)}
				<span class="text-xs text-muted-foreground">{t('admins.selfHint')}</span>
			{:else}
				<Button variant="outline" size="sm" onclick={() => askResetPassword(row)}>{t('admins.resetPassword')}</Button>
				<Button variant="outline" size="sm" onclick={() => onToggle(row)}>{row.status === 'active' ? t('common.disable') : t('common.enable')}</Button>
			{/if}
		{/snippet}
	</DataTable>
</PageContainer>

<Sheet.Root bind:open>
	<Sheet.Content class="flex flex-col gap-4">
		<Sheet.Header><Sheet.Title>{t('admins.createTitle')}</Sheet.Title></Sheet.Header>
		<div class="space-y-3 px-1">
			<div class="space-y-1"><Label>{t('admins.username')}</Label><Input bind:value={form.username} /></div>
			<div class="space-y-1"><Label>{t('admins.initialPassword')}</Label><Input bind:value={form.password} /></div>
			<div class="space-y-1"><Label>{t('admins.fullName')}</Label><Input bind:value={form.full_name} /></div>
			<div class="space-y-1"><Label>{t('admins.email')}</Label><Input bind:value={form.email} /></div>
			<div class="space-y-1">
				<Label>{t('admins.role')}</Label>
				<select bind:value={form.role_id} class="h-9 w-full rounded-md border border-input bg-background px-3 text-sm">
					{#each roles as r}<option value={r.id}>{r.name}</option>{/each}
				</select>
			</div>
		</div>
		<Sheet.Footer>
			<Button onclick={create} disabled={saving || !form.username || form.password.length < 8}>
				{saving ? t('common.saving') : t('common.save')}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

<Dialog.Root bind:open={pwOpen}>
	<Dialog.Content class="sm:max-w-sm">
		<Dialog.Header><Dialog.Title>{t('admins.resetPwTitle', { name: pwTarget?.username ?? '' })}</Dialog.Title></Dialog.Header>
		<div class="space-y-2 py-2">
			<Label>{t('admins.newPassword')}</Label>
			<Input bind:value={newPassword} type="password" placeholder="••••••••" />
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => (pwOpen = false)}>{t('common.cancel')}</Button>
			<Button onclick={doResetPassword} disabled={newPassword.length < 8}>{t('admins.confirmReset')}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ConfirmDialog
	bind:open={toggleOpen}
	title={toggleDisabling ? t('admins.disableTitle') : t('admins.enableTitle')}
	description={toggleDisabling
		? t('admins.disableDesc', { name: toggleTarget?.username ?? '' })
		: t('admins.enableDesc', { name: toggleTarget?.username ?? '' })}
	confirmText={toggleDisabling ? t('common.disable') : t('common.enable')}
	cancelText={t('common.cancel')}
	variant={toggleDisabling ? 'destructive' : 'default'}
	onConfirm={doToggle}
/>

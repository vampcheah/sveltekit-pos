<!-- 权限管理 — 角色↔权限矩阵；PUT /roles/:id/permissions。 -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { PageContainer, PageHeader } from '$lib/components/shared';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { toast } from 'svelte-sonner';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface Role { id: number; code: string; name: string }
	interface Permission { id: number; code: string; name: string; group_: string }

	let roles = $state<Role[]>([]);
	let permissions = $state<Permission[]>([]);
	let groups = $state<string[]>([]);
	let activeRole = $state<Role | null>(null);
	let granted = $state<Set<string>>(new Set());
	let loading = $state(true);
	let saving = $state(false);

	onMount(async () => {
		try {
			[roles, permissions] = await Promise.all([
				api.get<Role[]>('/roles'),
				api.get<Permission[]>('/permissions')
			]);
			groups = [...new Set(permissions.map((p) => p.group_))];
			if (roles[0]) await selectRole(roles[0]);
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	});

	async function selectRole(r: Role) {
		activeRole = r;
		try {
			const codes = await api.get<string[]>(`/roles/${r.id}/permissions`);
			granted = new Set(codes);
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		}
	}
	function toggle(code: string, on: boolean) {
		const next = new Set(granted);
		if (on) next.add(code); else next.delete(code);
		granted = next;
	}
	async function save() {
		if (!activeRole) return;
		saving = true;
		try {
			await api.put(`/roles/${activeRole.id}/permissions`, { permissions: [...granted] });
			toast.success(t('roles.saved'));
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('roles.saveFailed'));
		} finally {
			saving = false;
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('roles.title')} description={t('roles.desc')} />

	<div class="grid gap-4 md:grid-cols-[200px_1fr]">
		<Card.Root>
			<Card.Header><Card.Title class="text-sm">{t('roles.roles')}</Card.Title></Card.Header>
			<Card.Content class="space-y-1">
				{#each roles as r}
					<button onclick={() => selectRole(r)}
						class="w-full rounded-md px-3 py-2 text-left text-sm {activeRole?.id === r.id ? 'bg-muted font-medium' : 'hover:bg-muted/50'}"
					>{r.name}</button>
				{/each}
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header class="flex-row items-center justify-between">
				<Card.Title class="text-sm">{t('roles.permissionsOf', { role: activeRole?.name ?? '' })}</Card.Title>
				<Button size="sm" onclick={save} disabled={saving || !activeRole}>{saving ? t('common.saving') : t('common.save')}</Button>
			</Card.Header>
			<Card.Content class="space-y-4">
				{#if loading}
					<p class="text-sm text-muted-foreground">{t('common.loading')}</p>
				{:else}
					{#each groups as g}
						<div>
							<p class="mb-2 text-xs font-medium uppercase text-muted-foreground">{g}</p>
							<div class="grid gap-2 sm:grid-cols-2">
								{#each permissions.filter((p) => p.group_ === g) as p}
									<label class="flex items-center gap-2 text-sm">
										<Checkbox checked={granted.has(p.code)} onCheckedChange={(v) => toggle(p.code, !!v)} />
										<span>{p.name}</span>
										<span class="font-mono text-xs text-muted-foreground">{p.code}</span>
									</label>
								{/each}
							</div>
						</div>
					{/each}
				{/if}
			</Card.Content>
		</Card.Root>
	</div>
</PageContainer>

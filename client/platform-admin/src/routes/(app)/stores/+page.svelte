<!-- 门店管理 — GET /stores + 新建。 -->
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

	interface Store {
		id: number;
		code: string;
		name: string;
		phone: string | null;
		timezone: string;
		status: string;
	}

	let rows = $state<Store[]>([]);
	let loading = $state(true);
	let open = $state(false);
	let saving = $state(false);
	let form = $state({ name: '', code: '', phone: '' });

	const columns = $derived<Column<Store>[]>([
		{ key: 'code', header: t('stores.code'), sortable: true, searchable: true },
		{ key: 'name', header: t('stores.name'), sortable: true, searchable: true },
		{ key: 'phone', header: t('stores.phone') },
		{ key: 'timezone', header: t('stores.timezone') },
		{ key: 'status', header: t('common.status') }
	]);

	async function load() {
		loading = true;
		try {
			rows = await api.get<Store[]>('/stores');
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.load'));
		} finally {
			loading = false;
		}
	}
	onMount(load);

	async function create() {
		saving = true;
		try {
			await api.post('/stores', { name: form.name, code: form.code, phone: form.phone || null });
			toast.success(t('stores.created'));
			open = false;
			form = { name: '', code: '', phone: '' };
			await load();
		} catch (e) {
			toast.error(e instanceof ApiError ? e.message : t('error.save'));
		} finally {
			saving = false;
		}
	}
</script>

<PageContainer>
	<PageHeader title={t('stores.title')} description={t('stores.desc')}>
		{#snippet actions()}
			<Button onclick={() => (open = true)}><Plus class="mr-1 size-4" />{t('stores.new')}</Button>
		{/snippet}
	</PageHeader>

	<DataTable data={rows} {columns} {loading} searchable emptyTitle={t('stores.empty')} />
</PageContainer>

<Sheet.Root bind:open>
	<Sheet.Content class="flex flex-col gap-4">
		<Sheet.Header><Sheet.Title>{t('stores.createTitle')}</Sheet.Title></Sheet.Header>
		<div class="space-y-3 px-1">
			<div class="space-y-1"><Label>{t('stores.code')}</Label><Input bind:value={form.code} placeholder="S002" /></div>
			<div class="space-y-1"><Label>{t('stores.name')}</Label><Input bind:value={form.name} placeholder="二店" /></div>
			<div class="space-y-1"><Label>{t('stores.phone')}</Label><Input bind:value={form.phone} /></div>
		</div>
		<Sheet.Footer>
			<Button onclick={create} disabled={saving || !form.code || !form.name}>
				{saving ? t('common.saving') : t('common.save')}
			</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

<!-- 个人资料 — 改头像(/auth/profile)、改姓名、改登录密码(/auth/change-password)。 -->
<script lang="ts">
	import Camera from '@lucide/svelte/icons/camera';
	import { PageContainer, PageHeader } from '$lib/components/shared';
	import * as Card from '$lib/components/ui/card';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toast } from 'svelte-sonner';
	import { auth } from '$lib/auth';
	import { initials } from '$lib/utils/formatters';
	import { api, ApiError } from '$lib/api';
	import { t } from '$lib/i18n';

	interface MeResp { username: string | null; full_name: string | null; avatar_url: string | null }

	let name = $state(auth.user?.name ?? '');
	let savingName = $state(false);
	let uploading = $state(false);
	let fileInput = $state<HTMLInputElement | null>(null);
	let currentPw = $state('');
	let newPw = $state('');
	let confirmPw = $state('');
	let changingPw = $state(false);

	const avatarUrl = $derived(auth.user?.avatarUrl);
	const avatarInitials = $derived(initials(name || auth.user?.email || 'U'));

	function applyMe(me: MeResp) {
		auth.setUser({
			...auth.user!,
			name: me.full_name || me.username || auth.user!.name,
			avatarUrl: me.avatar_url ?? undefined
		});
	}
	async function saveProfile(patch: { full_name?: string; avatar_url?: string }) {
		applyMe(await api.patch<MeResp>('/auth/profile', patch));
	}
	async function saveName(e: SubmitEvent) {
		e.preventDefault();
		savingName = true;
		try {
			await saveProfile({ full_name: name });
			toast.success(t('profile.saved'));
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : t('error.save'));
		} finally {
			savingName = false;
		}
	}
	function onPickFile(e: Event) {
		const file = (e.currentTarget as HTMLInputElement).files?.[0];
		if (!file) return;
		if (!file.type.startsWith('image/')) return toast.error(t('profile.notImage'));
		if (file.size > 500 * 1024) return toast.error(t('profile.avatarTooBig'));
		uploading = true;
		const reader = new FileReader();
		reader.onload = async () => {
			try {
				await saveProfile({ avatar_url: reader.result as string });
				toast.success(t('profile.avatarUpdated'));
			} catch (err) {
				toast.error(err instanceof ApiError ? err.message : t('error.op'));
			} finally {
				uploading = false;
			}
		};
		reader.onerror = () => { uploading = false; toast.error(t('error.op')); };
		reader.readAsDataURL(file);
	}
	async function removeAvatar() {
		uploading = true;
		try {
			await saveProfile({ avatar_url: '' });
			toast.success(t('profile.avatarRemoved'));
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : t('error.op'));
		} finally {
			uploading = false;
		}
	}
	async function changePassword(e: SubmitEvent) {
		e.preventDefault();
		if (newPw.length < 6) return toast.error(t('profile.passwordTooShort'));
		if (newPw !== confirmPw) return toast.error(t('profile.passwordMismatch'));
		changingPw = true;
		try {
			await api.post('/auth/change-password', { current_password: currentPw, new_password: newPw });
			toast.success(t('profile.passwordChanged'));
			currentPw = newPw = confirmPw = '';
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : t('error.op'));
		} finally {
			changingPw = false;
		}
	}
</script>

<svelte:head><title>{t('profile.title')} · POS</title></svelte:head>

<PageContainer>
	<PageHeader title={t('profile.title')} description={t('profile.desc')} />

	<div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
		<Card.Root class="lg:col-span-1">
			<Card.Header>
				<Card.Title>{t('profile.avatar')}</Card.Title>
				<Card.Description>{t('profile.avatarDesc')}</Card.Description>
			</Card.Header>
			<Card.Content class="flex flex-col items-center gap-4 text-center">
				<Avatar.Root class="size-24">
					{#if avatarUrl}<Avatar.Image src={avatarUrl} alt="" />{/if}
					<Avatar.Fallback class="text-xl">{avatarInitials}</Avatar.Fallback>
				</Avatar.Root>
				<input bind:this={fileInput} type="file" accept="image/*" class="hidden" onchange={onPickFile} />
				<div class="flex gap-2">
					<Button variant="outline" disabled={uploading} onclick={() => fileInput?.click()}>
						<Camera class="mr-1 size-4" />{uploading ? t('common.processing') : t('profile.changeAvatar')}
					</Button>
					{#if avatarUrl}
						<Button variant="ghost" disabled={uploading} onclick={removeAvatar}>{t('profile.removeAvatar')}</Button>
					{/if}
				</div>
				<p class="text-xs text-muted-foreground">{t('profile.avatarHint')}</p>
			</Card.Content>
		</Card.Root>

		<div class="space-y-6 lg:col-span-2">
			<Card.Root>
				<Card.Header><Card.Title>{t('profile.basicInfo')}</Card.Title></Card.Header>
				<Card.Content>
					<form class="space-y-4" onsubmit={saveName}>
						<div class="space-y-1"><Label for="name">{t('profile.displayName')}</Label><Input id="name" bind:value={name} /></div>
						<div class="space-y-1"><Label>{t('profile.loginAccount')}</Label><Input value={auth.user?.email ?? ''} disabled /></div>
						<Button type="submit" disabled={savingName}>{savingName ? t('common.saving') : t('common.save')}</Button>
					</form>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header>
					<Card.Title>{t('profile.changePassword')}</Card.Title>
					<Card.Description>{t('profile.changePasswordDesc')}</Card.Description>
				</Card.Header>
				<Card.Content>
					<form class="space-y-4" onsubmit={changePassword}>
						<div class="space-y-1"><Label for="cur">{t('profile.currentPassword')}</Label><Input id="cur" type="password" bind:value={currentPw} autocomplete="current-password" /></div>
						<div class="space-y-1"><Label for="new">{t('profile.newPassword')}</Label><Input id="new" type="password" bind:value={newPw} autocomplete="new-password" /></div>
						<div class="space-y-1"><Label for="confirm">{t('profile.confirmPassword')}</Label><Input id="confirm" type="password" bind:value={confirmPw} autocomplete="new-password" /></div>
						<Button type="submit" disabled={changingPw || !currentPw || !newPw}>{changingPw ? t('profile.submitting') : t('profile.submit')}</Button>
					</form>
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</PageContainer>

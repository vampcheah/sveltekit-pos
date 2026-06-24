<!--
  (app) route-group layout — mock auth guard + admin shell.
  Restores the mock session on the client; redirects to /login when not
  authenticated, otherwise renders the children inside the AppShell.
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { auth } from '$lib/auth';
	import { AppShell, logoutDialog } from '$lib/shell';
	import { ConfirmDialog } from '$lib/components/shared';
	import { config } from '$lib/config';
	import { t } from '$lib/i18n';

	let { children } = $props();
	let ready = $state(false);

	onMount(() => {
		auth.init();
		if (!auth.isAuthenticated) {
			goto(resolve(config.auth.afterLogout));
			return;
		}
		ready = true;
		// 后台用真实会话 cookie 校验；失效则登出回登录页
		void auth.revalidate().then(() => {
			if (!auth.isAuthenticated) goto(resolve(config.auth.afterLogout));
		});
	});
</script>

{#if ready}
	<AppShell>{@render children()}</AppShell>
	<ConfirmDialog
		bind:open={logoutDialog.open}
		title={t('common.logoutConfirmTitle')}
		description={t('common.logoutConfirmDescription')}
		confirmText={t('common.logout')}
		cancelText={t('common.cancel')}
		variant="destructive"
		onConfirm={() => auth.logout()}
	/>
{/if}

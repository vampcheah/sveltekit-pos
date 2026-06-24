<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { login, restore, cashier } from '$lib/session.svelte';
	import * as m from '$lib/paraglide/messages.js';

	let username = $state('cashier1');
	let pin = $state('');
	let submitting = $state(false);

	onMount(async () => {
		// 已有有效 cookie 会话则直接进收银台
		if (await restore()) goto('/pos/cashier');
	});

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		submitting = true;
		const res = await login(username, pin);
		submitting = false;
		if (res.ok) {
			toast.success(m.pos_login_success());
			goto('/pos/cashier');
		} else {
			toast.error(res.error ?? m.pos_login_failed());
		}
	}
</script>

<div class="flex h-screen flex-col items-center justify-center gap-6">
	<h1 class="text-2xl font-semibold">{m.pos_login_title()}</h1>
	<form class="flex w-full max-w-sm flex-col gap-4 p-4" onsubmit={handleLogin}>
		<div class="space-y-2">
			<Label for="username">{m.pos_login_username()}</Label>
			<Input id="username" bind:value={username} placeholder="cashier1" autocomplete="username" />
		</div>
		<div class="space-y-2">
			<Label for="pin">{m.pos_login_pin()}</Label>
			<Input id="pin" type="password" bind:value={pin} placeholder="••••••" autocomplete="current-password" />
		</div>
		<Button type="submit" class="w-full" disabled={submitting}>
			{submitting ? m.pos_login_submitting() : m.pos_login_submit()}
		</Button>
		<p class="text-center text-xs text-muted-foreground">{m.pos_login_demo()}</p>
		{#if cashier.loggedIn}
			<p class="text-center text-xs text-green-600">
				{m.pos_login_store_info({ store: cashier.storeId ?? '', warehouse: cashier.warehouseId ?? '' })}
			</p>
		{/if}
	</form>
</div>

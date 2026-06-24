<!-- Sticky top bar inside the inset: trigger, breadcrumbs, search, notifications, toggles, user menu. -->
<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import { Button } from '$lib/components/ui/button';
	import { ThemeToggle, LanguageToggle, CommandMenu } from '$lib/components/shared';
	import { resolve } from '$app/paths';
	import { auth } from '$lib/auth';
	import { initials } from '$lib/utils/formatters';
	import { t } from '$lib/i18n';
	import { logoutDialog } from './logout-dialog.svelte';
	import Breadcrumbs from './Breadcrumbs.svelte';
	import NotificationsMenu from './NotificationsMenu.svelte';
	import Search from '@lucide/svelte/icons/search';
	import UserIcon from '@lucide/svelte/icons/user';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import LogOut from '@lucide/svelte/icons/log-out';

	let commandOpen = $state(false);

	const user = $derived(auth.user);
</script>

<header
	class="bg-background/95 supports-[backdrop-filter]:bg-background/60 sticky top-0 z-20 flex h-14 shrink-0 items-center gap-2 border-b px-4 backdrop-blur"
>
	<Sidebar.Trigger class="-ml-1" />
	<Separator orientation="vertical" class="mr-1 data-[orientation=vertical]:h-4" />
	<Breadcrumbs />

	<div class="ml-auto flex items-center gap-1 sm:gap-2">
		<!-- Search: opens the command palette -->
		<Button
			variant="outline"
			size="sm"
			class="text-muted-foreground hidden h-8 w-48 justify-start gap-2 px-2.5 sm:flex lg:w-64"
			onclick={() => (commandOpen = true)}
		>
			<Search class="size-4" />
			<span class="flex-1 text-left">{t('common.search')}</span>
			<kbd
				class="bg-muted text-muted-foreground pointer-events-none hidden h-5 items-center gap-0.5 rounded border px-1.5 text-[10px] font-medium sm:inline-flex"
			>
				<span class="text-xs">⌘</span>K
			</kbd>
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="sm:hidden"
			aria-label={t('common.search')}
			onclick={() => (commandOpen = true)}
		>
			<Search />
		</Button>

		<NotificationsMenu />
		<LanguageToggle />
		<ThemeToggle />

		<Separator
			orientation="vertical"
			class="mx-1 hidden data-[orientation=vertical]:h-4 sm:block"
		/>

		<!-- User menu -->
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" size="icon" class="rounded-full" aria-label="Account">
						<Avatar.Root class="size-8">
							{#if user?.avatarUrl}
								<Avatar.Image src={user.avatarUrl} alt={user.name} />
							{/if}
							<Avatar.Fallback>{user ? initials(user.name) : '?'}</Avatar.Fallback>
						</Avatar.Root>
					</Button>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end" class="min-w-56">
				<DropdownMenu.Label class="p-0 font-normal">
					<div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
						<Avatar.Root class="size-8">
							{#if user?.avatarUrl}
								<Avatar.Image src={user.avatarUrl} alt={user.name} />
							{/if}
							<Avatar.Fallback>{user ? initials(user.name) : '?'}</Avatar.Fallback>
						</Avatar.Root>
						<div class="grid flex-1 leading-tight">
							<span class="truncate font-semibold">{user?.name ?? 'Guest'}</span>
							<span class="text-muted-foreground truncate text-xs">{user?.email ?? ''}</span>
						</div>
					</div>
				</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a href={resolve('/profile')} {...props}>
							<UserIcon />
							{t('common.profile')}
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a href={resolve('/settings')} {...props}>
							<SettingsIcon />
							{t('common.settings')}
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Separator />
				<DropdownMenu.Item variant="destructive" onSelect={() => (logoutDialog.open = true)}>
					<LogOut />
					{t('common.logout')}
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>

<CommandMenu bind:open={commandOpen} />

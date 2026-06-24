<!-- Admin sidebar: brand header, grouped nav with active highlighting, user footer dropdown. -->
<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { auth } from '$lib/auth';
	import { initials } from '$lib/utils/formatters';
	import { visibleNavGroups } from './nav';
	import { t } from '$lib/i18n';
	import { logoutDialog } from './logout-dialog.svelte';
	import { config } from '$lib/config';
	import ChevronsUpDown from '@lucide/svelte/icons/chevrons-up-down';
	import UserIcon from '@lucide/svelte/icons/user';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import LogOut from '@lucide/svelte/icons/log-out';

	const user = $derived(auth.user);
	const pathname = $derived(page.url.pathname);
	// 按当前用户权限过滤菜单（无权限的项不展示）
	const groups = $derived(visibleNavGroups((p) => auth.has(p)));

	function isActive(href: string): boolean {
		return pathname === href || pathname.startsWith(href + '/');
	}
</script>

<Sidebar.Root collapsible="icon">
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<a href={resolve('/dashboard')} {...props}>
							<div
								class="bg-primary text-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg text-base font-semibold"
							>
								{config.app.logo}
							</div>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-semibold">{config.app.name}</span>
								<span class="text-muted-foreground truncate text-xs">Dashboard</span>
							</div>
						</a>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>

	<Sidebar.Content>
		{#each groups as group (group.labelKey)}
			<Sidebar.Group>
				<Sidebar.GroupLabel>{t(group.labelKey)}</Sidebar.GroupLabel>
				<Sidebar.Menu>
					{#each group.items as item (item.href)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton
								isActive={isActive(item.href)}
								tooltipContent={t(item.titleKey)}
								class="relative transition-colors data-active:text-sidebar-primary data-active:before:absolute data-active:before:inset-y-1.5 data-active:before:left-0 data-active:before:w-0.5 data-active:before:rounded-full data-active:before:bg-sidebar-primary data-active:before:content-['']"
							>
								{#snippet child({ props })}
									<a href={resolve(item.href)} {...props}>
										<item.icon />
										<span>{t(item.titleKey)}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
							{#if item.badge != null}
								<Sidebar.MenuBadge>{item.badge}</Sidebar.MenuBadge>
							{/if}
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.Group>
		{/each}
	</Sidebar.Content>

	<Sidebar.Footer>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Sidebar.MenuButton
								size="lg"
								class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
								{...props}
							>
								<Avatar.Root class="size-8 rounded-lg">
									{#if user?.avatarUrl}
										<Avatar.Image src={user.avatarUrl} alt={user.name} />
									{/if}
									<Avatar.Fallback class="rounded-lg">
										{user ? initials(user.name) : '?'}
									</Avatar.Fallback>
								</Avatar.Root>
								<div class="grid flex-1 text-left text-sm leading-tight">
									<span class="truncate font-semibold">{user?.name ?? 'Guest'}</span>
									<span class="text-muted-foreground truncate text-xs capitalize">
										{user?.role ?? ''}
									</span>
								</div>
								<ChevronsUpDown class="ml-auto size-4" />
							</Sidebar.MenuButton>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content
						class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
						side="right"
						align="end"
						sideOffset={4}
					>
						<DropdownMenu.Label class="p-0 font-normal">
							<div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
								<Avatar.Root class="size-8 rounded-lg">
									{#if user?.avatarUrl}
										<Avatar.Image src={user.avatarUrl} alt={user.name} />
									{/if}
									<Avatar.Fallback class="rounded-lg">
										{user ? initials(user.name) : '?'}
									</Avatar.Fallback>
								</Avatar.Root>
								<div class="grid flex-1 text-left text-sm leading-tight">
									<span class="truncate font-semibold">{user?.name ?? 'Guest'}</span>
									<span class="text-muted-foreground truncate text-xs">{user?.email ?? ''}</span>
								</div>
							</div>
						</DropdownMenu.Label>
						<DropdownMenu.Separator />
						<DropdownMenu.Group>
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a href={resolve('/profile')} {...props}>
										<UserIcon />
										Profile
									</a>
								{/snippet}
							</DropdownMenu.Item>
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a href={resolve('/settings')} {...props}>
										<SettingsIcon />
										Settings
									</a>
								{/snippet}
							</DropdownMenu.Item>
						</DropdownMenu.Group>
						<DropdownMenu.Separator />
						<DropdownMenu.Item variant="destructive" onSelect={() => (logoutDialog.open = true)}>
							<LogOut />
							Logout
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>

	<Sidebar.Rail />
</Sidebar.Root>

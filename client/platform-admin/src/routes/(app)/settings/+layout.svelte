<!--
  Settings layout. Wraps every settings sub-page in a shared PageContainer +
  PageHeader and a responsive secondary nav (vertical list on sm+, horizontally
  scrollable tabs on mobile). The active link is derived from the current path.
-->
<script lang="ts">
	import type { Snippet } from 'svelte';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import type { Pathname } from '$app/types';
	import { PageContainer, PageHeader } from '$lib/components/shared';
	import { cn } from '$lib/utils';
	import SlidersHorizontal from '@lucide/svelte/icons/sliders-horizontal';
	import Palette from '@lucide/svelte/icons/palette';
	import Bell from '@lucide/svelte/icons/bell';
	import type { Component } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	interface SettingsNavItem {
		title: string;
		href: Pathname;
		icon: Component;
	}

	const navItems: SettingsNavItem[] = [
		{ title: 'General', href: '/settings', icon: SlidersHorizontal },
		{ title: 'Appearance', href: '/settings/appearance', icon: Palette },
		{ title: 'Notifications', href: '/settings/notifications', icon: Bell }
	];

	// Exact match for the index route, prefix match for nested ones so that the
	// correct tab stays highlighted on every sub-page.
	function isActive(href: string): boolean {
		const path = page.url.pathname;
		if (href === '/settings') return path === href;
		return path === href || path.startsWith(`${href}/`);
	}
</script>

<PageContainer>
	<PageHeader
		title="Settings"
		description="Manage your organization, appearance, and notification preferences."
	/>

	<div class="flex flex-col gap-6 lg:flex-row lg:gap-10">
		<!-- Secondary nav: horizontal scroll on mobile, vertical list from sm up. -->
		<nav
			aria-label="Settings"
			class="-mx-1 flex shrink-0 gap-1 overflow-x-auto px-1 pb-1 lg:mx-0 lg:w-52 lg:flex-col lg:overflow-visible lg:px-0 lg:pb-0"
		>
			{#each navItems as item (item.href)}
				{@const active = isActive(item.href)}
				<a
					href={resolve(item.href)}
					aria-current={active ? 'page' : undefined}
					class={cn(
						'inline-flex shrink-0 items-center gap-2 rounded-md px-3 py-2 text-sm font-medium whitespace-nowrap transition-colors',
						'hover:bg-accent hover:text-accent-foreground',
						active ? 'bg-accent text-accent-foreground' : 'text-muted-foreground'
					)}
				>
					<item.icon class="size-4" />
					{item.title}
				</a>
			{/each}
		</nav>

		<div class="min-w-0 flex-1 space-y-6">
			{@render children()}
		</div>
	</div>
</PageContainer>

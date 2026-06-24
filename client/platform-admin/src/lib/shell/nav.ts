// 导航模型：POS 平台分组侧栏 + 查找助手。
import type { Component } from 'svelte';
import type { Pathname } from '$app/types';
import LayoutDashboard from '@lucide/svelte/icons/layout-dashboard';
import Package from '@lucide/svelte/icons/package';
import Users from '@lucide/svelte/icons/users';
import Store from '@lucide/svelte/icons/store';
import UserCog from '@lucide/svelte/icons/user-cog';
import ShoppingBag from '@lucide/svelte/icons/shopping-bag';
import ChartLine from '@lucide/svelte/icons/chart-line';
import Settings from '@lucide/svelte/icons/settings';
import Shield from '@lucide/svelte/icons/shield';
import KeyRound from '@lucide/svelte/icons/key-round';
import User from '@lucide/svelte/icons/user';

export interface NavItem {
	/** i18n 键，如 'nav.products'，渲染时经 t() 翻译。 */
	titleKey: string;
	href: Pathname;
	icon: Component;
	badge?: string | number;
	/** 所需权限码；未设=所有登录用户可见。 */
	permission?: string;
}

export interface NavGroup {
	labelKey: string;
	items: NavItem[];
}

export const navGroups: NavGroup[] = [
	{
		labelKey: 'nav.groupOverview',
		items: [{ titleKey: 'nav.dashboard', href: '/dashboard', icon: LayoutDashboard, permission: 'reports.read' }]
	},
	{
		labelKey: 'nav.groupCommerce',
		items: [
			{ titleKey: 'nav.products', href: '/products', icon: Package, permission: 'products.write' },
			{ titleKey: 'nav.orders', href: '/orders', icon: ShoppingBag, permission: 'orders.refund' },
			{ titleKey: 'nav.members', href: '/members', icon: Users, permission: 'members.write' },
			{ titleKey: 'nav.reports', href: '/reports', icon: ChartLine, permission: 'reports.read' }
		]
	},
	{
		labelKey: 'nav.groupOrg',
		items: [
			{ titleKey: 'nav.stores', href: '/stores', icon: Store, permission: 'stores.write' },
			{ titleKey: 'nav.cashiers', href: '/cashiers', icon: UserCog, permission: 'cashiers.write' }
		]
	},
	{
		labelKey: 'nav.groupSystem',
		items: [
			{ titleKey: 'nav.admins', href: '/admins', icon: Shield, permission: 'admins.write' },
			{ titleKey: 'nav.roles', href: '/roles', icon: KeyRound, permission: 'roles.write' },
			{ titleKey: 'nav.settings', href: '/settings', icon: Settings, permission: 'settings.write' }
		]
	},
	{
		labelKey: 'nav.groupAccount',
		items: [{ titleKey: 'nav.profile', href: '/profile', icon: User }] // 始终可见
	}
];

/** 按权限过滤可见菜单：无 permission 的项始终显示；过滤后空组自动隐藏。 */
export function visibleNavGroups(has: (permission: string) => boolean): NavGroup[] {
	return navGroups
		.map((g) => ({ ...g, items: g.items.filter((i) => !i.permission || has(i.permission)) }))
		.filter((g) => g.items.length > 0);
}

/**
 * Find the nav item whose `href` is the longest prefix of `pathname`.
 */
export function findNavItem(pathname: string): { group: NavGroup; item: NavItem } | undefined {
	let best: { group: NavGroup; item: NavItem } | undefined;
	for (const group of navGroups) {
		for (const item of group.items) {
			const isMatch = pathname === item.href || pathname.startsWith(item.href + '/');
			if (!isMatch) continue;
			if (!best || item.href.length > best.item.href.length) {
				best = { group, item };
			}
		}
	}
	return best;
}

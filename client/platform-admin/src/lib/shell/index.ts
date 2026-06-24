// Public barrel for the admin shell.
export { default as AppShell } from './AppShell.svelte';
export { default as AppSidebar } from './AppSidebar.svelte';
export { default as AppHeader } from './AppHeader.svelte';
export { default as Breadcrumbs } from './Breadcrumbs.svelte';
export { default as NotificationsMenu } from './NotificationsMenu.svelte';
export { navGroups, findNavItem } from './nav';
export type { NavItem, NavGroup } from './nav';
export { logoutDialog } from './logout-dialog.svelte';

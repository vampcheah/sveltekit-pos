<!-- Bell button with unread badge and a dropdown listing app notifications. -->
<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { cn } from '$lib/utils';
	import { notifications, type AppNotification } from '$lib/stores/notifications.svelte';
	import Bell from '@lucide/svelte/icons/bell';
	import Info from '@lucide/svelte/icons/info';
	import CircleCheck from '@lucide/svelte/icons/circle-check';
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
	import CheckCheck from '@lucide/svelte/icons/check-check';

	const items = $derived(notifications.items);
	const unread = $derived(notifications.unread);

	const typeIcon = {
		info: Info,
		success: CircleCheck,
		warning: TriangleAlert
	} as const;

	const typeColor = {
		info: 'text-sky-500',
		success: 'text-emerald-500',
		warning: 'text-amber-500'
	} as const;

	function iconFor(n: AppNotification) {
		return typeIcon[n.type];
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon" class="relative" aria-label="Notifications">
				<Bell />
				{#if unread > 0}
					<Badge
						class="absolute -top-1 -right-1 size-4 min-w-4 justify-center rounded-full p-0 text-[10px] tabular-nums"
					>
						{unread > 9 ? '9+' : unread}
					</Badge>
				{/if}
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="w-80 p-0">
		<div class="flex items-center justify-between px-3 py-2">
			<span class="text-sm font-semibold">Notifications</span>
			{#if unread > 0}
				<button
					type="button"
					class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 text-xs transition-colors"
					onclick={() => notifications.markAllRead()}
				>
					<CheckCheck class="size-3.5" />
					Mark all read
				</button>
			{/if}
		</div>
		<DropdownMenu.Separator class="my-0" />
		{#if items.length === 0}
			<div class="text-muted-foreground px-3 py-8 text-center text-sm">You're all caught up</div>
		{:else}
			<div class="max-h-80 overflow-y-auto">
				{#each items as n (n.id)}
					{@const Icon = iconFor(n)}
					<button
						type="button"
						class="hover:bg-accent flex w-full items-start gap-3 px-3 py-2.5 text-left transition-colors"
						onclick={() => notifications.markRead(n.id)}
					>
						<Icon class={cn('mt-0.5 size-4 shrink-0', typeColor[n.type])} />
						<div class="grid flex-1 gap-0.5">
							<span class="text-sm leading-tight font-medium">{n.title}</span>
							<span class="text-muted-foreground line-clamp-2 text-xs">{n.body}</span>
							<span class="text-muted-foreground text-[11px]">{n.time}</span>
						</div>
						{#if !n.read}
							<span class="bg-primary mt-1.5 size-2 shrink-0 rounded-full"></span>
						{/if}
					</button>
				{/each}
			</div>
		{/if}
	</DropdownMenu.Content>
</DropdownMenu.Root>

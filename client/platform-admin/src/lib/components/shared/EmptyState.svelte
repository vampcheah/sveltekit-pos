<!--
  EmptyState — centered placeholder shown when a list/table/area has no
  content. Optional icon, title, description and an action slot (e.g. a
  "Create" button).
-->
<script lang="ts">
	import type { Component, Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	interface Props {
		icon?: Component;
		title: string;
		description?: string;
		action?: Snippet;
		class?: string;
	}

	let { icon: Icon, title, description, action, class: className }: Props = $props();
</script>

<div
	class={cn(
		'flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-border px-6 py-12 text-center',
		className
	)}
>
	{#if Icon}
		<div
			class="flex size-11 items-center justify-center rounded-full bg-muted text-muted-foreground"
		>
			<Icon class="size-5" aria-hidden="true" />
		</div>
	{/if}

	<div class="space-y-1">
		<p class="text-sm font-medium text-foreground">{title}</p>
		{#if description}
			<p class="mx-auto max-w-sm text-sm text-muted-foreground">{description}</p>
		{/if}
	</div>

	{#if action}
		<div class="mt-1">
			{@render action()}
		</div>
	{/if}
</div>

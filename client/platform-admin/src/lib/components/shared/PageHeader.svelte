<!--
  PageHeader — page title block with optional description and right-aligned
  action slot (buttons, menus, etc). Stacks on small screens.
-->
<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	interface Props {
		title: string;
		description?: string;
		actions?: Snippet;
		class?: string;
	}

	let { title, description, actions, class: className }: Props = $props();
</script>

<div class={cn('flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between', className)}>
	<div class="space-y-1">
		<h1 class="text-2xl font-semibold tracking-tight text-foreground">
			{title}
		</h1>
		{#if description}
			<p class="text-sm text-muted-foreground">{description}</p>
		{/if}
	</div>

	{#if actions}
		<div class="flex flex-shrink-0 items-center gap-2">
			{@render actions()}
		</div>
	{/if}
</div>

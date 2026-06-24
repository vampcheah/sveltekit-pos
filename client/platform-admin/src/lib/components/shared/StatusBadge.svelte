<!--
  StatusBadge — a consistently tinted status/role pill. Pick a semantic `tone`
  and the soft tint + matching text color are resolved here, so every table and
  page renders the same conventions from a single source of truth. Built on the
  base Badge primitive.
-->
<script lang="ts" module>
	export type BadgeTone =
		| 'neutral'
		| 'outline'
		| 'brand'
		| 'success'
		| 'warning'
		| 'danger'
		| 'info';
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { cn } from '$lib/utils';

	interface Props {
		tone?: BadgeTone;
		class?: string;
		children: Snippet;
	}

	let { tone = 'neutral', class: className, children }: Props = $props();

	const TONE: Record<BadgeTone, string> = {
		neutral: 'border-transparent bg-muted text-foreground',
		outline: 'border-border text-muted-foreground',
		brand: 'border-transparent bg-primary/10 text-primary',
		success: 'border-transparent bg-emerald-500/15 text-emerald-600 dark:text-emerald-400',
		warning: 'border-transparent bg-amber-500/15 text-amber-600 dark:text-amber-400',
		danger: 'border-transparent bg-red-500/15 text-red-600 dark:text-red-400',
		info: 'border-transparent bg-blue-500/15 text-blue-600 dark:text-blue-400'
	};
</script>

<Badge variant="outline" class={cn(TONE[tone], className)}>
	{@render children()}
</Badge>

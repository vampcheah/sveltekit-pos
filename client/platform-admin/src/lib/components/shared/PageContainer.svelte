<!--
  PageContainer — the standard centered content wrapper for every page.
  Provides consistent max-width, responsive padding and vertical rhythm.
-->
<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	interface Props {
		children: Snippet;
		class?: string;
	}

	let { children, class: className }: Props = $props();
</script>

<div class={cn('mx-auto w-full max-w-7xl p-4 sm:p-6 space-y-6', className)}>
	{@render children()}
</div>

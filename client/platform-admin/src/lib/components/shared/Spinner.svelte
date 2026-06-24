<!--
  Spinner — a minimal, accessible loading indicator.
  Wraps the Lucide `loader-2` icon with `animate-spin`. Size/color is
  controlled via the merged `class` (e.g. `size-6 text-primary`).
-->
<script lang="ts">
	// `loader-circle` is the canonical Lucide icon that the deprecated `loader-2`
	// alias points to (the alias ships no `.svelte.d.ts`, so we import the real
	// icon for clean type resolution).
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import { cn } from '$lib/utils';

	interface Props {
		class?: string;
	}

	let { class: className }: Props = $props();
</script>

<LoaderCircle
	class={cn('size-4 animate-spin text-muted-foreground', className)}
	aria-hidden="true"
/>
<span class="sr-only">Loading</span>

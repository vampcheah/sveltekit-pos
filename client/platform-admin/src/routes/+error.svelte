<!--
  Top-level error boundary — handles 404s (unmatched routes) and any uncaught
  load/render error that escapes the app shell. Status-aware copy + recovery
  actions, rendered on a clean branded canvas (no sidebar).
-->
<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import House from '@lucide/svelte/icons/house';
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';

	const isNotFound = $derived(page.status === 404);

	const headline = $derived(
		page.status === 404
			? 'Page not found'
			: page.status === 403
				? 'Access denied'
				: page.status >= 500
					? 'Something went wrong'
					: 'Unexpected error'
	);

	const description = $derived(
		page.status === 404
			? "The page you're looking for doesn't exist or may have been moved."
			: page.status === 403
				? "You don't have permission to view this page."
				: (page.error?.message ?? 'An unexpected error occurred. Please try again.')
	);

	function goBack() {
		if (typeof history !== 'undefined') history.back();
	}
	function reload() {
		if (typeof location !== 'undefined') location.reload();
	}
</script>

<svelte:head>
	<title>{page.status} · Admin Starter</title>
</svelte:head>

<div
	class="bg-background relative flex min-h-svh flex-col items-center justify-center overflow-hidden px-6 text-center"
>
	<!-- Decorative glow -->
	<div
		class="bg-primary/10 pointer-events-none absolute -top-24 left-1/2 size-96 -translate-x-1/2 rounded-full blur-3xl"
		aria-hidden="true"
	></div>

	<div class="relative flex items-center gap-2">
		<div
			class="bg-primary text-primary-foreground flex size-7 items-center justify-center rounded-md text-sm font-bold"
		>
			A
		</div>
		<span class="text-sm font-semibold tracking-tight">Admin Starter</span>
	</div>

	<p
		class="text-primary relative mt-8 text-7xl font-bold tracking-tighter tabular-nums sm:text-8xl"
	>
		{page.status}
	</p>

	<div class="relative mt-2 space-y-2">
		<h1 class="text-2xl font-semibold tracking-tight">{headline}</h1>
		<p class="text-muted-foreground mx-auto max-w-md text-sm">{description}</p>
	</div>

	<div class="relative mt-6 flex flex-wrap items-center justify-center gap-2">
		<Button href="/dashboard">
			<House class="size-4" aria-hidden="true" />
			Go to dashboard
		</Button>
		{#if isNotFound}
			<Button variant="outline" onclick={goBack}>
				<ArrowLeft class="size-4" aria-hidden="true" />
				Go back
			</Button>
		{:else}
			<Button variant="outline" onclick={reload}>
				<RotateCcw class="size-4" aria-hidden="true" />
				Try again
			</Button>
		{/if}
	</div>
</div>

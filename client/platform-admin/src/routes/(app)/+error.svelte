<!--
  In-app error boundary — renders inside the admin shell so the sidebar, header
  and navigation stay available when a route under (app) throws. Status-aware
  copy with recovery actions.
-->
<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { PageContainer } from '$lib/components/shared';
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import House from '@lucide/svelte/icons/house';

	const headline = $derived(
		page.status === 404
			? 'Page not found'
			: page.status >= 500
				? 'Something went wrong'
				: 'Unexpected error'
	);

	const description = $derived(
		page.status === 404
			? "This page doesn't exist or may have been moved."
			: (page.error?.message ?? 'An unexpected error occurred while loading this page.')
	);

	function reload() {
		if (typeof location !== 'undefined') location.reload();
	}
</script>

<svelte:head>
	<title>{page.status} · Admin Starter</title>
</svelte:head>

<PageContainer>
	<div class="flex min-h-[60vh] flex-col items-center justify-center gap-5 text-center">
		<div
			class="bg-destructive/10 text-destructive flex size-14 items-center justify-center rounded-2xl"
		>
			<TriangleAlert class="size-7" aria-hidden="true" />
		</div>

		<div class="space-y-1.5">
			<p class="text-muted-foreground text-sm font-medium tabular-nums">Error {page.status}</p>
			<h1 class="text-2xl font-semibold tracking-tight">{headline}</h1>
			<p class="text-muted-foreground mx-auto max-w-md text-sm">{description}</p>
		</div>

		<div class="flex flex-wrap items-center justify-center gap-2">
			<Button onclick={reload}>
				<RotateCcw class="size-4" aria-hidden="true" />
				Try again
			</Button>
			<Button href="/dashboard" variant="outline">
				<House class="size-4" aria-hidden="true" />
				Back to dashboard
			</Button>
		</div>
	</div>
</PageContainer>

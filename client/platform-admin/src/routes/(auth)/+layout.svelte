<!--
  Auth layout — a split-screen canvas. On large screens an indigo brand panel
  (headline + product highlights) sits beside the form; on small screens it
  collapses to a centered form with a compact brand mark. Already-authenticated
  visitors are bounced to the dashboard.
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { auth } from '$lib/auth';
	import { config } from '$lib/config';
	import { ThemeToggle, LanguageToggle } from '$lib/components/shared';
	import Check from '@lucide/svelte/icons/check';

	let { children } = $props();

	const highlights = [
		'Svelte 5 runes + SvelteKit 2',
		'Tailwind v4 + shadcn-svelte UI',
		'Dark mode, i18n & ⌘K command palette',
		'Responsive, accessible and fully typed'
	];

	onMount(() => {
		auth.init();
		if (auth.isAuthenticated) goto(resolve(config.auth.afterLogin));
	});
</script>

<div class="bg-background relative min-h-svh lg:grid lg:grid-cols-2">
	<!-- Brand panel — large screens only -->
	<aside
		class="bg-primary text-primary-foreground relative hidden overflow-hidden p-10 lg:flex lg:flex-col lg:justify-between xl:p-14"
	>
		<!-- Decorative depth: soft glows + a subtle sheen, all token-driven. -->
		<div class="pointer-events-none absolute inset-0" aria-hidden="true">
			<div
				class="bg-primary-foreground/10 absolute -top-24 -left-24 size-80 rounded-full blur-3xl"
			></div>
			<div
				class="bg-primary-foreground/10 absolute right-0 -bottom-32 size-96 rounded-full blur-3xl"
			></div>
			<div
				class="from-primary-foreground/10 absolute inset-0 bg-gradient-to-br to-transparent"
			></div>
		</div>

		<a href={resolve('/')} class="relative flex items-center gap-2.5">
			<div
				class="bg-primary-foreground/15 flex size-9 items-center justify-center rounded-lg text-base font-bold backdrop-blur"
			>
				{config.app.logo}
			</div>
			<span class="text-lg font-semibold tracking-tight">{config.app.name}</span>
		</a>

		<div class="relative max-w-md space-y-8">
			<div class="space-y-3">
				<h1 class="text-3xl font-semibold tracking-tight text-balance xl:text-4xl">
					{config.app.tagline}
				</h1>
				<p class="text-primary-foreground/80 text-base leading-relaxed">
					A polished SvelteKit starter — auth, dashboard, tables, forms and charts, fully wired and
					ready to extend.
				</p>
			</div>
			<ul class="space-y-3">
				{#each highlights as item (item)}
					<li class="flex items-center gap-3">
						<span
							class="bg-primary-foreground/15 flex size-5 shrink-0 items-center justify-center rounded-full"
						>
							<Check class="size-3.5" aria-hidden="true" />
						</span>
						<span class="text-primary-foreground/90 text-sm">{item}</span>
					</li>
				{/each}
			</ul>
		</div>

		<p class="text-primary-foreground/60 relative text-xs">
			Svelte 5 · Tailwind v4 · shadcn-svelte
		</p>
	</aside>

	<!-- Form side -->
	<main class="relative flex min-h-svh flex-col items-center justify-center p-4 sm:p-6 lg:min-h-0">
		<!-- Corner controls -->
		<div class="absolute top-4 right-4 z-10 flex items-center gap-1">
			<LanguageToggle />
			<ThemeToggle />
		</div>

		<div class="w-full max-w-sm space-y-6">
			<!-- Compact brand mark (small screens, where the panel is hidden) -->
			<a href={resolve('/')} class="flex items-center justify-center gap-2 lg:hidden">
				<div
					class="bg-primary text-primary-foreground flex size-8 items-center justify-center rounded-md text-base font-bold shadow-sm"
				>
					{config.app.logo}
				</div>
				<span class="text-lg font-semibold tracking-tight">{config.app.name}</span>
			</a>

			{@render children()}
		</div>

		<p class="text-muted-foreground absolute inset-x-0 bottom-4 text-center text-xs">
			A SvelteKit admin starter template.
		</p>
	</main>
</div>

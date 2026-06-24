<!--
  Appearance settings. A segmented theme-mode control (Light / Dark / System)
  backed by mode-watcher, a live preview swatch, a language switch, and an
  interface density toggle persisted via the reusable `persisted` store.
-->
<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { setMode, mode } from 'mode-watcher';
	import { setLocale, LOCALES, i18n, type Locale } from '$lib/i18n';
	import { persisted } from '$lib/stores/persisted.svelte';
	import { cn } from '$lib/utils';
	import { toast } from 'svelte-sonner';
	import type { Component } from 'svelte';
	import Sun from '@lucide/svelte/icons/sun';
	import Moon from '@lucide/svelte/icons/moon';
	import Monitor from '@lucide/svelte/icons/monitor';
	import Check from '@lucide/svelte/icons/check';

	type ThemeMode = 'light' | 'dark' | 'system';

	interface ModeOption {
		value: ThemeMode;
		label: string;
		icon: Component;
	}

	const modeOptions: ModeOption[] = [
		{ value: 'light', label: 'Light', icon: Sun },
		{ value: 'dark', label: 'Dark', icon: Moon },
		{ value: 'system', label: 'System', icon: Monitor }
	];

	// Track the user's selected preference locally. mode-watcher only exposes the
	// resolved mode ('light'/'dark'), so we persist the "System" intent ourselves.
	const selectedMode = persisted<ThemeMode>('admin-starter:theme-pref', 'system');

	function selectMode(value: ThemeMode): void {
		selectedMode.current = value;
		setMode(value);
	}

	// Resolved appearance used to drive the live preview swatch.
	const resolved = $derived(mode.current ?? 'light');

	// Language switch, mirrored from the live i18n locale.
	let language = $state<Locale>(i18n.locale);

	function selectLanguage(value: Locale): void {
		language = value;
		setLocale(value);
	}

	// Interface density, persisted so it survives reloads in the demo.
	const compact = persisted<boolean>('admin-starter:density-compact', false);

	function onDensityChange(next: boolean): void {
		compact.current = next;
		toast.success(next ? 'Compact density enabled' : 'Comfortable density enabled');
	}
</script>

<svelte:head>
	<title>Appearance · Admin Starter</title>
</svelte:head>

<Card.Root>
	<Card.Header>
		<Card.Title>Theme</Card.Title>
		<Card.Description>
			Choose how the interface looks. System follows your device setting.
		</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-6">
		<!-- Segmented mode control. -->
		<div
			role="radiogroup"
			aria-label="Theme mode"
			class="bg-muted grid grid-cols-3 gap-1 rounded-lg p-1"
		>
			{#each modeOptions as option (option.value)}
				{@const active = selectedMode.current === option.value}
				<button
					type="button"
					role="radio"
					aria-checked={active}
					onclick={() => selectMode(option.value)}
					class={cn(
						'flex items-center justify-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors',
						'focus-visible:ring-ring focus-visible:ring-2 focus-visible:outline-none',
						active
							? 'bg-primary text-primary-foreground shadow-sm dark:text-background'
							: 'text-muted-foreground hover:text-foreground'
					)}
				>
					<option.icon class="size-4" />
					{option.label}
				</button>
			{/each}
		</div>

		<!-- Live preview swatch reflecting the currently resolved theme. -->
		<div class="grid gap-2">
			<Label class="text-muted-foreground text-xs font-normal">Preview</Label>
			<div class="overflow-hidden rounded-lg border">
				<div class="bg-background p-4">
					<div class="flex items-center gap-3">
						<div class="bg-primary size-9 shrink-0 rounded-md"></div>
						<div class="min-w-0 flex-1 space-y-1.5">
							<div class="bg-foreground/80 h-2.5 w-24 rounded-full"></div>
							<div class="bg-muted-foreground/40 h-2 w-36 rounded-full"></div>
						</div>
						<span
							class="bg-secondary text-secondary-foreground rounded-md px-2 py-1 text-xs font-medium"
						>
							{resolved === 'dark' ? 'Dark' : 'Light'}
						</span>
					</div>
					<div class="mt-4 flex flex-wrap gap-2">
						<span class="bg-primary text-primary-foreground rounded-md px-3 py-1.5 text-xs">
							Primary
						</span>
						<span class="border-border text-foreground rounded-md border px-3 py-1.5 text-xs">
							Outline
						</span>
						<span class="bg-muted text-muted-foreground rounded-md px-3 py-1.5 text-xs">
							Muted
						</span>
					</div>
				</div>
			</div>
		</div>
	</Card.Content>
</Card.Root>

<Card.Root>
	<Card.Header>
		<Card.Title>Language</Card.Title>
		<Card.Description>Select the display language for the interface.</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="grid gap-2 sm:grid-cols-2">
			{#each LOCALES as locale (locale.value)}
				{@const active = language === locale.value}
				<button
					type="button"
					onclick={() => selectLanguage(locale.value)}
					aria-pressed={active}
					class={cn(
						'flex items-center justify-between rounded-lg border px-4 py-3 text-sm transition-colors',
						active
							? 'border-primary bg-accent text-accent-foreground'
							: 'border-border hover:bg-accent/50'
					)}
				>
					<span class="font-medium">{locale.label}</span>
					{#if active}
						<Check class="text-primary size-4" />
					{/if}
				</button>
			{/each}
		</div>
	</Card.Content>
</Card.Root>

<Card.Root>
	<Card.Header>
		<Card.Title>Density</Card.Title>
		<Card.Description>Adjust the spacing of interface elements.</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="flex items-center justify-between gap-4">
			<div class="space-y-0.5">
				<Label>Compact mode</Label>
				<p class="text-muted-foreground text-sm">Reduce padding to fit more content on screen.</p>
			</div>
			<Switch checked={compact.current} onCheckedChange={onDensityChange} />
		</div>
	</Card.Content>
</Card.Root>

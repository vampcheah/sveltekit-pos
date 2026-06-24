<!--
  Root layout — global, shell-agnostic. Loads the stylesheet, mounts the
  theme watcher and the toast portal, and restores session + locale on mount.
  Route groups ((auth)/(app)) layer their own chrome on top of this.
-->
<script lang="ts">
	import '../app.css';
	// Self-hosted fonts (no external CDN) — pairs with --font-sans/--font-mono in app.css.
	import '@fontsource-variable/geist';
	import '@fontsource-variable/geist-mono';
	import { onMount } from 'svelte';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner';
	import { auth } from '$lib/auth';
	import { initLocale } from '$lib/i18n';
	import { config } from '$lib/config';

	let { children } = $props();

	onMount(() => {
		auth.init();
		initLocale();
	});
</script>

<ModeWatcher defaultMode={config.theme.defaultMode} />
<Toaster richColors position="top-right" />

{@render children()}

<!--
  ThemeToggle — ghost icon button that toggles light/dark via `mode-watcher`.
  Shows a Sun in dark mode and a Moon in light mode; the inactive icon is
  cross-faded for a polished transition. Accessible label reflects target.
-->
<script lang="ts">
	import Sun from '@lucide/svelte/icons/sun';
	import Moon from '@lucide/svelte/icons/moon';
	import { toggleMode, mode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button';

	const label = $derived(
		mode.current === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'
	);
</script>

<Button variant="ghost" size="icon" onclick={toggleMode} aria-label={label} title={label}>
	<Sun
		class="size-4 scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90"
		aria-hidden="true"
	/>
	<Moon
		class="absolute size-4 scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0"
		aria-hidden="true"
	/>
</Button>

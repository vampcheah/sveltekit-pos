<script lang="ts">
	import { i18n } from '$lib/i18n';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from 'svelte-sonner';
	import '../app.css';

	let { children } = $props();
</script>

<Toaster position="bottom-left" />
<ModeWatcher />

<ParaglideJS {i18n}>
	{@render children()}
</ParaglideJS>

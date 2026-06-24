<!-- Top-level admin layout: sidebar provider wrapping the sidebar and the inset (header + main). -->
<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from './AppSidebar.svelte';
	import AppHeader from './AppHeader.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset>
		<AppHeader />
		<main class="flex flex-1 flex-col">
			{@render children()}
		</main>
	</Sidebar.Inset>
</Sidebar.Provider>

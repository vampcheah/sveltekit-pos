<!--
  Index page. The `load` function redirects to /dashboard, so this only
  flashes briefly (e.g. with JS-disabled SSR) — show a centered spinner.
-->
<script lang="ts">
	import { Spinner } from '$lib/components/shared';
</script>

<div class="flex min-h-svh items-center justify-center bg-background">
	<Spinner class="size-6 text-primary" />
</div>

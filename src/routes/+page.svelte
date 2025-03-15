<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button/index.js';

	const openWindow = (url: string) => {
		window.open(url, '_blank');
	};
</script>

<div class="flex h-screen flex-col items-center justify-center gap-4">
	<h1>Welcome to Svelte-POS</h1>
	<div class="flex w-full max-w-2xl flex-col gap-4 p-2">
		<div class="flex flex-col gap-4 sm:flex-row">
			<Button
				class="basis-1/2 bg-blue-700 text-white hover:bg-blue-800"
				variant="outline"
				onclick={() => goto('/pos/cashier')}
			>
				Go to POS
			</Button>
			<Button
				class="basis-1/2 bg-blue-700 text-white hover:bg-blue-800"
				variant="outline"
				onclick={() => openWindow('/pos/display')}
			>
				Go to POS Display
			</Button>
		</div>
		<div class="flex flex-col gap-4 sm:flex-row">
			<Button
				class="basis-1/2 bg-blue-700 text-white hover:bg-blue-800"
				variant="outline"
				onclick={() => goto('/page-db')}
			>
				DB - Raw Management
			</Button>
			<Button
				class="basis-1/2 bg-red-700 text-white hover:bg-red-800"
				variant="outline"
				onclick={() => goto('/pos/importer')}
			>
				Import Sample Data
			</Button>
		</div>
	</div>
</div>

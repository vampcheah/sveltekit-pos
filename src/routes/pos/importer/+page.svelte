<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { AlertCircle, Upload, Home } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';
	import { importSampleData } from '$lib/components/handler/dexie/seed';

	let isImporting = $state(false);
	let progress = $state(0);

	const startImport = async () => {
		if (isImporting) return;

		try {
			isImporting = true;
			progress = 0;

			toast.success('Import Started', {
				description: 'Beginning data import process...'
			});

			await importSampleData();

			toast.success('Import Complete', {
				description: 'All data has been successfully imported'
			});
		} catch (error) {
			console.error('Import error:', error);
			toast.error('Import Failed', {
				description: 'An error occurred during the import process'
			});
		} finally {
			isImporting = false;
		}
	};

	const goToHome = () => {
		goto('/');
	};
</script>

<div class="container mx-auto px-4 py-8">
	<h1 class="mb-6 text-3xl font-bold">Data Importer</h1>

	<Alert.Root variant="destructive" class="mb-8">
		<AlertCircle class="mr-2 h-4 w-4" />
		<Alert.Title>Warning</Alert.Title>
		<Alert.Description>
			All existing data will be deleted and replaced with imported data. This action cannot be
			undone. Please make sure you have a backup if needed.
		</Alert.Description>
	</Alert.Root>

	<Card.Root class="rounded-lg p-6 shadow-md">
		<Card.Header>
			<h2 class="mb-4 text-xl font-semibold">Import Categories and Products</h2>
		</Card.Header>
		<Card.Content>
			<p class="mb-6 text-gray-600">
				Click the button below to start importing categories and products. This process will replace
				all existing data in these categories.
			</p>

			{#if progress > 0 && progress < 100}
				<div class="mb-6">
					<div class="h-2.5 w-full rounded-full bg-gray-200">
						<div class="h-2.5 rounded-full bg-primary" style="width: {progress}%"></div>
					</div>
					<p class="mt-2 text-sm text-gray-600">Import progress: {progress}%</p>
				</div>
			{/if}

			<div class="flex flex-col space-y-3 sm:flex-row sm:space-x-3 sm:space-y-0">
				<Button
					onclick={startImport}
					disabled={isImporting}
					class="w-full bg-green-800 text-white hover:bg-green-900 sm:w-auto"
				>
					<Upload class="mr-2 h-4 w-4" />
					{isImporting ? 'Importing...' : 'Start Import'}
				</Button>

				<Button onclick={goToHome} variant="outline" class="w-full sm:w-auto">
					<Home class="mr-2 h-4 w-4" />
					Back to Home
				</Button>
			</div>
		</Card.Content>
	</Card.Root>
</div>

<script lang="ts">
	import { onMount } from 'svelte';
	import { db } from '$lib/components/handler/dexie/db';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import {
		Dialog,
		DialogContent,
		DialogDescription,
		DialogFooter,
		DialogHeader,
		DialogTitle,
		DialogTrigger
	} from '$lib/components/ui/dialog';
	import {
		Table,
		TableBody,
		TableCell,
		TableHead,
		TableHeader,
		TableRow
	} from '$lib/components/ui/table';
	import { Plus, Edit, Trash2, Database, Home } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';

	// State variables
	let tables = $state<string[]>([]);
	let selectedTable = $state<string>('');
	let tableData = $state<any[]>([]);
	let columns = $state<any[]>([]);
	let isAddDialogOpen = $state(false);
	let isEditDialogOpen = $state(false);
	let currentRecord = $state<any>({});
	let newTableColumns = $state<{ name: string; type: string }[]>([{ name: '', type: 'string' }]);
	let loading = $state(true);

	// Load tables on mount
	onMount(async () => {
		try {
			tables = await db.tables.map((table) => table.name);
			loading = false;
		} catch (error) {
			console.error('Error loading tables:', error);
			toast.error('Failed to load database tables', {
				description: 'Please check your database connection'
			});
			loading = false;
		}
	});

	// Load table data when a table is selected
	$effect(() => {
		if (selectedTable) {
			loadTableData();
		}
	});

	const loadTableData = async () => {
		try {
			loading = true;
			tableData = await db.table(selectedTable).toArray();

			// Extract columns from the first record or schema
			const tableSchema = db.table(selectedTable).schema;
			columns = Object.entries(tableSchema.indexes).map(([key, value]) => ({
				accessorKey: key,
				header: value.name
			}));

			loading = false;
		} catch (error) {
			console.error('Error loading table data:', error);
			toast.error(`Failed to load data from ${selectedTable}`, {
				description: 'Could not retrieve table data'
			});
			loading = false;
		}
	};

	const openAddDialog = () => {
		currentRecord = {};
		isAddDialogOpen = true;
	};

	const openEditDialog = (record: any) => {
		currentRecord = { ...record };
		isEditDialogOpen = true;
	};

	const saveRecord = async () => {
		try {
			if (Object.keys(currentRecord).length === 0) {
				toast.error('Record cannot be empty');
				return;
			}

			loading = true;

			// Create a copy of the record without the id for new records
			const recordToSave = { ...currentRecord };

			if (currentRecord.id) {
				// Update existing record
				const id = currentRecord.id;
				// Remove id from the object to avoid Dexie errors
				delete recordToSave.id;
				await db.table(selectedTable).update(id, recordToSave);
				toast.success('Record updated successfully');
			} else {
				// For new records, make sure we don't have an undefined id field
				if (recordToSave.id === undefined) {
					delete recordToSave.id;
				}

				// Add new record
				await db.table(selectedTable).add(recordToSave);
				toast.success('Record added successfully');
			}

			isAddDialogOpen = false;
			isEditDialogOpen = false;
			await loadTableData();
		} catch (error) {
			console.error('Error saving record:', error);
			toast.error('Failed to save record');
			loading = false;
		}
	};

	const deleteRecord = async (id: any) => {
		try {
			loading = true;
			await db.table(selectedTable).delete(id);
			toast.success('Record deleted successfully');
			await loadTableData();
		} catch (error) {
			console.error('Error deleting record:', error);
			toast.error('Failed to delete record');
			loading = false;
		}
	};

	const addColumnField = () => {
		newTableColumns = [...newTableColumns, { name: '', type: 'string' }];
	};

	const removeColumnField = (index: number) => {
		newTableColumns = newTableColumns.filter((_, i) => i !== index);
	};

	const navigateToHome = () => {
		goto('/');
	};
</script>

<div class="container mx-auto py-8">
	<div class="mb-6 flex items-center justify-between">
		<h1 class="text-2xl font-bold">Database Manager</h1>
		<Button variant="outline" onclick={navigateToHome}>
			<Home class="mr-2 h-4 w-4" />
			Back to Home
		</Button>
	</div>

	<div class="mb-6 w-full max-w-xs">
		<Select.Root type="single" name="select_table" bind:value={selectedTable}>
			<Select.Trigger class="w-full">Select a table</Select.Trigger>
			<Select.Content>
				<Select.Group>
					{#each tables as table}
						<Select.Item value={table}>{table}</Select.Item>
					{/each}
				</Select.Group>
			</Select.Content>
		</Select.Root>
	</div>

	{#if selectedTable}
		<div class="mb-4 flex items-center justify-between">
			<h2 class="text-xl font-semibold">{selectedTable}</h2>

			<Dialog bind:open={isAddDialogOpen}>
				<DialogTrigger>
					<Button onclick={openAddDialog}>
						<Plus class="mr-2 h-4 w-4" />
						Add Record
					</Button>
				</DialogTrigger>
				<DialogContent
					class="max-w-sm rounded-lg sm:max-w-xl"
					onkeydown={(e) => {
						if (e.key === 'Enter') {
							e.preventDefault();
							saveRecord();
						}
					}}
				>
					<DialogHeader>
						<DialogTitle>Add New Record</DialogTitle>
						<DialogDescription>Fill in the details for the new record.</DialogDescription>
					</DialogHeader>

					<div class="grid gap-4 py-4">
						{#each columns.filter((col) => col.accessorKey !== 'id') as column}
							<div class="grid gap-2">
								<Label for={column.accessorKey}>{column.header}</Label>
								<Input
									id={column.accessorKey}
									bind:value={currentRecord[column.header]}
									placeholder={`Enter ${column.header.toLowerCase()}`}
								/>
							</div>
						{/each}
					</div>

					<DialogFooter>
						<Button variant="outline" onclick={() => (isAddDialogOpen = false)}>Cancel</Button>
						<Button onclick={saveRecord}>Save</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>

			<Dialog bind:open={isEditDialogOpen}>
				<DialogContent
					class="max-w-sm rounded-lg sm:max-w-xl"
					onkeydown={(e) => {
						if (e.key === 'Enter') {
							e.preventDefault();
							saveRecord();
						}
					}}
				>
					<DialogHeader>
						<DialogTitle>Edit Record</DialogTitle>
						<DialogDescription>Update the record details.</DialogDescription>
					</DialogHeader>

					<div class="grid gap-4 py-4">
						{#each columns.filter((col) => col.accessorKey !== 'id') as column}
							<div class="grid gap-2">
								<Label for={`edit-${column.accessorKey}`}>{column.header}</Label>
								<Input
									id={`edit-${column.accessorKey}`}
									bind:value={currentRecord[column.header]}
									placeholder={`Enter ${column.header.toLowerCase()}`}
								/>
							</div>
						{/each}
					</div>

					<DialogFooter>
						<Button variant="outline" onclick={() => (isEditDialogOpen = false)}>Cancel</Button>
						<Button onclick={saveRecord}>Update</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>

		{#if loading}
			<div class="flex h-64 items-center justify-center">
				<div class="h-12 w-12 animate-spin rounded-full border-b-2 border-primary"></div>
			</div>
		{:else if tableData.length > 0}
			<div class="rounded-md border">
				<Table>
					<TableHeader>
						<TableRow>
							{#each columns as column}
								<TableHead>{column.header}</TableHead>
							{/each}
							<TableHead>Actions</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each tableData as row}
							<TableRow>
								{#each columns as column}
									<TableCell>{row[column.header]}</TableCell>
								{/each}
								<TableCell>
									<div class="flex items-center gap-2">
										<Button variant="ghost" size="icon" onclick={() => openEditDialog(row)}>
											<Edit class="h-4 w-4" />
										</Button>
										<Button variant="ghost" size="icon" onclick={() => deleteRecord(row.id)}>
											<Trash2 class="h-4 w-4" />
										</Button>
									</div>
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</div>
		{:else}
			<div class="rounded-md border p-8 text-center">
				<p class="text-muted-foreground">No records found in this table.</p>
				<Button class="mt-4" onclick={openAddDialog}>
					<Plus class="mr-2 h-4 w-4" />
					Add First Record
				</Button>
			</div>
		{/if}
	{:else if !loading}
		<div class="flex h-64 flex-col items-center justify-center text-center">
			<Database class="mb-4 h-16 w-16 text-muted-foreground" />
			<h3 class="mb-2 text-xl font-medium">No Table Selected</h3>
			<p class="mb-4 text-muted-foreground">
				Select a table from the dropdown or create a new one to get started.
			</p>
		</div>
	{/if}
</div>

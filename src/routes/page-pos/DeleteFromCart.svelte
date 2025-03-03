<script lang="ts">
	import { onDestroy } from 'svelte';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';

	let { item, deleteFromCart } = $props();
	let isOpen = $state(false);
	let confirm_text = $state('');

	const delete_now = async () => {
		deleteFromCart(item.product.id);
	};

	onDestroy(() => {
		confirm_text = '';
	});
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Trigger>
		<Button variant="outline" class="h-5 border-red-600 px-2 text-xs">Remove</Button>
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
			<Dialog.Description>
				<div class="grid gap-4 py-4">
					<div class="flex items-center gap-4">
						Deleting Item: <span class="border-b-2 border-red-500 text-lg font-bold text-red-500"
							>{item.product.name}</span
						>
					</div>
				</div>
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<div class="flex w-full flex-row justify-end gap-2">
				<div class="flex flex-row gap-2">
					<Button variant="destructive" onclick={delete_now}>Delete</Button>
				</div>
			</div>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { ShoppingCart, Trash2, Save, Loader2 } from 'lucide-svelte';

	let { savedCarts, cart, isSaving, onLoadCart, onDeleteCart, onSaveCart } = $props();

	// Format date for display
	const formatDate = (date: Date) => {
		return new Intl.DateTimeFormat('default', {
			hour: '2-digit',
			minute: '2-digit'
		}).format(date);
	};
</script>

<div class="bg-card flex w-72 flex-col border-l border-r">
	<div class="flex h-9 items-center justify-between border-b p-4">
		<h2 class="flex items-center gap-2 text-lg font-bold">
			<ShoppingCart class="h-5 w-5" />
			Saved Carts
		</h2>
	</div>

	<ScrollArea class="flex-1">
		{#if savedCarts.length === 0}
			<div class="text-muted-foreground flex h-64 flex-col items-center justify-center p-4">
				<ShoppingCart class="mb-2 h-12 w-12" />
				<p>No saved carts</p>
			</div>
		{:else}
			<div class="space-y-3 p-3">
				{#each savedCarts as savedCart (savedCart.id)}
					<Card class="hover:bg-accent/50 cursor-pointer transition-colors">
						<CardContent class="p-3" onclick={() => onLoadCart(savedCart)}>
							<div class="flex items-center justify-between">
								<div>
									<p class="font-medium">{savedCart.name}</p>
									<p class="text-muted-foreground text-xs">
										{savedCart.items.reduce(
											(
												sum: number,
												item: { product: { isWeighed: boolean }; quantity: number }
											) => {
												if (!item.product.isWeighed) {
													return sum + item.quantity;
												}
												return sum;
											},
											0
										)} items,
										{savedCart.items
											.reduce(
												(
													sum: number,
													item: { product: { isWeighed: boolean }; quantity: number }
												) => {
													if (item.product.isWeighed) {
														return sum + item.quantity;
													}
													return sum;
												},
												0
											)
											.toFixed(2)} kg
									</p>
									<p class="text-muted-foreground text-xs">
										{formatDate(savedCart.timestamp)}
									</p>
								</div>
								<Button
									variant="destructive"
									size="sm"
									onclick={(e) => {
										e.stopPropagation();
										onDeleteCart(savedCart.id);
									}}
								>
									<Trash2 class="h-4 w-4" />
								</Button>
							</div>
						</CardContent>
					</Card>
				{/each}
			</div>
		{/if}
	</ScrollArea>

	<div class="flex gap-2 border-t p-3">
		<Button
			onclick={onSaveCart}
			disabled={isSaving || cart.length === 0}
			size="sm"
			class="text-primary-foreground w-full bg-green-600 hover:bg-green-800"
		>
			{#if isSaving}
				<Loader2 class="mr-2 h-4 w-4 animate-spin" />
				Saving
			{:else}
				<Save class="mr-2 h-4 w-4" />
				Save
			{/if}
		</Button>
	</div>
</div>

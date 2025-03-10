<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { ShoppingCart, Trash2, Save, Loader2 } from 'lucide-svelte';
	import { cartStore } from './CartStore.svelte';
	import * as m from '$lib/paraglide/messages.js';

	// Format date for display
	const formatDate = (date: string) => {
		return new Intl.DateTimeFormat('default', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		}).format(new Date(date));
	};
</script>

<div
	class="flex-col border-l border-r bg-card sm:flex sm:w-72 {cartStore.showSavedCarts
		? 'flex w-full'
		: 'hidden'}"
>
	<div class="flex h-9 items-center justify-between border-b p-4">
		<h2 class="flex items-center gap-2 text-lg font-bold">
			<ShoppingCart class="h-5 w-5" />
			{m.pos_saved_carts()}
		</h2>
	</div>

	<ScrollArea class="flex-1">
		{#if cartStore.savedCarts.length === 0}
			<div class="flex h-64 flex-col items-center justify-center p-4 text-muted-foreground">
				<ShoppingCart class="mb-2 h-12 w-12" />
				<p>{m.pos_no_saved_carts()}</p>
			</div>
		{:else}
			<div class="space-y-3 p-3">
				{#each cartStore.savedCarts as savedCart (savedCart.id)}
					<Card class="cursor-pointer transition-colors hover:bg-accent/50">
						<CardContent class="p-3" onclick={() => cartStore.loadSavedCart(savedCart)}>
							<div class="flex items-center justify-between">
								<div>
									<p class="font-medium">{savedCart.name}</p>
									<p class="text-xs text-muted-foreground">
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
										)}
										{m.pos_items()},
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
											.toFixed(2)}
										{m.pos_kg()}
									</p>
									<p class="text-xs text-muted-foreground">
										{formatDate(savedCart.timestamp)}
									</p>
								</div>
								<Button
									variant="destructive"
									size="sm"
									onclick={(e) => {
										e.stopPropagation();
										cartStore.deleteSavedCart(savedCart.id);
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

	<div class="flex gap-2 border-t px-3 py-4">
		<Button
			onclick={cartStore.saveCurrentCart}
			disabled={cartStore.isSaving || cartStore.cart.length === 0}
			size="lg"
			class="w-full bg-green-600 text-white hover:bg-green-800"
		>
			{#if cartStore.isSaving}
				<Loader2 class="mr-2 h-4 w-4 animate-spin" />
				{m.pos_button_saving()}
			{:else}
				<Save class="mr-2 h-4 w-4" />
				{m.pos_button_save()}
			{/if}
		</Button>
	</div>
</div>

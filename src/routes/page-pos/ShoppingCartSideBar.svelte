<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { ShoppingCart, Trash2 } from 'lucide-svelte';
	import CartItem from './CartItem.svelte';
	import { cartStore } from './CartStore.svelte';
	import CheckoutDialog from './CheckoutDialog.svelte';
	import * as m from '$lib/paraglide/messages.js';
</script>

<div
	class="mb-14 w-full flex-col border-l bg-card sm:mb-0 sm:w-96 {cartStore.showCartOnMobile
		? 'flex'
		: 'hidden'} sm:flex"
>
	<div class="border-b p-4">
		<div class="flex h-9 items-center justify-between">
			<h2 class="flex items-center gap-2 text-xl font-bold">
				<ShoppingCart class="h-5 w-5" />
				{m.pos_cart()}
			</h2>

			{#if cartStore.cart.length > 0}
				<Button variant="ghost" size="sm" onclick={cartStore.clearCart}>
					<Trash2 class="mr-1 h-4 w-4" />
					{m.pos_button_clear_cart()}
				</Button>
			{/if}
		</div>
	</div>

	<!-- Cart items list -->
	<ScrollArea class="flex-1">
		{#if cartStore.cart.length === 0}
			<div class="flex h-64 flex-col items-center justify-center text-muted-foreground">
				<ShoppingCart class="mb-2 h-12 w-12" />
				<p>{m.pos_cart_empty()}</p>
			</div>
		{:else}
			<div class="space-y-4 p-4">
				{#each cartStore.cart as item (item.product.id)}
					<CartItem {item} />
				{/each}
			</div>
		{/if}
	</ScrollArea>

	<!-- Checkout area -->
	<div class="border-t px-3 py-4 sm:px-4">
		<div class="flex flex-col pb-4">
			<div class="text-md flex justify-between">
				<span>{m.pos_items()}</span>
				<span>
					{cartStore.cart.reduce(
						(sum: number, item: { product: { isWeighed: boolean }; quantity: number }) => {
							// Only count non-weighed items for the item count
							if (!item.product.isWeighed) {
								return sum + item.quantity;
							}
							return sum;
						},
						0
					)}
					{m.pos_items()},
					{cartStore.cart
						.reduce((sum: number, item: { product: { isWeighed: boolean }; quantity: number }) => {
							// Only count weighed items for the weight
							if (item.product.isWeighed) {
								return sum + item.quantity;
							}
							return sum;
						}, 0)
						.toFixed(2)}
					{m.pos_kg()}
				</span>
			</div>
			<div class="text-md flex justify-between">
				<span>{m.pos_subtotal()}</span>
				<span>{cartStore.total.toFixed(2)}</span>
			</div>
			<div class="text-md flex justify-between font-thin">
				<span>
					{m.pos_tax()}
					{#if cartStore.tax}
						({cartStore.tax}%)
					{/if}
				</span>
				<span>{cartStore.tax.toFixed(2)}</span>
			</div>
			<div class="text-md flex justify-between font-thin">
				<span>{m.pos_discount()}</span>
				<span>{cartStore.discount.toFixed(2)}</span>
			</div>
			<div class="flex justify-between text-xl font-extrabold">
				<span>{m.pos_total()}</span>
				<span>{cartStore.grandTotal.toFixed(2)}</span>
			</div>
		</div>
		<div class="flex justify-between text-lg font-bold">
			<CheckoutDialog />
		</div>
	</div>
</div>

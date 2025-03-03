<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { ShoppingCart, Trash2, CreditCard } from 'lucide-svelte';
	import CartItem from './CartItem.svelte';

	let {
		showCartOnMobile,
		cart,
		total,
		addToCart,
		removeFromCart,
		deleteFromCart,
		clearCart,
		checkout,
		onEditWeight
	} = $props();
</script>

<div
	class="mb-14 w-full flex-col border-l bg-card sm:mb-0 sm:w-96 {showCartOnMobile
		? 'flex'
		: 'hidden'} sm:flex"
>
	<div class="border-b p-4">
		<div class="flex h-9 items-center justify-between">
			<h2 class="flex items-center gap-2 text-xl font-bold">
				<ShoppingCart class="h-5 w-5" />
				Cart
			</h2>

			{#if cart.length > 0}
				<Button variant="ghost" size="sm" onclick={clearCart}>
					<Trash2 class="mr-1 h-4 w-4" />
					Clear
				</Button>
			{/if}
		</div>
	</div>

	<!-- Cart items list -->
	<ScrollArea class="flex-1">
		{#if cart.length === 0}
			<div class="flex h-64 flex-col items-center justify-center text-muted-foreground">
				<ShoppingCart class="mb-2 h-12 w-12" />
				<p>Your cart is empty</p>
			</div>
		{:else}
			<div class="space-y-4 p-4">
				{#each cart as item (item.product.id)}
					<CartItem {item} {addToCart} {removeFromCart} {deleteFromCart} {onEditWeight} />
				{/each}
			</div>
		{/if}
	</ScrollArea>

	<!-- Checkout area -->
	<div class="border-t p-4">
		<div class="space-y-2">
			<div class="flex justify-between text-lg font-bold">
				<span>Items:</span>
				<span>
					{cart.reduce(
						(sum: number, item: { product: { isWeighed: boolean }; quantity: number }) => {
							// Only count non-weighed items for the item count
							if (!item.product.isWeighed) {
								return sum + item.quantity;
							}
							return sum;
						},
						0
					)} items,
					{cart
						.reduce((sum: number, item: { product: { isWeighed: boolean }; quantity: number }) => {
							// Only count weighed items for the weight
							if (item.product.isWeighed) {
								return sum + item.quantity;
							}
							return sum;
						}, 0)
						.toFixed(2)} kg
				</span>
			</div>
			<div class="flex justify-between text-lg">
				<span>Subtotal:</span>
				<span>{total.toFixed(2)}</span>
			</div>
			<div class="flex justify-between text-lg font-bold">
				<span>Total:</span>
				<span>{total.toFixed(2)}</span>
			</div>
			<Button
				class="w-full bg-blue-700 text-white hover:bg-blue-800"
				size="lg"
				disabled={cart.length === 0}
				onclick={checkout}
			>
				<CreditCard class="mr-2 h-5 w-5" />
				Checkout
			</Button>
		</div>
	</div>
</div>

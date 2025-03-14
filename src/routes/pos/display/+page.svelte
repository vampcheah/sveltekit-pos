<script lang="ts">
	import { onMount } from 'svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import type { WeightedProduct, Product } from '../types';
	import * as Card from '$lib/components/ui/card/index.js';
	import { numberWithCurrency } from '$lib/tools/numbering';
	import { localStore, type LocalStorageType } from '$lib/localStore.svelte';
	import * as m from '$lib/paraglide/messages.js';

	let cartItems = $state<{ product: WeightedProduct | Product; quantity: number }[]>([]);
	let subtotal = $state(0);
	let tax = $state(0);
	let discount = $state(0);
	let total = $state(0);

	const loadCartData = () => {
		const cartStorage: LocalStorageType<
			{ product: WeightedProduct | Product; quantity: number }[]
		> = localStore('pos.cart', [] as { product: WeightedProduct | Product; quantity: number }[]);
		if (cartStorage.current) {
			cartItems = cartStorage.current;
			calculateTotals();
		}
	};

	const calculateTotals = () => {
		subtotal = cartItems.reduce((sum, item) => sum + item.product.price * item.quantity, 0);
		total = subtotal + tax - discount;
	};

	// Using $effect to update when the localStorage value changes
	$effect(() => {
		const handleStorageChange = (event: StorageEvent) => {
			if (event.key === 'pos.cart') {
				loadCartData();
			}
		};

		// Listen to the 'storage' event
		window.addEventListener('storage', handleStorageChange);

		// Cleanup event listener when the component is destroyed
		return () => {
			window.removeEventListener('storage', handleStorageChange);
		};
	});

	onMount(() => {
		loadCartData();

		// Set the viewport height variable for mobile browsers
		const setVh = () => {
			const vh = window.innerHeight * 0.01;
			document.documentElement.style.setProperty('--vh', `${vh}px`);
		};

		setVh();
		window.addEventListener('resize', setVh);

		return () => {
			window.removeEventListener('resize', setVh);
		};
	});
</script>

<div class="full-height flex flex-col overflow-hidden bg-background p-4">
	<Card.Root class="flex h-full w-full flex-1 flex-col">
		<Card.Content class="flex flex-1 flex-col overflow-hidden rounded-lg p-0">
			<ScrollArea class="flex-1">
				{#if cartItems.length === 0}
					<div class="flex h-full items-center justify-center p-8">
						<p class="text-xl text-muted-foreground">No items in cart</p>
					</div>
				{:else}
					<div class="w-full">
						<div
							class="sticky top-0 z-10 grid grid-cols-4 gap-4 border-b bg-background p-4 font-medium"
						>
							<div>Item</div>
							<div class="text-right">Qty</div>
							<div class="text-right">Price</div>
							<div class="text-right">Total</div>
						</div>
						<div class="divide-y">
							{#each cartItems as item}
								<div class="grid grid-cols-4 gap-4 p-4">
									<div class="font-medium">{item.product.name}</div>
									<div class="text-right">{item.quantity}</div>
									<div class="text-right">{numberWithCurrency(item.product.price)}</div>
									<div class="text-right">
										{numberWithCurrency(item.product.price * item.quantity)}
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</ScrollArea>
		</Card.Content>
		<Card.Footer class="w-full border-t bg-muted/30 px-4 py-2">
			<div class="flex w-full flex-col">
				<div class="text-md flex justify-between font-thin">
					<span>{m.pos_items()}</span>
					<span>
						{cartItems.reduce(
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
						{cartItems
							.reduce(
								(sum: number, item: { product: { isWeighed: boolean }; quantity: number }) => {
									// Only count weighed items for the weight
									if (item.product.isWeighed) {
										return sum + item.quantity;
									}
									return sum;
								},
								0
							)
							.toFixed(2)}
						{m.pos_kg()}
					</span>
				</div>
				<div class="text-md mb-1 flex justify-between font-thin">
					<span>Subtotal:</span>
					<span>{numberWithCurrency(subtotal)}</span>
				</div>
				<div class="text-md mb-1 flex justify-between font-thin">
					<span>Tax:</span>
					<span>{numberWithCurrency(tax)}</span>
				</div>
				<div class="text-md mb-1 flex justify-between font-thin">
					<span>Discount:</span>
					<span>{numberWithCurrency(discount)}</span>
				</div>
				<div class="flex justify-between text-xl font-bold">
					<span>Total:</span>
					<span>{numberWithCurrency(total)}</span>
				</div>
			</div>
		</Card.Footer>
	</Card.Root>
</div>

<style>
	/* use dynamic --vh value to control height */
	.full-height {
		height: calc(var(--vh, 1vh) * 100);
	}
</style>

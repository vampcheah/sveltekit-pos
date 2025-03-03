<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { Plus, Minus } from 'lucide-svelte';
	import DeleteFromCart from './DeleteFromCart.svelte';
	import type { WeightedProduct } from './types';

	let { item, removeFromCart, addToCart, deleteFromCart, onEditWeight } = $props();

	const subtotal = $derived(item.product.price * item.quantity);
</script>

<div class="py-1">
	<div class="flex items-start justify-between gap-4">
		<!-- Product image and info -->
		<div class="flex gap-3">
			<img
				src={item.product.image}
				alt={item.product.name}
				class="h-16 w-16 rounded-md object-cover shadow-sm"
			/>
			<div class="flex flex-col items-start justify-center">
				<h4 class="text-base font-medium">{item.product.name}</h4>
				<p class="text-sm text-muted-foreground">
					{item.product.price.toFixed(2)} ×
					{#if item.product.isWeighed}
						{item.quantity.toFixed(2)} {item.product.unit}
					{:else}
						{item.quantity}
					{/if}
				</p>
				<p>
					<DeleteFromCart {item} {deleteFromCart} />
				</p>
			</div>
		</div>

		<!-- Quantity controls and delete button -->
		<div class="flex flex-col items-center gap-1">
			<div class="flex w-full items-center justify-end">
				<div class="mt-1 text-xl font-semibold text-green-400">
					{subtotal.toFixed(2)}
				</div>
			</div>
			<div class="flex items-center gap-2">
				{#if item.product.isWeighed}
					<Button
						variant="outline"
						size="sm"
						class="h-8"
						onclick={() => onEditWeight(item.product as WeightedProduct, item.quantity)}
					>
						Edit Weight
					</Button>
				{:else}
					<div class="flex items-center overflow-hidden rounded-md border">
						<Button
							variant="ghost"
							size="icon"
							class="h-8 w-8 rounded-none"
							onclick={() => removeFromCart(item.product.id)}
						>
							<Minus class="h-4 w-4" />
						</Button>
						<span class="w-20 text-center font-medium">{item.quantity}</span>
						<Button
							variant="ghost"
							size="icon"
							class="h-8 w-8 rounded-none"
							onclick={() => addToCart(item.product)}
						>
							<Plus class="h-4 w-4" />
						</Button>
					</div>
				{/if}
			</div>
		</div>
	</div>
	<Separator class="mt-3" />
</div>

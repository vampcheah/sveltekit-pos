<script lang="ts">
	import { Card, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { cartStore } from '../CartStore.svelte';
	import { project } from '$lib/index';

	let { product } = $props();
	let isFlashing = $state(false);
	const handleFlash = () => {
		isFlashing = true;
		setTimeout(() => {
			isFlashing = false;
		}, 100);
	};
	const onAddCart = () => {
		cartStore.addToCart(product);
		handleFlash();
	};
</script>

<Card
	class="max-w-xs cursor-pointer transition-shadow hover:shadow-md {isFlashing
		? 'bg-blue-800'
		: ''}"
	onclick={onAddCart}
>
	<CardContent class="p-4">
		<div class="mb-2 flex aspect-[2/1] items-center overflow-hidden rounded-md">
			<img
				src={product.image || project.default_image}
				alt={product.name}
				class="h-auto w-full object-cover"
			/>
		</div>
		<h3 class="font-medium">{product.name}</h3>
		<div class="mt-1 flex items-center justify-between">
			<span class="text-lg font-bold text-green-500">{parseFloat(product.price).toFixed(2)}</span>
			<Badge variant="outline">{product.category}</Badge>
		</div>
	</CardContent>
</Card>

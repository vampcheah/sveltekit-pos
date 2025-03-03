<script lang="ts">
	import { Tabs, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import ProductItem from './ProductItem.svelte';
	import type { Product } from './types';

	let { categories, searchQuery, products, addToCart } = $props();
	let activeCategory = $state('All');

	// Filter products
	const filteredProducts = $derived(
		products.filter((product: Product) => {
			const matchesSearch = product.name.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesCategory = activeCategory === 'All' || product.category === activeCategory;
			return matchesSearch && matchesCategory;
		})
	);
</script>

<div class="flex flex-1 flex-col overflow-hidden p-6">
	<!-- Category tabs -->
	<Tabs value={activeCategory} class="mb-6">
		<TabsList
			class="bg-background grid rounded-lg border p-1"
			style="grid-template-columns: repeat({categories.length}, minmax(0, 1fr));"
		>
			{#each categories as category}
				<TabsTrigger
					value={category}
					onclick={() => (activeCategory = category)}
					class="transition-all duration-200 data-[state=active]:bg-blue-700 data-[state=active]:text-white data-[state=active]:shadow-sm"
				>
					{category}
				</TabsTrigger>
			{/each}
		</TabsList>
	</Tabs>

	<!-- Product grid with ScrollArea -->
	<ScrollArea class="flex-1">
		<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6">
			{#each filteredProducts as product (product.id)}
				<ProductItem {product} {addToCart} />
			{/each}
		</div>
	</ScrollArea>
</div>

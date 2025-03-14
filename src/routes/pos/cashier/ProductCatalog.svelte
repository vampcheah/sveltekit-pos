<script lang="ts">
	import { Tabs, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import ProductItem from './ProductItem.svelte';
	import { cartStore } from '../CartStore.svelte';
	// Import sample data
	import { products } from '$lib/sample_data/products';
	import { categories } from '$lib/sample_data/categories';

	let activeCategory = $state('All');

	// Filter products
	const filteredProducts = $derived(
		products.filter((product: any) => {
			const matchesSearch = product.name
				.toLowerCase()
				.includes(cartStore.searchQuery.toLowerCase());
			const matchesCategory = activeCategory === 'All' || product.category === activeCategory;
			return matchesSearch && matchesCategory;
		})
	);
</script>

<div class="flex flex-1 flex-col overflow-hidden p-2 sm:p-6">
	<!-- Category tabs -->
	<Tabs value={activeCategory} class="mb-0 sm:mb-6">
		<div class="hide-scrollbar overflow-x-auto pb-2">
			<TabsList
				class="inline-flex w-auto min-w-full justify-between rounded-lg border bg-background p-1"
			>
				{#each categories as category}
					<TabsTrigger
						value={category}
						onclick={() => (activeCategory = category)}
						class="min-w-[100px] whitespace-nowrap transition-all duration-200 data-[state=active]:bg-blue-700 data-[state=active]:text-white data-[state=active]:shadow-sm sm:min-w-[150px]"
					>
						{category}
					</TabsTrigger>
				{/each}
			</TabsList>
		</div>
	</Tabs>

	<!-- Product grid with ScrollArea -->
	<ScrollArea class="flex-1">
		<div class="grid grid-cols-2 gap-4 lg:grid-cols-3 xl:grid-cols-4">
			{#each filteredProducts as product (product.id)}
				<ProductItem {product} />
			{/each}
		</div>
	</ScrollArea>
</div>

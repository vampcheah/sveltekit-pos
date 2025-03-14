<script lang="ts">
	import { onMount } from 'svelte';
	import { Tabs, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import ProductItem from './ProductItem.svelte';
	import { cartStore } from '../CartStore.svelte';
	import { Loader2 } from 'lucide-svelte';
	// load data
	import {
		actions as categoriesActions,
		type Category
	} from '$lib/components/handler/dexie/categories';
	import { actions as productsActions, type Product } from '$lib/components/handler/dexie/products';

	// state variables
	let isLoading = $state(true);
	let activeCategory = $state('');
	let categories = $state<Category[]>([]);
	let products = $state<Product[]>([]);
	// Filter products
	const filteredProducts = $derived(
		products.filter((product: any) => {
			const matchesSearch = product.name
				.toLowerCase()
				.includes(cartStore.searchQuery.toLowerCase());
			const matchesCategory = activeCategory === '' || product.category === activeCategory;
			return matchesSearch && matchesCategory;
		})
	);

	onMount(async () => {
		// set loading to true
		isLoading = true;

		// get all categories
		categories = await categoriesActions.getAll();
		activeCategory = categories[0].code;

		// get all products
		products = await productsActions.getAll();

		// set loading to false
		isLoading = false;
	});
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
						value={category.code}
						onclick={() => (activeCategory = category.code)}
						class="min-w-[100px] whitespace-nowrap transition-all duration-200 data-[state=active]:bg-blue-700 data-[state=active]:text-white data-[state=active]:shadow-sm sm:min-w-[150px]"
					>
						{category.name}
					</TabsTrigger>
				{/each}
			</TabsList>
		</div>
	</Tabs>

	<!-- Product grid with ScrollArea -->
	<ScrollArea class="flex-1">
		{#if !isLoading}
			<div class="grid grid-cols-2 gap-4 lg:grid-cols-3 xl:grid-cols-4">
				{#each filteredProducts as product (product.id)}
					<ProductItem {product} />
				{/each}
			</div>
		{:else}
			<div class="flex h-full items-center justify-center">
				<Loader2 class="h-10 w-10 animate-spin" />
			</div>
		{/if}
	</ScrollArea>
</div>

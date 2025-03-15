<script lang="ts">
	import { onMount } from 'svelte';
	import { Search, ShoppingCart } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	// Import project data
	import { project } from '$lib/index';
	// Import Shadcn UI components
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator/index.js';
	// Import page components
	import ProductCatalog from './ProductCatalog.svelte';
	import ShoppingCartSideBar from './ShoppingCartSideBar.svelte';
	import SavedCarts from './SavedCarts.svelte';
	import WeightInputDialog from './WeightInputDialog.svelte';
	import UserNav from './UserNav.svelte';
	// Import cart store
	import { cartStore } from '../CartStore.svelte';
	// Import i18n
	import * as m from '$lib/paraglide/messages.js';

	const setVh = () => {
		const vh = window.innerHeight * 0.01;
		document.documentElement.style.setProperty('--vh', `${vh}px`);
	};

	onMount(() => {
		setVh();
		window.addEventListener('resize', setVh);
	});
</script>

<div class="full-height flex overflow-hidden bg-background">
	<!-- Main content area -->
	<div class="flex flex-1 flex-col overflow-hidden">
		<!-- Top navigation bar -->
		<header class="flex items-center justify-between border-b px-2 py-2 sm:py-3.5">
			<button onclick={() => goto('/')}>
				<h1 class="hidden text-2xl font-bold sm:flex">{project.name}</h1>
				<h1 class="flex text-2xl font-bold sm:hidden">{project.name_short}</h1>
			</button>
			<div class="flex items-center gap-2">
				<div class="relative w-fit">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="search"
						placeholder="Search products..."
						class="w-48 pl-8 sm:w-full"
						value={cartStore.searchQuery}
						oninput={(e) => (cartStore.searchQuery = e.currentTarget.value)}
					/>
				</div>

				<Button
					variant={cartStore.showSavedCarts ? 'default' : 'outline'}
					onclick={cartStore.toggleSavedCarts}
					class="hidden sm:flex"
				>
					<ShoppingCart class="mr-2 h-4 w-4" />
					{m.pos_saved_count({ count: cartStore.savedCarts.length | 0 })}
				</Button>

				<UserNav />
			</div>
		</header>

		<!-- Main content with products and saved carts -->
		<div class="flex flex-1 overflow-hidden pb-14 sm:pb-0">
			<!-- Products catalog -->
			<div class="{cartStore.showSavedCarts ? 'hidden' : 'flex w-full'} sm:flex sm:flex-1">
				<ProductCatalog />
			</div>

			<!-- Saved Carts Section (conditionally shown) -->
			{#if cartStore.showSavedCarts}
				<SavedCarts />
			{/if}
		</div>
	</div>

	<!-- Shopping cart sidebar - desktop version -->
	<ShoppingCartSideBar />

	<!-- Mobile bottom navigation -->
	<div
		class="fixed bottom-0 flex w-full items-center justify-around gap-1 border-t bg-background px-1 py-2 sm:hidden"
	>
		<Button
			class="basis-1/2"
			variant={cartStore.showSavedCarts ? 'default' : 'ghost'}
			size="sm"
			onclick={cartStore.toggleSavedCarts}
		>
			<ShoppingCart class="h-5 w-5" />
			<span class="ml-1 text-xs"
				>{m.pos_saved_count({ count: cartStore.savedCarts.length | 0 })}</span
			>
		</Button>

		<Separator orientation="vertical" class="h-5" />

		<Button
			variant={cartStore.showCartOnMobile ? 'default' : 'ghost'}
			size="sm"
			onclick={cartStore.toggleCartOnMobile}
			class="relative basis-1/2"
		>
			<ShoppingCart class="h-5 w-5" />
			<span class="ml-1 text-xs">{m.pos_cart()}</span>
			{#if cartStore.cartItemCount > 0}
				<Badge
					class="flex h-5 w-5 items-center justify-center bg-green-600 px-1.5 py-0.5 text-xs text-white hover:bg-green-600 active:bg-green-600"
				>
					{cartStore.cartItemCount}
				</Badge>
			{/if}
		</Button>
	</div>

	<!-- Weight Input Dialog -->
	{#if cartStore.editingWeightItem}
		<WeightInputDialog />
	{/if}
</div>

<style>
	/* use dynamic --vh value to control height */
	.full-height {
		height: calc(var(--vh, 1vh) * 100);
	}
</style>

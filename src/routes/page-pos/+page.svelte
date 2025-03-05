<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Search, ShoppingCart, User } from 'lucide-svelte';
	import type { WeightedProduct, Product } from './types';
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
	// Import sample data
	import { products } from '$lib/sample_data/products';
	import { categories } from '$lib/sample_data/categories';

	// Shopping cart state
	let cart: { product: WeightedProduct | Product; quantity: number }[] = $state([]);
	let searchQuery = $state('');

	// Saved carts functionality
	let savedCarts: {
		id: number;
		name: string;
		items: { product: WeightedProduct | Product; quantity: number }[];
		timestamp: Date;
	}[] = $state([]);
	let newCartName = $state('');
	let isSaving = $state(false);
	let guestCount = $state(1);
	let showSavedCarts = $state(false);

	// Mobile view state
	let showCartOnMobile = $state(false);

	// Weight input state
	let editingWeightItem: WeightedProduct | null = $state(null);
	let weightInputValue = $state('');

	// Calculate total
	const total = $derived(cart.reduce((sum, item) => sum + item.product.price * item.quantity, 0));
	// Calculate cart item count
	const cartItemCount = $derived(cart.length);

	// Add product to cart
	const addToCart = (product: any) => {
		if (product.isWeighed) {
			// for weighted item, open the weight input dialog
			editingWeightItem = product;

			// Check if the product already exists in the cart
			const existingItem = cart.find((item) => item.product.id === product.id);
			// Use existing weight if available, otherwise empty string for better UX
			weightInputValue = existingItem ? existingItem.quantity.toString() : '';
			return;
		} else {
			editingWeightItem = null;
		}

		const existingItem = cart.find((item) => item.product.id === product.id);

		if (existingItem) {
			cart = cart.map((item) =>
				item.product.id === product.id ? { ...item, quantity: item.quantity + 1 } : item
			);
		} else {
			cart = [...cart, { product, quantity: 1 }];
		}
	};

	// Remove product from cart
	const removeFromCart = (productId: any) => {
		const existingItem = cart.find((item) => item.product.id === productId);

		if (existingItem && existingItem.quantity > 1) {
			cart = cart.map((item) =>
				item.product.id === productId ? { ...item, quantity: item.quantity - 1 } : item
			);
		} else {
			cart = cart.filter((item) => item.product.id !== productId);
		}
	};

	// Delete item from cart
	const deleteFromCart = (productId: any) => {
		cart = cart.filter((item) => item.product.id !== productId);
	};

	// Clear cart
	const clearCart = () => {
		cart = [];
	};

	// Checkout
	const checkout = () => {
		alert(`Checkout successful! Total amount: ${total.toFixed(2)}`);
		clearCart();
	};

	// Save current cart
	const saveCurrentCart = () => {
		if (cart.length === 0) return;

		isSaving = true;

		// Use "Guest X" if no name provided
		const cartName = newCartName.trim() || `Guest ${guestCount}`;
		if (!newCartName.trim()) {
			guestCount++;
		}

		// Add the new cart
		savedCarts = [
			...savedCarts,
			{
				id: Date.now(),
				name: cartName,
				items: [...cart],
				timestamp: new Date()
			}
		];

		newCartName = '';
		isSaving = false;
		cart = [];
	};

	// Load a saved cart
	const loadSavedCart = (savedCart: any) => {
		cart = [...savedCart.items];
		// Remove the loaded cart from saved carts
		deleteSavedCart(savedCart.id);

		if (cart.length === 0) {
			showSavedCarts = false;
		}
	};

	// Delete a saved cart
	const deleteSavedCart = (id: number) => {
		savedCarts = savedCarts.filter((cart) => cart.id !== id);

		if (savedCarts.length === 0) {
			guestCount = 1;
			showSavedCarts = false;
		}
	};

	// Toggle saved carts view
	const toggleSavedCarts = () => {
		showSavedCarts = !showSavedCarts;
		// If showing saved carts, hide cart on mobile
		if (showSavedCarts) {
			showCartOnMobile = false;
		}
	};

	// Toggle mobile cart view
	const toggleCartOnMobile = () => {
		showCartOnMobile = !showCartOnMobile;
		// If showing cart, hide saved carts
		if (showCartOnMobile) {
			showSavedCarts = false;
		}
	};

	// Handle weight input
	const handleEditWeight = (product: WeightedProduct, quantity: number) => {
		editingWeightItem = product;
		weightInputValue = quantity.toString();
	};

	// Confirm weight input
	const confirmWeightInput = () => {
		if (!editingWeightItem) return;

		const weight = parseFloat(weightInputValue);
		if (isNaN(weight) || weight <= 0) {
			alert('Please enter a valid weight value');
			return;
		}

		const existingItem = cart.find(
			(item) => item.product.id === (editingWeightItem as WeightedProduct)?.id
		);

		if (existingItem) {
			cart = cart.map((item) =>
				item.product.id === (editingWeightItem as WeightedProduct)?.id
					? { ...item, quantity: weight }
					: item
			);
		} else {
			cart = [...cart, { product: editingWeightItem, quantity: weight }];
		}

		// reset the state
		editingWeightItem = null;
		weightInputValue = '';
	};

	// Cancel weight input
	const cancelWeightInput = () => {
		editingWeightItem = null;
		weightInputValue = '';
	};

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
			<h1 class="hidden text-2xl font-bold sm:flex">{project.name}</h1>
			<h1 class="flex text-2xl font-bold sm:hidden">{project.name_short}</h1>
			<div class="flex items-center gap-2">
				<div class="relative w-fit">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="search"
						placeholder="Search products..."
						class="w-48 pl-8 sm:w-full"
						bind:value={searchQuery}
					/>
				</div>

				<Button
					variant={showSavedCarts ? 'default' : 'outline'}
					onclick={() => (showSavedCarts = !showSavedCarts)}
					class="hidden sm:flex"
				>
					<ShoppingCart class="mr-2 h-4 w-4" />
					Saved ({savedCarts.length})
				</Button>

				<Button variant="outline" size="icon" onclick={() => goto('/')}>
					<User class="h-5 w-5" />
				</Button>
			</div>
		</header>

		<!-- Main content with products and saved carts -->
		<div class="flex flex-1 overflow-hidden pb-14 sm:pb-0">
			<!-- Products catalog -->
			<div class="{showSavedCarts ? 'hidden' : 'flex w-full'} sm:flex sm:flex-1">
				<ProductCatalog {categories} {searchQuery} {products} {addToCart} />
			</div>

			<!-- Saved Carts Section (conditionally shown) -->
			{#if showSavedCarts}
				<SavedCarts
					{showSavedCarts}
					{savedCarts}
					{cart}
					{isSaving}
					onLoadCart={loadSavedCart}
					onDeleteCart={deleteSavedCart}
					onSaveCart={saveCurrentCart}
				/>
			{/if}
		</div>
	</div>

	<!-- Shopping cart sidebar - desktop version -->
	<ShoppingCartSideBar
		{showCartOnMobile}
		{cart}
		{total}
		{addToCart}
		{removeFromCart}
		{deleteFromCart}
		{clearCart}
		{checkout}
		onEditWeight={handleEditWeight}
	/>

	<!-- Mobile bottom navigation -->
	<div
		class="fixed bottom-0 flex w-full items-center justify-around border-t bg-background py-2 sm:hidden"
	>
		<Button
			class="basis-1/2"
			variant={showSavedCarts ? 'default' : 'ghost'}
			size="sm"
			onclick={toggleSavedCarts}
		>
			<ShoppingCart class="h-5 w-5" />
			<span class="ml-1 text-xs">Saved ({savedCarts.length})</span>
		</Button>

		<Separator orientation="vertical" class="h-5" />

		<Button
			variant={showCartOnMobile ? 'default' : 'ghost'}
			size="sm"
			onclick={toggleCartOnMobile}
			class="relative basis-1/2"
		>
			<ShoppingCart class="h-5 w-5" />
			<span class="ml-1 text-xs">Cart</span>
			{#if cartItemCount > 0}
				<Badge
					class="flex h-5 w-5 items-center justify-center bg-green-600 px-1.5 py-0.5 text-xs text-white hover:bg-green-600 active:bg-green-600"
				>
					{cartItemCount}
				</Badge>
			{/if}
		</Button>
	</div>

	<!-- Weight Input Dialog -->
	{#if editingWeightItem}
		<WeightInputDialog
			bind:editingWeightItem
			bind:weightInputValue
			onConfirm={confirmWeightInput}
			onCancel={cancelWeightInput}
		/>
	{/if}
</div>

<style>
	/* use dynamic --vh value to control height */
	.full-height {
		height: calc(var(--vh, 1vh) * 100);
	}
</style>

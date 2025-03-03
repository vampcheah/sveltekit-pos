<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Search, ShoppingCart, User } from 'lucide-svelte';
	import type { WeightedProduct, Product } from './types';
	import { goto } from '$app/navigation';
	import { project } from '$lib/index';
	// Import our new components
	import ProductCatalog from './ProductCatalog.svelte';
	import ShoppingCartSideBar from './ShoppingCartSideBar.svelte';
	import SavedCarts from './SavedCarts.svelte';
	import WeightInputDialog from './WeightInputDialog.svelte';
	import { onMount } from 'svelte';

	// Mock product data
	const categories = ['All', 'Food', 'Drinks', 'Desserts', 'Main Dishes'];

	const products = [
		{
			id: 1,
			name: 'Hamburger',
			price: 25,
			category: 'Main Dishes',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 2,
			name: 'French Fries',
			price: 12,
			category: 'Food',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 3,
			name: 'Cola',
			price: 8,
			category: 'Drinks',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 4,
			name: 'Ice Cream',
			price: 10,
			category: 'Desserts',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 5,
			name: 'Pizza',
			price: 45,
			category: 'Main Dishes',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 6,
			name: 'Salad',
			price: 18,
			category: 'Food',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 7,
			name: 'Milk Tea',
			price: 15,
			category: 'Drinks',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 8,
			name: 'Cake',
			price: 22,
			category: 'Desserts',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 9,
			name: 'Sandwich',
			price: 20,
			category: 'Main Dishes',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 10,
			name: 'Fried Chicken',
			price: 28,
			category: 'Food',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 11,
			name: 'Juice',
			price: 12,
			category: 'Drinks',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		{
			id: 12,
			name: 'Pudding',
			price: 8,
			category: 'Desserts',
			image: 'https://placehold.co/100x100',
			isWeighed: false
		},
		// item with weight
		{
			id: 13,
			name: 'Beef (kg)',
			price: 35,
			category: 'Food',
			image: 'https://placehold.co/100x100',
			isWeighed: true,
			unit: 'kg'
		},
		{
			id: 14,
			name: 'Apples (kg)',
			price: 8.2,
			category: 'Food',
			image: 'https://placehold.co/100x100',
			isWeighed: true,
			unit: 'kg'
		},
		{
			id: 15,
			name: 'Cheese (kg)',
			price: 25,
			category: 'Food',
			image: 'https://placehold.co/100x100',
			isWeighed: true,
			unit: 'kg'
		}
	];

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

	// Weight input state
	let editingWeightItem: WeightedProduct | null = $state(null);
	let weightInputValue = $state('');

	// Calculate total
	const total = $derived(cart.reduce((sum, item) => sum + item.product.price * item.quantity, 0));

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
		<header class="flex items-center justify-between border-b px-6 py-3.5">
			<h1 class="text-2xl font-bold">{project.name}</h1>
			<div class="flex items-center gap-4">
				<div class="relative w-64">
					<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
					<Input
						type="search"
						placeholder="Search products..."
						class="pl-8"
						bind:value={searchQuery}
					/>
				</div>

				<Button
					variant={showSavedCarts ? 'default' : 'outline'}
					onclick={() => (showSavedCarts = !showSavedCarts)}
				>
					<ShoppingCart class="mr-2 h-4 w-4" />
					Saved Carts ({savedCarts.length})
				</Button>

				<Button variant="outline" size="icon" onclick={() => goto('/')}>
					<User class="h-5 w-5" />
				</Button>
			</div>
		</header>

		<!-- Main content with products and saved carts -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Products catalog -->
			<ProductCatalog {categories} {searchQuery} {products} {addToCart} />

			<!-- Saved Carts Section (conditionally shown) -->
			{#if showSavedCarts}
				<SavedCarts
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

	<!-- Shopping cart sidebar -->
	<ShoppingCartSideBar
		{cart}
		{total}
		{addToCart}
		{removeFromCart}
		{deleteFromCart}
		{clearCart}
		{checkout}
		onEditWeight={handleEditWeight}
	/>

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
	/* 使用动态设置的 --vh 值来控制高度 */
	.full-height {
		height: calc(var(--vh, 1vh) * 100);
	}
</style>

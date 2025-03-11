import { goto } from '$app/navigation';
import type { WeightedProduct, Product, SavedCart } from './types';
import { toast } from 'svelte-sonner';
import * as m from '$lib/paraglide/messages.js';
import { localStore, type LocalStorageType } from '$lib/localStore.svelte';
import { getCurrentTime } from '$lib/tools/time';
import { IsMobile } from '$lib/hooks/is-mobile.svelte.js';

export class CartStore {
	// Cart state variables
	localCart: LocalStorageType<{ product: WeightedProduct | Product; quantity: number }[]> =
		localStore('pos.cart', [] as { product: WeightedProduct | Product; quantity: number }[]);
	cart: { product: WeightedProduct | Product; quantity: number }[] = $state(this.localCart.current);
	localSavedCarts: LocalStorageType<SavedCart[]> = localStore('pos.savedCarts', [] as SavedCart[]);
	savedCarts: SavedCart[] = $state(this.localSavedCarts.current);

	// Checkout state variables
	discount = $state(0);
	tax = $state(0);

	// UI state variables
	searchQuery = $state('');
	newCartName = $state('');
	isSaving = $state(false);
	localGuestCount: LocalStorageType<number> = localStore('pos.guestCount', 1);
	guestCount = $state(this.localGuestCount.current);
	showSavedCarts = $state(false);
	showCartOnMobile = $state(false);
	#isMobile: IsMobile | null = $state(null);

	constructor() {
		$effect.root(() => {
			this.#isMobile = new IsMobile();
		});
	}

	// Weight input state
	editingWeightItem: WeightedProduct | null = $state(null);
	weightInputValue = $state('');

	// Computed values
	get total() {
		return this.cart.reduce((sum, item) => sum + item.product.price * item.quantity, 0);
	}

	get grandTotal() {
		const calculatedTax = this.total * (this.tax / 100);
		const calculatedDiscount = this.discount;

		return this.total + calculatedTax - calculatedDiscount;
	}

	get cartItemCount() {
		return this.cart.length;
	}

	// Cart management methods
	addToCart = (product: WeightedProduct | Product) => {
		if (product.isWeighed) {
			// For weighted item, open the weight input dialog
			this.editingWeightItem = product as WeightedProduct;

			// Check if the product already exists in the cart
			const existingItem = this.cart.find((item) => item.product.id === product.id);
			// Use existing weight if available, otherwise empty string for better UX
			this.weightInputValue = existingItem ? existingItem.quantity.toString() : '';
			return;
		} else {
			this.editingWeightItem = null;
		}

		const existingItem = this.cart.find((item) => item.product.id === product.id);

		if (existingItem) {
			this.cart = this.cart.map((item) =>
				item.product.id === product.id ? { ...item, quantity: item.quantity + 1 } : item
			);
		} else {
			this.cart = [...this.cart, { product, quantity: 1 }];
		}
		this.localCart.current = this.cart;
	};

	removeFromCart = (productId: number) => {
		const existingItem = this.cart.find((item) => item.product.id === productId);

		if (existingItem && existingItem.quantity > 1) {
			this.cart = this.cart.map((item) =>
				item.product.id === productId ? { ...item, quantity: item.quantity - 1 } : item
			);
		} else {
			this.cart = this.cart.filter((item) => item.product.id !== productId);
		}
		this.localCart.current = this.cart;
	};

	deleteFromCart = (productId: number) => {
		this.cart = this.cart.filter((item) => item.product.id !== productId);
		this.localCart.current = this.cart;
	};

	clearCart = () => {
		this.cart = [];
		this.localCart.current = [];
	};

	checkout = () => {
		toast.success(m.pos_checkout_success({ total: this.total.toFixed(2) }));
		this.clearCart();
		this.showCartOnMobile = false;
	};

	printReceipt = () => {
		toast.success(m.pos_print_receipt());
	};

	// Saved carts methods
	saveCurrentCart = () => {
		if (this.cart.length === 0) return;

		this.isSaving = true;

		// Use "Guest X" if no name provided
		const cartName = this.newCartName.trim() || m.pos_guest({ queue_no: this.guestCount });
		if (!this.newCartName.trim()) {
			this.guestCount++;
			this.localGuestCount.current = this.guestCount;
		}

		// Add the new cart
		this.savedCarts = [
			...this.savedCarts,
			{
				id: Date.now(),
				name: cartName,
				items: [...this.cart],
				timestamp: getCurrentTime().toISOString()
			}
		];

		this.localSavedCarts.current = this.savedCarts;

		this.newCartName = '';
		this.isSaving = false;
		this.clearCart();
	};

	loadSavedCart = (savedCart: SavedCart) => {
		// load the saved cart into the cart
		this.cart = [...savedCart.items];
		this.localCart.current = this.cart;

		// Remove the loaded cart from saved carts
		this.deleteSavedCart(savedCart.id);

		if (this.cart.length === 0) {
			this.showSavedCarts = false;
		}

		if (this.#isMobile?.current) {
			this.showSavedCarts = false;
			this.showCartOnMobile = true;
		}
	};

	deleteSavedCart = (id: number) => {
		this.savedCarts = this.savedCarts.filter((cart) => cart.id !== id);
		this.localSavedCarts.current = this.savedCarts;

		if (this.savedCarts.length === 0) {
			this.guestCount = 1;
			this.localGuestCount.current = 1;
			this.showSavedCarts = false;
		}
	};

	// UI toggle methods
	toggleSavedCarts = () => {
		this.showSavedCarts = !this.showSavedCarts;
		// If showing saved carts, hide cart on mobile
		if (this.showSavedCarts) {
			this.showCartOnMobile = false;
		}
	};

	toggleCartOnMobile = () => {
		this.showCartOnMobile = !this.showCartOnMobile;
		// If showing cart, hide saved carts
		if (this.showCartOnMobile) {
			this.showSavedCarts = false;
		}
	};

	// Weight input methods
	handleEditWeight = (product: WeightedProduct, quantity: number) => {
		this.editingWeightItem = product;
		this.weightInputValue = quantity.toString();
	};

	confirmWeightInput = () => {
		if (!this.editingWeightItem) return;

		const weight = parseFloat(this.weightInputValue);
		if (isNaN(weight) || weight <= 0) {
			toast.error(m.pos_invalid_weight());
			return;
		}

		const existingItem = this.cart.find((item) => item.product.id === this.editingWeightItem?.id);

		if (existingItem) {
			this.cart = this.cart.map((item) =>
				item.product.id === this.editingWeightItem?.id ? { ...item, quantity: weight } : item
			);
		} else {
			this.cart = [...this.cart, { product: this.editingWeightItem, quantity: weight }];
		}
		this.localCart.current = this.cart;

		// Reset the state
		this.editingWeightItem = null;
		this.weightInputValue = '';
	};

	cancelWeightInput = () => {
		this.editingWeightItem = null;
		this.weightInputValue = '';
	};

	// Navigation
	navigateToHome = () => {
		goto('/');
	};
}

// Create and export a singleton instance
export const cartStore = new CartStore();

import { goto } from '$app/navigation';
import type { WeightedProduct, Product, SavedCart } from './types';
import { toast } from 'svelte-sonner';
import * as m from '$lib/paraglide/messages.js';

export class CartStore {
	// Cart state variables
	cart: { product: WeightedProduct | Product; quantity: number }[] = $state([]);
	savedCarts: {
		id: number;
		name: string;
		items: { product: WeightedProduct | Product; quantity: number }[];
		timestamp: Date;
	}[] = $state([]);

	// UI state variables
	searchQuery = $state('');
	newCartName = $state('');
	isSaving = $state(false);
	guestCount = $state(1);
	showSavedCarts = $state(false);
	showCartOnMobile = $state(false);

	// Weight input state
	editingWeightItem: WeightedProduct | null = $state(null);
	weightInputValue = $state('');

	// Computed values
	get total() {
		return this.cart.reduce((sum, item) => sum + item.product.price * item.quantity, 0);
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
	};

	deleteFromCart = (productId: number) => {
		this.cart = this.cart.filter((item) => item.product.id !== productId);
	};

	clearCart = () => {
		this.cart = [];
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
		}

		// Add the new cart
		this.savedCarts = [
			...this.savedCarts,
			{
				id: Date.now(),
				name: cartName,
				items: [...this.cart],
				timestamp: new Date()
			}
		];

		this.newCartName = '';
		this.isSaving = false;
		this.cart = [];
	};

	loadSavedCart = (savedCart: SavedCart) => {
		this.cart = [...savedCart.items];
		// Remove the loaded cart from saved carts
		this.deleteSavedCart(savedCart.id);

		if (this.cart.length === 0) {
			this.showSavedCarts = false;
		}
	};

	deleteSavedCart = (id: number) => {
		this.savedCarts = this.savedCarts.filter((cart) => cart.id !== id);

		if (this.savedCarts.length === 0) {
			this.guestCount = 1;
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

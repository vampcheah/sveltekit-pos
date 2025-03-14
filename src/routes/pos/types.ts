export interface Product {
	id: number;
	name: string;
	image?: string;
	price: number;
	unit?: string;
	category: string;
	isWeighted: false;
}

export interface WeightedProduct extends Omit<Product, 'isWeighted'> {
	isWeighted: true;
	unit: string;
}

export interface CartItem {
	product: WeightedProduct | Product;
	quantity: number;
}

export interface Cart {
	items: CartItem[];
	total: number;
}

export interface SavedCart {
	id: number;
	name: string;
	items: CartItem[];
	timestamp: string;
}

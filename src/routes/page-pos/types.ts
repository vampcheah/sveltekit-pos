export interface Product {
	id: number;
	name: string;
	image: string;
	price: number;
	unit?: string;
	category: string;
	isWeighed: boolean;
}

export interface CartItem {
	product: Product | WeightedProduct | boolean;
	quantity: number;
}

export interface Cart {
	items: CartItem[];
	total: number;
}

export interface WeightedProduct extends Product {
	weight: number;
	isWeighed: true;
	unit: string;
}

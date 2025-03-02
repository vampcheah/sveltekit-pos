import Dexie from 'dexie';
import DexieBase from './base';
import type { Category } from './categories/types';
import categorySchema from './categories/schema';
import type { Product } from './products/types';
import productSchema from './products/schema';

// Create our database class
class DPOSDatabase extends DexieBase {
	categories: Dexie.Table<Category, number>;
	products: Dexie.Table<Product, number>;

	constructor() {
		super('dPOSDatabase');

		// Define tables and their schemas
		this.version(1).stores({
			categories: categorySchema,
			products: productSchema
		});

		// Define typed tables
		this.categories = this.table('categories');
		this.products = this.table('products');
	}
}

// Create and export a single instance of the database
export const db = new DPOSDatabase();

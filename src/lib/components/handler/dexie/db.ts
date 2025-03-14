import Dexie, { type EntityTable } from 'dexie';
// categories
import type { Category } from './categories/types';
import categorySchema from './categories/schema';
// products
import type { Product } from './products/types';
import productSchema from './products/schema';

const db = new Dexie('dPOSDatabase') as Dexie & {
	categories: EntityTable<
		Category,
		'id' // primary key "id" (for the typings only)
	>;
	products: EntityTable<
		Product,
		'id' // primary key "id" (for the typings only)
	>;
};

// Schema declaration:
db.version(1).stores({
	categories: categorySchema,
	products: productSchema
});

export type { Category, Product };
export { db };

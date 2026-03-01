import { db } from '../db';
import { createCrudActions } from '../crud';
import { defaultProduct, type Product } from './types';

const baseCrud = createCrudActions(db.products, defaultProduct);

export const actions = {
	...baseCrud,

	// override add to include timestamps
	add: async (product: Product) => {
		const newProduct = {
			...defaultProduct,
			...product,
			createdAt: new Date(),
			updatedAt: new Date()
		};
		return await db.products.add(newProduct);
	},

	// override update to auto-set updatedAt
	update: async (id: number, changes: Partial<Product>) => {
		return await db.products.update(id, {
			...changes,
			updatedAt: new Date()
		});
	},

	// product-specific: get by category
	getByCategoryId: async (categoryId: number) => {
		return await db.products.where('categoryId').equals(categoryId).toArray();
	},

	// search by name
	search: async (query: string) => {
		return await db.products
			.filter((product) => product.name.toLowerCase().includes(query.toLowerCase()))
			.toArray();
	},

	// get products with joined category info
	getAllWithCategories: async () => {
		const products = await db.products.toArray();
		const categories = await db.categories.toArray();

		return products.map((product) => {
			const category = categories.find((c) => c.code === product.category) || null;
			return { ...product, category };
		});
	}
};

export default actions;

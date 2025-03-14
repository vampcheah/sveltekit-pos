import { db } from '../db';
import { defaultProduct, type Product } from './types';

// actions for products table
export const actions = {
	// get all products
	getAll: async () => {
		const products = await db.products.toArray();
		const columns = Object.entries(db.products.schema.indexes).map(([key, value]) => ({
			accessorKey: key,
			header: value.name
		}));

		const newProducts: Product[] = [];

		products.forEach((product) => {
			const productData = {
				...Object.entries(product).reduce(
					(data, [key, value]) => {
						const header = columns.find((column) => column.accessorKey === key)?.header;
						if (header) {
							data[header] = value;
						}
						return data;
					},
					{} as Record<string, string>
				),
				id: product.id
			};
			newProducts.push(productData as Product);
		});

		return newProducts;
	},

	// get product by id
	getById: async (id: number) => {
		return await db.products.get(id);
	},

	// get products by category id
	getByCategoryId: async (categoryId: number) => {
		return await db.products.where('categoryId').equals(categoryId).toArray();
	},

	// add product
	add: async (product: Product) => {
		const newProduct = {
			...defaultProduct,
			...product,
			createdAt: new Date(),
			updatedAt: new Date()
		};
		return await db.products.add(newProduct);
	},

	// update product
	update: async (id: number, changes: Partial<Product>) => {
		const updatedProduct = {
			...changes,
			updatedAt: new Date()
		};
		return await db.products.update(id, updatedProduct);
	},

	// delete product
	delete: async (id: number) => {
		return await db.products.delete(id);
	},

	// search product
	search: async (query: string) => {
		return await db.products
			.filter((product) => product.name.toLowerCase().includes(query.toLowerCase()))
			.toArray();
	},

	// get products with category information
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

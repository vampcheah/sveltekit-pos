import { db } from '../db';
import { defaultProduct, type Product } from './types';

// 产品操作
export const productActions = {
	// 获取所有产品
	getAll: async () => {
		return await db.products.toArray();
	},

	// 根据ID获取产品
	getById: async (id: number) => {
		return await db.products.get(id);
	},

	// 根据分类ID获取产品
	getByCategoryId: async (categoryId: number) => {
		return await db.products.where('categoryId').equals(categoryId).toArray();
	},

	// 添加产品
	add: async (product: Product) => {
		const newProduct = {
			...defaultProduct,
			...product,
			createdAt: new Date(),
			updatedAt: new Date()
		};
		return await db.products.add(newProduct);
	},

	// 更新产品
	update: async (id: number, changes: Partial<Product>) => {
		const updatedProduct = {
			...changes,
			updatedAt: new Date()
		};
		return await db.products.update(id, updatedProduct);
	},

	// 删除产品
	delete: async (id: number) => {
		return await db.products.delete(id);
	},

	// 搜索产品
	search: async (query: string) => {
		return await db.products
			.filter(
				(product) =>
					product.name.toLowerCase().includes(query.toLowerCase()) ||
					product.description.toLowerCase().includes(query.toLowerCase()) ||
					product.barcode.includes(query) ||
					product.sku.includes(query)
			)
			.toArray();
	},

	// 更新库存
	updateStock: async (id: number, quantity: number) => {
		const product = await db.products.get(id);
		if (!product) throw new Error('产品不存在');

		return await db.products.update(id, {
			stock: product.stock + quantity,
			updatedAt: new Date()
		});
	},

	// 获取低库存产品
	getLowStock: async (threshold = 10) => {
		return await db.products.where('stock').below(threshold).toArray();
	},

	// 获取带有分类信息的产品
	getAllWithCategories: async () => {
		const products = await db.products.toArray();
		const categories = await db.categories.toArray();

		return products.map((product) => {
			const category = categories.find((c) => c.id === product.categoryId) || null;
			return { ...product, category };
		});
	}
};

export default productActions;

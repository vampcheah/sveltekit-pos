import { db } from '../db';
import { defaultCategory, type Category } from './types';
// 分类操作
export const categoryActions = {
	// 获取所有分类
	getAll: async () => {
		return await db.categories.toArray();
	},

	// 根据ID获取分类
	getById: async (id: number) => {
		return await db.categories.get(id);
	},

	// 添加分类
	add: async (category: Category) => {
		const newCategory = {
			...defaultCategory,
			...category,
			createdAt: new Date(),
			updatedAt: new Date()
		};
		return await db.categories.add(newCategory);
	},

	// 更新分类
	update: async (id: number, changes: Partial<Category>) => {
		const updatedCategory = {
			...changes,
			updatedAt: new Date()
		};
		return await db.categories.update(id, updatedCategory);
	},

	// 删除分类
	delete: async (id: number) => {
		// 检查是否有产品使用此分类
		const productsWithCategory = await db.products.where('categoryId').equals(id).count();
		if (productsWithCategory > 0) {
			throw new Error('无法删除此分类，因为有产品正在使用它');
		}
		return await db.categories.delete(id);
	},

	// 搜索分类
	search: async (query: string) => {
		return await db.categories
			.filter(
				(category) =>
					category.name.toLowerCase().includes(query.toLowerCase()) ||
					category.description.toLowerCase().includes(query.toLowerCase())
			)
			.toArray();
	}
};

export default categoryActions;

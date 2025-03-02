import { db } from './db';
import type { Category } from './categories/types';

// 数据库初始化和种子数据
export const initializeDatabase = async () => {
	const categoryCount = await db.categories.count();
	const productCount = await db.products.count();

	// 如果数据库为空，添加一些示例数据
	if (categoryCount === 0) {
		await db.categories.bulkAdd([
			{ name: '饮料', description: '各种饮料', createdAt: new Date(), updatedAt: new Date() },
			{ name: '食品', description: '各种食品', createdAt: new Date(), updatedAt: new Date() },
			{ name: '日用品', description: '日常用品', createdAt: new Date(), updatedAt: new Date() }
		]);
	}

	if (productCount === 0) {
		// 获取刚添加的分类
		const categories = await db.categories.toArray();
		const beverageCategoryId = categories.find((c: Category) => c.name === '饮料')?.id;
		const foodCategoryId = categories.find((c: Category) => c.name === '食品')?.id;

		if (beverageCategoryId && foodCategoryId) {
			await db.products.bulkAdd([
				{
					name: '可乐',
					categoryId: beverageCategoryId,
					price: 3.5,
					cost: 2.0,
					barcode: '6901234567890',
					sku: 'BEV001',
					description: '330ml 罐装可乐',
					imageUrl: '/images/cola.jpg',
					stock: 100,
					createdAt: new Date(),
					updatedAt: new Date()
				},
				{
					name: '矿泉水',
					categoryId: beverageCategoryId,
					price: 2.0,
					cost: 0.8,
					barcode: '6901234567891',
					sku: 'BEV002',
					description: '550ml 瓶装矿泉水',
					imageUrl: '/images/water.jpg',
					stock: 200,
					createdAt: new Date(),
					updatedAt: new Date()
				},
				{
					name: '面包',
					categoryId: foodCategoryId,
					price: 5.0,
					cost: 2.5,
					barcode: '6901234567892',
					sku: 'FOOD001',
					description: '全麦面包',
					imageUrl: '/images/bread.jpg',
					stock: 50,
					createdAt: new Date(),
					updatedAt: new Date()
				}
			]);
		}
	}
};

export default initializeDatabase;

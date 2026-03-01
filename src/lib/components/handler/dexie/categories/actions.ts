import { db } from '../db';
import { createCrudActions } from '../crud';
import { defaultCategory, type Category } from './types';

const baseCrud = createCrudActions(db.categories, defaultCategory);

export const actions = {
	...baseCrud,

	// override delete with referential integrity check
	delete: async (id: number) => {
		const productsWithCategory = await db.products.where('categoryId').equals(id).count();
		if (productsWithCategory > 0) {
			throw new Error('Cannot delete category, products are using it');
		}
		return await db.categories.delete(id);
	},

	// search by code or name
	search: async (query: string) => {
		return await db.categories
			.filter(
				(category) =>
					category.code.toLowerCase().includes(query.toLowerCase()) ||
					category.name.toLowerCase().includes(query.toLowerCase())
			)
			.toArray();
	}
};

export default actions;

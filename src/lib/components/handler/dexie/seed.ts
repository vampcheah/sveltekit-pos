import { db } from './db';
import { categories as categoriesData, products as productsData } from './sample_data';

// import sample data
export const importSampleData = async () => {
	try {
		// Ensure database is open and ready
		if (!db.isOpen()) {
			await db.open();
		}

		// if database is not empty, clear it
		await db.categories.clear();
		await db.categories.bulkAdd(categoriesData);

		await db.products.clear();
		await db.products.bulkAdd(productsData);

		return { success: true };
	} catch (error: unknown) {
		console.error('Error importing sample data:', error);
		// Try to reopen the database if that was the issue
		if (error instanceof Error && error.message.includes('reopen')) {
			db.close();
			db.open();
			// You could retry the import here if needed
		}
		return { success: false, error };
	}
};

export default importSampleData;

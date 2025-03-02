import Dexie from 'dexie';

/**
 * Base Dexie database class for the dPOS application
 * This class provides the foundation for all database operations
 */
class DexieBase extends Dexie {
	constructor(databaseName: string) {
		super(databaseName);
	}

	/**
	 * Clears all data from the database
	 */
	async clearAllData(): Promise<void> {
		return this.transaction('rw', this.tables, async () => {
			await Promise.all(this.tables.map((table) => table.clear()));
		});
	}

	/**
	 * Deletes the entire database
	 */
	async deleteDatabase(): Promise<void> {
		await this.delete();
	}

	/**
	 * Checks if the database is empty
	 * @returns boolean indicating if all tables are empty
	 */
	async isDatabaseEmpty(): Promise<boolean> {
		const counts = await Promise.all(
			this.tables.map(async (table) => {
				return await table.count();
			})
		);

		return counts.every((count) => count === 0);
	}
}

export default DexieBase;

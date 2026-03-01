import type { EntityTable, IDType, InsertType, UpdateSpec } from 'dexie';

export interface CrudActions<T> {
	getAll: () => Promise<T[]>;
	getById: (id: number) => Promise<T | undefined>;
	getCount: () => Promise<number>;
	add: (item: T) => Promise<number>;
	update: (id: number, changes: Partial<T>) => Promise<number>;
	delete: (id: number) => Promise<void>;
}

export function createCrudActions<T extends { id?: number }>(
	table: EntityTable<T, 'id'>,
	defaults: Omit<T, 'id'>
): CrudActions<T> {
	type Id = IDType<T, 'id'>;
	return {
		getAll: async () => table.toArray(),
		getById: async (id: number) => table.get(id as Id),
		getCount: async () => table.count(),
		add: async (item: T) => table.add({ ...defaults, ...item } as T) as Promise<number>,
		update: async (id: number, changes: Partial<T>) =>
			table.update(id as Id, changes as UpdateSpec<InsertType<T, 'id'>>),
		delete: async (id: number) => {
			await table.delete(id as Id);
		}
	};
}

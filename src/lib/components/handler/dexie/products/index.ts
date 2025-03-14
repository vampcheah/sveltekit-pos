import { schema } from './schema';
import { actions } from './actions';
import type { Product } from './types';

export const tableName = 'products';
export { schema, actions };
export type { Product };

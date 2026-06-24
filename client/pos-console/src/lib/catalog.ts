// 商品/分类数据：从后端取，映射成收银台 UI 用的形状（替代原 Dexie 数据源）。
import { api } from '$lib/api';

export interface UiCategory {
	name: string;
	code: string;
}
export interface UiProduct {
	id: number;
	name: string;
	category: string; // 分类 code
	price: number;
	image: string;
	isWeighted: boolean;
}

interface ApiCategory {
	id: number;
	code: string;
	name: string;
}
interface ApiProduct {
	id: number;
	name: string;
	category_id: number | null;
	price: string;
	image_url: string | null;
	is_weighted: boolean;
}

export async function getCategories(): Promise<UiCategory[]> {
	const cats = await api.get<ApiCategory[]>('/categories');
	return cats.map((c) => ({ name: c.name, code: c.code }));
}

export async function getProducts(): Promise<UiProduct[]> {
	const [cats, products] = await Promise.all([
		api.get<ApiCategory[]>('/categories'),
		api.get<ApiProduct[]>('/products')
	]);
	const codeById = new Map(cats.map((c) => [c.id, c.code]));
	return products.map((p) => ({
		id: p.id,
		name: p.name,
		category: p.category_id != null ? (codeById.get(p.category_id) ?? '') : '',
		price: Number(p.price),
		image: p.image_url ?? 'https://placehold.co/100x100',
		isWeighted: p.is_weighted
	}));
}

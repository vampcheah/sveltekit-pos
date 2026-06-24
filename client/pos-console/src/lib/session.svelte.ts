// 收银员会话：登录（username+PIN）→ 解析主店 + 默认仓库；登出。
// 会话本身是后端 HttpOnly cookie；这里只缓存展示态 + 结账要用的 store/warehouse。
import { api, ApiError } from '$lib/api';
import * as m from '$lib/paraglide/messages.js';

interface Me {
	actor_type: string;
	actor_id: number;
	store_id: number | null;
	is_supervisor: boolean;
}
interface Warehouse {
	id: number;
	store_id: number;
}

export const cashier = $state<{
	loggedIn: boolean;
	actorId: number | null;
	storeId: number | null;
	warehouseId: number | null;
	isSupervisor: boolean;
}>({
	loggedIn: false,
	actorId: null,
	storeId: null,
	warehouseId: null,
	isSupervisor: false
});

async function hydrate(me: Me) {
	cashier.loggedIn = me.actor_type === 'cashier';
	cashier.actorId = me.actor_id;
	cashier.storeId = me.store_id;
	cashier.isSupervisor = me.is_supervisor;
	// 默认仓库 = 本店第一个仓库（结账扣库存用）
	if (me.store_id != null) {
		const whs = await api.get<Warehouse[]>(`/warehouses?store_id=${me.store_id}`);
		cashier.warehouseId = whs[0]?.id ?? null;
	}
}

export async function login(username: string, pin: string): Promise<{ ok: boolean; error?: string }> {
	try {
		const me = await api.post<Me>('/auth/cashier/login', { username, pin });
		await hydrate(me);
		return { ok: true };
	} catch (e) {
		return { ok: false, error: e instanceof ApiError ? m.pos_login_invalid() : m.pos_login_neterr() };
	}
}

/** 刷新页面后用 cookie 恢复会话。 */
export async function restore(): Promise<boolean> {
	try {
		const me = await api.get<Me>('/auth/me');
		if (me.actor_type !== 'cashier') return false;
		await hydrate(me);
		return true;
	} catch {
		return false;
	}
}

export async function logout(): Promise<void> {
	try {
		await api.post('/auth/logout');
	} finally {
		cashier.loggedIn = false;
		cashier.actorId = null;
		cashier.storeId = null;
		cashier.warehouseId = null;
		cashier.isSupervisor = false;
	}
}

// ============================================================================
//  Auth provider — THE single seam between the app and your auth backend.
//  接入 pos-server：服务端 Session（HttpOnly cookie），登录用 username + password。
//  `email` 形参在本项目里承载的是用户名（登录页已改为用户名输入）。
// ============================================================================

import type { User } from './types';
import { config } from '$lib/config';

export interface AuthResult {
	ok: boolean;
	error?: string;
	user?: User;
}

export interface AuthProvider {
	login(email: string, password: string): Promise<AuthResult>;
	register(name: string, email: string, password: string): Promise<AuthResult>;
	logout(): Promise<void>;
	/** 校验当前会话 cookie，返回用户或 null（刷新页面后恢复真实会话）。 */
	me?(): Promise<User | null>;
}

const BASE = config.api.baseUrl;

// pos-server /auth/me 的响应形状
interface MeResp {
	actor_type: string;
	actor_id: number;
	username: string | null;
	full_name: string | null;
	avatar_url: string | null;
	role_id: number | null;
	permissions: string[];
}

function toUser(fallbackName: string, me: MeResp): User {
	const username = me.username ?? fallbackName;
	return {
		id: String(me.actor_id),
		name: me.full_name || username,
		email: username,
		role: 'admin',
		avatarUrl: me.avatar_url ?? undefined,
		permissions: me.permissions ?? []
	};
}

export const apiAuthProvider: AuthProvider = {
	async login(username, password) {
		const res = await fetch(`${BASE}/auth/admin/login`, {
			method: 'POST',
			credentials: 'include',
			headers: { 'content-type': 'application/json' },
			body: JSON.stringify({ username, password })
		});
		if (!res.ok) return { ok: false, error: '用户名或密码错误' };
		const me = (await res.json()) as MeResp;
		return { ok: true, user: toUser(username, me) };
	},

	async register() {
		// 管理员由其它管理员创建，不开放自助注册。
		return { ok: false, error: '请联系管理员开通账号' };
	},

	async logout() {
		await fetch(`${BASE}/auth/logout`, { method: 'POST', credentials: 'include' });
	},

	async me() {
		const res = await fetch(`${BASE}/auth/me`, { credentials: 'include' });
		if (!res.ok) return null;
		const me = (await res.json()) as MeResp;
		if (me.actor_type !== 'admin') return null;
		return toUser('admin', me); // 仅用于校验会话有效；显示名取自登录时的缓存
	}
};

export const authProvider: AuthProvider = apiAuthProvider;

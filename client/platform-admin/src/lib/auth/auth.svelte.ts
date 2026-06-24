// Reactive auth store. Holds the current user + loading state, restores a cached
// session synchronously for instant UI (and so the route guard works on first
// paint), and delegates the actual authentication to `authProvider`.
//
// The swap point for a real backend is provider.ts — not this file. This store's
// public API (`auth.user`, `auth.login`, …) stays stable, so pages never change.

import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { resolve } from '$app/paths';
import type { User } from './types';
import { authProvider } from './provider';
import { config } from '$lib/config';

const SESSION_KEY = config.auth.sessionKey;

class AuthStore {
	#user = $state<User | null>(null);
	#loading = $state(false);
	#initialized = false;

	get user(): User | null {
		return this.#user;
	}

	get isAuthenticated(): boolean {
		return this.#user !== null;
	}

	get loading(): boolean {
		return this.#loading;
	}

	/** 当前用户是否拥有某权限码。 */
	has(permission: string): boolean {
		return this.#user?.permissions?.includes(permission) ?? false;
	}

	/** 用最新资料覆盖当前用户（改头像/姓名后调用）。 */
	setUser(user: User): void {
		this.#persist(user);
	}

	/** Restore a cached session from localStorage. Safe to call repeatedly. */
	init(): void {
		if (this.#initialized || !browser) return;
		this.#initialized = true;
		try {
			const raw = localStorage.getItem(SESSION_KEY);
			if (raw) this.#user = JSON.parse(raw) as User;
		} catch {
			this.#user = null;
		}
	}

	/** 用真实会话 cookie 校验缓存（/auth/me）。失效则清空（守卫会回登录）。 */
	async revalidate(): Promise<void> {
		if (!browser || !authProvider.me) return;
		try {
			const me = await authProvider.me();
			if (!me) this.#clear();
			else if (!this.#user) this.#persist(me);
		} catch {
			// 网络异常保留缓存，避免误登出
		}
	}

	/** Cache the session so `init()` can restore it instantly on the next load. */
	#persist(user: User): void {
		this.#user = user;
		if (browser) {
			try {
				localStorage.setItem(SESSION_KEY, JSON.stringify(user));
			} catch {
				// Storage may be unavailable (private mode); session stays in-memory.
			}
		}
	}

	#clear(): void {
		this.#user = null;
		if (browser) {
			try {
				localStorage.removeItem(SESSION_KEY);
			} catch {
				// Ignore storage errors.
			}
		}
	}

	async login(email: string, password: string): Promise<{ ok: boolean; error?: string }> {
		this.#loading = true;
		try {
			const result = await authProvider.login(email, password);
			if (result.ok && result.user) this.#persist(result.user);
			return { ok: result.ok, error: result.error };
		} finally {
			this.#loading = false;
		}
	}

	async register(
		name: string,
		email: string,
		password: string
	): Promise<{ ok: boolean; error?: string }> {
		this.#loading = true;
		try {
			const result = await authProvider.register(name, email, password);
			if (result.ok && result.user) this.#persist(result.user);
			return { ok: result.ok, error: result.error };
		} finally {
			this.#loading = false;
		}
	}

	/** Sign out (revoke via the provider), clear the cached session, return to login. */
	async logout(): Promise<void> {
		try {
			await authProvider.logout();
		} finally {
			this.#clear();
			void goto(resolve(config.auth.afterLogout));
		}
	}
}

export const auth = new AuthStore();

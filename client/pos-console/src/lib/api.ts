// 后端 API 客户端：带 cookie 会话（credentials: include）。组件不直接 fetch。
import { env } from '$env/dynamic/public';

const BASE = env.PUBLIC_API_URL ?? 'http://localhost:8080/api/v1';

export class ApiError extends Error {
	constructor(
		public status: number,
		message: string
	) {
		super(message);
	}
}

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
	const res = await fetch(`${BASE}${path}`, {
		method,
		credentials: 'include',
		headers: body !== undefined ? { 'content-type': 'application/json' } : {},
		body: body !== undefined ? JSON.stringify(body) : undefined
	});
	if (!res.ok) {
		let message = res.statusText;
		try {
			const j = await res.json();
			message = j.message ?? j.error ?? message;
		} catch {
			/* ignore */
		}
		throw new ApiError(res.status, message);
	}
	if (res.status === 204) return undefined as T;
	return (await res.json()) as T;
}

export const api = {
	get: <T>(path: string) => request<T>('GET', path),
	post: <T>(path: string, body?: unknown) => request<T>('POST', path, body)
};

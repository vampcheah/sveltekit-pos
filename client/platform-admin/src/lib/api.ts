// 后端 API 客户端：带 cookie 会话（credentials: include），统一错误。
// 所有数据页/动作都经此，组件不直接 fetch（§1.2）。
import { config } from '$lib/config';

const BASE = config.api.baseUrl;

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
		credentials: 'include', // 携带/接收会话 cookie
		headers: body !== undefined ? { 'content-type': 'application/json' } : {},
		body: body !== undefined ? JSON.stringify(body) : undefined
	});
	if (!res.ok) {
		let message = res.statusText;
		try {
			const j = await res.json();
			message = j.message ?? j.error ?? message;
		} catch {
			/* 非 JSON 错误体 */
		}
		throw new ApiError(res.status, message);
	}
	if (res.status === 204) return undefined as T;
	const ct = res.headers.get('content-type') ?? '';
	return (ct.includes('application/json') ? await res.json() : await res.text()) as T;
}

export const api = {
	get: <T>(path: string) => request<T>('GET', path),
	post: <T>(path: string, body?: unknown) => request<T>('POST', path, body),
	put: <T>(path: string, body?: unknown) => request<T>('PUT', path, body),
	patch: <T>(path: string, body?: unknown) => request<T>('PATCH', path, body),
	del: <T>(path: string) => request<T>('DELETE', path)
};

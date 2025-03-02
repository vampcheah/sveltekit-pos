import { project } from '$lib';
import { goto } from '$app/navigation';
import { login_user } from '$lib/state.svelte';

export const apiSend = async (
	withToken: boolean,
	method: string,
	resource: string,
	data?: unknown,
	host?: string
) => {
	method = method.toUpperCase();

	const apiToken = login_user.loginAccessToken;
	const headers = {
		'Content-Type': 'application/json',
		Accept: 'application/json',
		...(withToken && apiToken ? { Authorization: `Bearer ${apiToken}` } : {})
	};

	const url = new URL(`${host || project.appUrl}${resource}`);
	if (method === 'GET' && data) {
		// Properly serialize complex objects for GET requests
		Object.entries(data).forEach(([key, value]) => {
			const serializedValue = typeof value === 'object' ? JSON.stringify(value) : String(value);
			url.searchParams.append(key, serializedValue);
		});
	}

	try {
		const response = await fetch(url.toString(), {
			method,
			headers,
			credentials: 'include', // Include cookies if needed
			mode: 'cors', // Ensure CORS is enabled in the request
			body: method !== 'GET' && data ? JSON.stringify(data) : undefined
		});

		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}

		const result = await response.json();

		if ((withToken && [403, 500].includes(result.payload)) || result.payload === '901') {
			goto('/');
		}

		return result;
	} catch (error) {
		console.error('API Error:', error);
		return {
			ok: false,
			error: error instanceof Error ? error.message : String(error),
			details: error instanceof Error ? error.stack : undefined
		};
	}
};

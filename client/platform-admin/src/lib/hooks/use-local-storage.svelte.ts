// Reactive localStorage helper. Returns a runes-backed `{ current }` container
// that reads the persisted value on the client and writes back on every change.
// SSR-safe: on the server it falls back to `initial` and never touches storage.

import { browser } from '$app/environment';

/**
 * Persist a reactive value in `localStorage` under `key`.
 * Assign to `.current` to update both the reactive state and storage.
 *
 * @example
 * const theme = useLocalStorage('admin-starter:theme', 'system');
 * theme.current = 'dark';
 */
export function useLocalStorage<T>(key: string, initial: T): { current: T } {
	// Read any existing value synchronously so the first render is correct.
	let value = $state<T>(read());

	function read(): T {
		if (!browser) return initial;
		try {
			const raw = localStorage.getItem(key);
			return raw === null ? initial : (JSON.parse(raw) as T);
		} catch {
			// Corrupt or unreadable entry — fall back to the initial value.
			return initial;
		}
	}

	$effect(() => {
		// Persist whenever the value changes (client only).
		if (!browser) return;
		try {
			localStorage.setItem(key, JSON.stringify(value));
		} catch {
			// Ignore quota / serialization errors — storage is best-effort.
		}
	});

	return {
		get current() {
			return value;
		},
		set current(next: T) {
			value = next;
		}
	};
}

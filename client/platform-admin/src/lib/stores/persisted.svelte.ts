// Reusable runes-based persisted state. Reads from localStorage on init and
// writes back whenever `current` changes. SSR-safe via the `browser` guard.
//
// Usage:
//   const theme = persisted('admin-starter:theme', 'system');
//   theme.current = 'dark';   // automatically persisted

import { browser } from '$app/environment';

export function persisted<T>(key: string, initial: T): { current: T } {
	let value = $state<T>(initial);

	if (browser) {
		try {
			const raw = localStorage.getItem(key);
			if (raw !== null) value = JSON.parse(raw) as T;
		} catch {
			// Corrupt/unavailable storage — fall back to the initial value.
		}
	}

	return {
		get current(): T {
			return value;
		},
		set current(next: T) {
			value = next;
			if (browser) {
				try {
					localStorage.setItem(key, JSON.stringify(next));
				} catch {
					// Ignore quota/availability errors.
				}
			}
		}
	};
}

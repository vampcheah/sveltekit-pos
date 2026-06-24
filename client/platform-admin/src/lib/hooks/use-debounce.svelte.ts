// Reactive debounce hook. Tracks a reactive source via `getValue` and exposes
// `current`, which only updates after `delayMs` of quiet time.

/**
 * Debounce a reactive value. Pass a thunk that reads reactive state so the
 * hook re-runs whenever the source changes; the returned `current` settles to
 * the latest value after `delayMs` milliseconds without further changes.
 *
 * @example
 * let search = $state('');
 * const debounced = useDebounce(() => search, 300);
 * // read `debounced.current` in a $derived/$effect
 */
export function useDebounce<T>(getValue: () => T, delayMs = 300): { readonly current: T } {
	let current = $state(getValue());

	$effect(() => {
		// Establish the reactive dependency on the source value.
		const next = getValue();
		const timer = setTimeout(() => {
			current = next;
		}, delayMs);
		return () => clearTimeout(timer);
	});

	return {
		get current() {
			return current;
		}
	};
}

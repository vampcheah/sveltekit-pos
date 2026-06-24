// Reactive media-query hook built on Svelte's reactive `MediaQuery`, which is
// SSR-safe and updates automatically when the query result changes.

import { MediaQuery } from 'svelte/reactivity';

/**
 * Track whether a CSS media query currently matches.
 *
 * @example
 * const wide = useMediaQuery('(min-width: 1024px)');
 * // read `wide.matches` in markup or a $derived
 */
export function useMediaQuery(query: string): { readonly matches: boolean } {
	const mq = new MediaQuery(query);
	return {
		get matches() {
			return mq.current;
		}
	};
}

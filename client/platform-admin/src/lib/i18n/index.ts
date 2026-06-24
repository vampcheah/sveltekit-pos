// Barrel so consumers can import from `$lib/i18n`. The implementation lives in
// `index.svelte.ts` (a runes module); a bare directory import does not resolve
// `.svelte.ts`, so this thin re-export bridges the two.
export { t, setLocale, initLocale, i18n, LOCALES } from './index.svelte';
export type { Locale } from './index.svelte';

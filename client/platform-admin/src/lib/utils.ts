import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * Merge Tailwind classes with conditional `clsx` syntax, de-duplicating
 * conflicting utilities. This is the canonical helper used by every
 * shadcn-svelte component — do not remove it.
 */
export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// --- Type helpers used by shadcn-svelte components -------------------------
export type WithoutChild<T> = T extends { child?: unknown } ? Omit<T, 'child'> : T;
export type WithoutChildren<T> = T extends { children?: unknown } ? Omit<T, 'children'> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };

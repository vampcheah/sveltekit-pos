// ============================================================================
//  App configuration — the SINGLE place to configure & rebrand the starter.
//
//  Edit this one file and the whole app updates: name & logo, default theme and
//  language, auth behaviour (session key, redirects, demo credentials), and a
//  few UI defaults. Read it anywhere with `import { config } from '$lib/config'`.
//
//  Two things live elsewhere by design:
//    • Design tokens (colors, fonts) → src/app.css
//    • Auth / data backends (the "how", not the "what") → $lib/auth/provider.ts
//      and $lib/server/db.ts
// ============================================================================

import type { Locale } from '$lib/i18n';
import { env } from '$env/dynamic/public';

export const config = {
	/** Backend connection — shared by BOTH the auth provider and the data layer.
	 *  This is "where the backend is"; auth strategy ("how") lives in `auth` below. */
	api: {
		/** Base URL of your backend API. Empty string = mock mode (no backend).
		 *  When you wire a real AuthProvider ($lib/auth/provider.ts) and/or the db
		 *  ($lib/server/db.ts), point them at this. In production you'll usually
		 *  source it from an env var (e.g. PUBLIC_API_URL via $env/dynamic/public)
		 *  rather than hardcoding it here. */
		baseUrl: env.PUBLIC_API_URL ?? 'http://localhost:8080/api/v1'
	},

	/** Brand identity — shown in the sidebar, auth screens, document titles. */
	app: {
		name: 'Admin Starter',
		/** Square logo glyph (swap the markup for an <img>/SVG when you have one). */
		logo: 'A',
		description: 'A SvelteKit admin dashboard starter',
		/** Headline shown on the split-screen auth pages. */
		tagline: 'Everything you need to ship an admin.'
	},

	/** Appearance defaults. The brand color is the `--primary` token in app.css. */
	theme: {
		/** Initial color mode before the user picks one. */
		defaultMode: 'system' as 'light' | 'dark' | 'system'
	},

	/** Localization. Dictionaries live in $lib/i18n/locales. */
	i18n: {
		defaultLocale: 'zh-CN' as Locale
	},

	/** Authentication strategy. The backend address lives in `api.baseUrl` above;
	 *  the provider implementation is $lib/auth/provider.ts. */
	auth: {
		/** localStorage key for the cached session. */
		sessionKey: 'admin-starter:session',
		minPasswordLength: 8,
		/** Where to send users after sign-in and sign-out. */
		afterLogin: '/dashboard',
		afterLogout: '/login',
		/** Credentials shown (and click-to-fill) on the login screen. */
		demo: { email: 'admin', password: 'admin123' }
	},

	/** Misc UI defaults. */
	ui: {
		/** Default rows per page for <DataTable>. */
		pageSize: 10
	}
} as const;

export type AppConfig = typeof config;

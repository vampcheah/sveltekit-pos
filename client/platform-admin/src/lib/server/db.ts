// ============================================================================
//  Data access seam (server-only) — THE single place to swap mock data for a
//  real database / API.
//
//  Real feature pages read their data through this `db` object from a
//  `+page.server.ts` `load` function, e.g.:
//
//      // src/routes/(app)/tables/+page.server.ts
//      import { db } from '$lib/server/db';
//      export const load = async () => ({ products: await db.products.list() });
//
//  Every method currently returns the in-memory mock fixtures from
//  `$lib/data/*`. To go to production, replace the method bodies (keep the
//  signatures) with your real queries — pages that consume `db` won't change.
//
//  This file lives in `$lib/server/`, so it — and any DB credentials it
//  imports — is NEVER bundled into the browser.
// ============================================================================

import { config } from '$lib/config';
import { demoUsers, type DemoUser } from '$lib/data/users';
import { demoProducts, type DemoProduct } from '$lib/data/products';
import { stats, revenueSeries, recentActivity, trafficByChannel } from '$lib/data/dashboard';

//  Base URL of your backend (empty string = mock mode). Replace the mock bodies
//  below with real queries/fetches against it — `products.list` shows the shape.
const API_BASE_URL = config.api.baseUrl;

export const db = {
	users: {
		list: async (): Promise<DemoUser[]> => demoUsers,
		get: async (id: string): Promise<DemoUser | undefined> => demoUsers.find((u) => u.id === id)
	},
	products: {
		list: async (): Promise<DemoProduct[]> => {
			//  When a backend is configured, fetch from it; otherwise serve mock data.
			if (API_BASE_URL) {
				const res = await fetch(`${API_BASE_URL}/products`);
				return (await res.json()) as DemoProduct[];
			}
			return demoProducts;
		}
	},
	dashboard: {
		summary: async () => ({ stats, revenueSeries, recentActivity, trafficByChannel })
	}
};

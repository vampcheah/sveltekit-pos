# Repo conventions (reference)

The rules the template relies on, grouped by concern. The SKILL.md workflow
points here for detail. When in doubt, open the cited example file — the code is
the source of truth.

## Routing & route groups

- Authenticated app pages live under the `(app)` group:
  `src/routes/(app)/<name>/+page.svelte`. The `(app)/+layout.svelte` renders the
  admin shell (sidebar + header) and guards the route (redirects to `/login`
  when not authenticated).
- Auth pages (login/register/forgot-password) live under `(auth)`.
- Route groups `(app)` / `(auth)` are organizational — they do **not** appear in
  the URL. `src/routes/(app)/orders/+page.svelte` serves `/orders`.
- Dynamic segment for detail pages: `src/routes/(app)/<name>/[id]/+page.svelte`
  serves `/<name>/:id`; read the id from `page.params.id` or the load function.
- Nested sections can add their own `+layout.svelte` (see `settings/`).

## Navigation — always `resolve()`

`eslint` enforces `svelte/no-navigation-without-resolve`. Every internal
navigation target must be wrapped:

```svelte
<script lang="ts">
	import { resolve } from '$app/paths';
	import { goto } from '$app/navigation';
</script>

<a href={resolve('/orders')}>Orders</a>
<button onclick={() => goto(resolve('/dashboard'))}>Go</button>

<!-- dynamic -->
<a href={resolve(`/users/${row.id}`)}>{row.name}</a>
```

If a value is a plain `string` (not a literal route), `resolve()` won't type-check.
Type the source as a route id: `import type { Pathname } from '$app/types'` and
annotate the field `href: Pathname` (this is exactly what `nav.ts` does).

External links (`http…`) must NOT be wrapped — for a component that accepts
arbitrary hrefs, the rule is disabled at the config level for
`src/lib/components/ui/**` (generated shadcn primitives). Don't re-enable it there.

## Sidebar registration (`src/lib/shell/nav.ts`)

To make a page appear in the sidebar, add a `NavItem` to the right `NavGroup`:

```ts
import type { Pathname } from '$app/types';
import Receipt from '@lucide/svelte/icons/receipt';

// inside the relevant group's items:
{ title: 'Orders', href: '/orders', icon: Receipt }
```

- `NavItem = { title: string; href: Pathname; icon: Component; badge?: string | number }`.
- `href` is `Pathname`, so the route must exist (create the `+page.svelte` first,
  then run `npm run check`/dev once so SvelteKit regenerates `$app/types`).
- Icons come from `@lucide/svelte/icons/<name>` (default import).
- Groups: Overview, Management, Apps, Commerce, Showcase, Billing, Account — pick
  the closest, or add a new group object.
- Active-state highlighting and breadcrumb lookups are derived from `nav.ts`
  automatically (`findNavItem`), so registering is all that's needed.

## Data — the server-only seam

Real/dynamic data flows: `+page.server.ts` `load` → `$lib/server/db` → `$lib/data/*`.

1. Add a method to `src/lib/server/db.ts` (keep the mock body or call a real
   backend via `config.api.baseUrl`):
   ```ts
   orders: {
   	list: async (): Promise<DemoOrder[]> => demoOrders;
   }
   ```
2. Load it in `src/routes/(app)/orders/+page.server.ts`:
   ```ts
   import { db } from '$lib/server/db';
   import type { PageServerLoad } from './$types';
   export const load: PageServerLoad = async () => ({ orders: await db.orders.list() });
   ```
3. Read it in the page:
   ```svelte
   <script lang="ts">
   	let { data } = $props();
   </script>

   {#each data.orders as order (order.id)}…{/each}
   ```

Rules:

- **Never** `import … from '$lib/server/...'` inside a `.svelte` file — `$lib/server`
  is server-only; importing it client-side leaks code/credentials and breaks the build.
- Detail load reads the param: `async ({ params }) => ({ user: await db.users.get(params.id) })`.
  Throw `error(404, …)` from `@sveltejs/kit` when not found.
- Mock fixtures live in `src/lib/data/*` (icon-free plain data + a `Demo*` type).
- Pages that only manipulate local UI state (like `users/+page.svelte`'s CRUD over
  a `$state` copy) need no server load — that's a valid choice for demo/local flows.

Reference example: `src/routes/(app)/tables/+page.server.ts` and `src/lib/server/db.ts`.

## Internationalization

- Translate user-facing text: `import { t } from '$lib/i18n'` then `t('orders.title')`.
  Keys are nested dot-paths.
- Add every new key to **both** `src/lib/i18n/locales/en.ts` and `zh-CN.ts`,
  mirroring the object shape exactly. A missing locale falls back to the raw key
  (a visible bug).
- Interpolation: `t('dashboard.greeting', { name })` with `{name}` in the string.
- The active locale is reactive; switching is handled by `LanguageToggle`.

## Shared components (`$lib/components/shared`)

Build pages from these, not raw markup:

| Component                                                 | Purpose                                                                                                             |
| --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `PageContainer`                                           | Standard centered wrapper (max-width, padding, vertical rhythm). Wrap every page in it.                             |
| `PageHeader`                                              | Title + optional `description` + right-aligned `actions` snippet.                                                   |
| `DataTable`                                               | Generic table: search, sort, selection, custom cell snippets, per-row actions, pagination. Pair with `type Column`. |
| `StatCard`                                                | KPI tile (label, value, delta, icon).                                                                               |
| `StatusBadge`                                             | Status pill; tones via `type BadgeTone`.                                                                            |
| `EmptyState`                                              | Empty/zero-results placeholder.                                                                                     |
| `ConfirmDialog`                                           | Destructive-action confirmation (`open` bindable, `onConfirm`).                                                     |
| `SearchInput`, `Spinner`, `ThemeToggle`, `LanguageToggle` | Misc building blocks.                                                                                               |

Lower-level shadcn primitives live in `$lib/components/ui/*` (Button, Card, Input,
Select, Sheet, Dialog, Tabs, …). Import these for form controls and layout pieces.

Utilities: `$lib/utils/formatters` (`formatDate`, `formatNumber`, `formatCurrency`,
`initials`), `$lib/utils/csv` (`exportToCsv`), `$lib/utils/validators`
(zod schemas + `fieldError(err, path)`).

## Design tokens & styling

- Use semantic Tailwind classes backed by the design tokens in `src/app.css`:
  `bg-background` (page canvas), `bg-card` (white surface), `bg-muted`,
  `text-foreground`, `text-muted-foreground`, `bg-primary` / `text-primary-foreground`
  (brand indigo), `border-border`, `border-input`. Never hardcode hex.
- Form controls are outlined (`bg-transparent` + visible `border-input`) with
  `rounded-sm`. Tabs use the full-width segmented style with an indigo active pill.
- Respect dark mode: tokens already have light/dark values; if you add a colored
  surface, check contrast in both modes (a light token + light text fails — see
  how the tabs use dark text on the dark-mode indigo pill).
- `docs/DESIGN.md` documents the full language; `docs/COMPONENTS.md` the component system.

## Verify (the contract)

A page isn't done until all three pass:

```bash
npm run check   # svelte-check: types, unused vars, a11y
npm run lint    # prettier --check + eslint (incl. the resolve rule)
npm run build   # production build
```

`npm run format` auto-fixes prettier; `npx eslint . --fix` fixes the auto-fixable
lint. Run these, then re-verify.

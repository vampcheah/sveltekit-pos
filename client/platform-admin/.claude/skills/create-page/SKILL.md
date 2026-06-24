---
name: create-page
description: >-
  Scaffold a new page in THIS SvelteKit admin starter the way the template
  intends — correct route placement, sidebar registration, the right page
  archetype (list/CRUD, form, detail, or overview/dashboard), the server-only
  data seam, i18n in both locales, resolve()-wrapped navigation, and a green
  check/lint/build. Use this whenever the user wants to add a page, screen,
  route, view, tab, section, or "another page like X" to this admin app —
  e.g. "add an orders page", "create a settings screen", "new report view",
  "build a customers list", "add a detail page for invoices". Prefer this over
  hand-writing a +page.svelte from scratch: the repo has non-obvious conventions
  (typed route ids, the db seam, the navigation-resolve lint rule) that are easy
  to get wrong and are enforced by the lint/type pipeline.
---

# Create a page in the admin starter

This template ships a consistent way to build pages. Follow it and the new page
looks native, type-checks, and passes lint on the first try. Skip it and you hit
the same traps the pipeline enforces (typed route ids, `resolve()` navigation,
the server-only data boundary).

The fastest, most reliable way to build a page here is **to copy the closest
existing page of the same archetype and adapt it** — the repo's example pages
are the source of truth, not snippets in this skill. This skill tells you which
example to copy and the rules that apply to all of them.

## Workflow

Work through these in order. Details for each live in the reference files —
read them when a step needs more than the one-liner here.

1. **Pick the archetype** (table below) and open its canonical example file.
   Read `references/archetypes.md` for what each one contains and what to adapt.
2. **Create the route file.** App pages live at
   `src/routes/(app)/<name>/+page.svelte` (inside the authenticated shell).
   Detail pages use a dynamic segment: `src/routes/(app)/<name>/[id]/+page.svelte`.
3. **Register it in the sidebar** (if it's a top-level nav destination): add an
   entry to `src/lib/shell/nav.ts`. The `href` field is typed `Pathname`, so use
   the literal route — see `references/conventions.md` → Navigation.
4. **Build the page** by adapting the archetype example. Use the shared
   components (`PageContainer`, `PageHeader`, `DataTable`, …) and design tokens —
   never raw hex or ad-hoc spacing. See `references/conventions.md` → Components.
5. **Wire data through the seam** _if the page shows real/dynamic data_: add a
   method to `src/lib/server/db.ts`, load it in a `+page.server.ts`, read it via
   `let { data } = $props()`. Never import `$lib/server/*` from a `.svelte`.
   See `references/conventions.md` → Data.
6. **Internationalize** every user-facing string with `t('…')` and add the keys
   to **both** `src/lib/i18n/locales/en.ts` and `zh-CN.ts`, kept in sync.
7. **Verify.** Run `npm run check && npm run lint && npm run build`. All three
   must be green before you're done — that's the template's contract.

## Archetype selector

| Archetype                | Use when                                       | Copy from                                                                                                                     | Has server load?        |
| ------------------------ | ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| **List / CRUD**          | A table of records, optionally add/edit/delete | `src/routes/(app)/users/+page.svelte` (full CRUD, local state) or `src/routes/(app)/tables/+page.svelte` (read-only, db seam) | tables: yes · users: no |
| **Form**                 | Create/edit a single entity with validation    | `src/routes/(app)/forms/+page.svelte`                                                                                         | usually no (mock)       |
| **Detail**               | One record by id, dynamic route                | `src/routes/(app)/users/[id]/+page.svelte`                                                                                    | yes (load by id)        |
| **Overview / Dashboard** | KPIs, charts, activity feeds                   | `src/routes/(app)/dashboard/+page.svelte`                                                                                     | optional                |

If a page mixes archetypes (e.g. a list with a detail drawer), start from the
dominant one and borrow pieces from the others.

## The rules that bite (apply to every archetype)

These are enforced by `svelte-check` / `eslint` — getting them wrong fails the
verify step, so internalize them up front:

- **Navigation must go through `resolve()`.** `import { resolve } from '$app/paths'`
  then `goto(resolve('/orders'))` and `href={resolve('/orders')}`. Dynamic:
  `resolve(\`/users/${id}\`)`. The `svelte/no-navigation-without-resolve` rule
  fails any bare path.
- **Sidebar hrefs are typed route ids.** `NavItem.href` is `Pathname` (from
  `$app/types`), so only real routes compile. Add the route before referencing it.
- **Data is server-only behind the seam.** Pages read data from a
  `+page.server.ts` `load` that calls `$lib/server/db`; the `.svelte` never
  imports `$lib/server/*` (it would leak server code / credentials into the bundle).
- **Both locales stay in sync.** A key in `en.ts` with no `zh-CN.ts` twin renders
  the raw key. Add to both.
- **Green or not done.** `npm run check && npm run lint && npm run build`.

## Reference files

- `references/conventions.md` — the repo conventions in depth: routing & groups,
  nav registration, the data seam, navigation/`resolve()`, i18n, the shared
  component catalog, design tokens, and the verify pipeline. Read this when a
  workflow step needs detail.
- `references/archetypes.md` — per-archetype recipe: which example to copy, its
  structure, the key imports, and exactly what to change. Read the section for
  the archetype you picked.

The living docs in `docs/DEVELOPMENT.md`, `docs/COMPONENTS.md`, and
`docs/DESIGN.md` go deeper on backend integration, the component system, and the
design language — consult them for anything beyond page scaffolding.

# Page archetypes (reference)

Each archetype maps to a real example page in the repo. The reliable move is to
**open that file, copy its structure, and adapt the names/fields** — then apply
the cross-cutting rules from `conventions.md`. Read only the section for the
archetype you picked.

---

## 1. List / CRUD

**Copy from:** `src/routes/(app)/users/+page.svelte` (full CRUD over local state)
or `src/routes/(app)/tables/+page.svelte` (read-only list via the db seam).

What it demonstrates:

- `<DataTable {columns} {rows}>` with `type Column` describing each column
  (key, header, sortable, align, custom cell snippet).
- Search, sort, row selection, per-row action menus, and pagination — all built
  into `DataTable`; you supply columns + data + cell/action snippets.
- A bulk toolbar (export CSV via `exportToCsv`, delete-selected).
- Add/edit via a `Sheet` (side panel) with a `zod` schema + `fieldError`.
- Delete via `ConfirmDialog`.
- `StatusBadge` for status columns (`type BadgeTone` for the color).

Adapt:

- Define your row type (or import a `Demo*` type from `$lib/data/*`).
- Rewrite the `columns` array for your fields.
- Decide data source: local `$state` copy (like `users`, no server load) **or**
  the db seam (like `tables` — add a `+page.server.ts`, see conventions → Data).
- Keep add/edit/delete only if the page is actually mutable; a read-only list
  drops the Sheet/ConfirmDialog.

Wrap everything in `PageContainer` with a `PageHeader` (title + an `actions`
snippet for the primary "Add" button).

---

## 2. Form

**Copy from:** `src/routes/(app)/forms/+page.svelte`.

What it demonstrates:

- A `zod` schema defining the shape + validation messages.
- Local `$state` for each field; `safeParse` on submit; `fieldError(err, path)`
  to surface the first message per field.
- The full control set: `Input`, `Textarea`, `Select`, `Checkbox`, `Switch`,
  `RadioGroup`, `Slider`, a `Popover` + `Calendar` date picker.
- `Label` per field, error text under invalid fields, `toast` on submit.

Adapt:

- Replace the schema with your entity's fields.
- Keep only the controls you need (don't ship a date picker you don't use).
- On submit: for a demo, `toast` + reset; for real persistence, call a
  `+page.server.ts` form action or a `db` mutation (conventions → Data).
- Validation messages are user-facing → run them through `t()` if the app is
  fully localized, or keep them in the schema for a starter.

Layout: `PageContainer` → `PageHeader` → one or more `Card.Root` sections.

---

## 3. Detail (dynamic `[id]`)

**Copy from:** `src/routes/(app)/users/[id]/+page.svelte`.

What it demonstrates:

- A dynamic route segment `[id]` serving `/<name>/:id`.
- A back link to the list using `resolve('/users')`.
- A profile-style header (avatar, name, meta) + detail cards.
- `EmptyState` / not-found handling when the record is missing.

Adapt:

- Create the folder `src/routes/(app)/<name>/[id]/+page.svelte`.
- Load the record by id in `src/routes/(app)/<name>/[id]/+page.server.ts`:
  ```ts
  import { db } from '$lib/server/db';
  import { error } from '@sveltejs/kit';
  import type { PageServerLoad } from './$types';
  export const load: PageServerLoad = async ({ params }) => {
    const item = await db.<name>.get(params.id);
    if (!item) throw error(404, 'Not found');
    return { item };
  };
  ```
- Link to the detail page from the list with `resolve(\`/<name>/${row.id}\`)`.
- Detail pages are usually NOT in the sidebar (`nav.ts`) — they're reached from
  the list — so skip nav registration.

---

## 4. Overview / Dashboard

**Copy from:** `src/routes/(app)/dashboard/+page.svelte`.

What it demonstrates:

- A responsive grid of `StatCard` KPIs.
- Charts hand-rolled with SVG/CSS using design tokens (note: the file explains
  why layerchart's high-level components are intentionally avoided — their
  published prop types don't type-check under svelte-check; follow that lead).
- A bar-list (traffic by channel) and an activity feed built from `Card` + tokens.
- A greeting derived from `auth.user`.

Adapt:

- Swap the stat/series/activity data (from `$lib/data/*` or the db seam).
- Reuse the SVG area-chart / token bar-list patterns rather than reaching for a
  charting lib — they're responsive, exact, and type-safe here.
- Compose with `Card.Root` sections inside `PageContainer`.

---

## After any archetype

Run the cross-cutting steps from SKILL.md / `conventions.md`:
register in `nav.ts` (if top-level), `resolve()` all navigation, i18n both
locales, then `npm run check && npm run lint && npm run build` until green.

---
version: 1
slug: "src-pages-wf-inventory-next"
primary_target: "src/pages/wf_inventory/next"
related_targets: []
---

# Surface brief — WF Inventory "next" layout

## Scope and mode
Scope: `src/pages/wf_inventory` (native-ish operating surface: Tauri desktop app, web platform). Mode: Operate.

## Audience, job, constraints
Warframe players preparing stock for Warframe.Market; job: scan thousands of owned mods/rivens, filter to sale candidates fast. Constraints pinned by the user:
- Implementation may ONLY use the installed Mantine component library plus the project's existing shared components (SearchField, CompactSortSelect, ExpandableButton, PreviewCard, PaginationFooter, ActionWithTooltip) and existing patterns/CSS module conventions. No bespoke visual systems, no custom-crafted world material.
- Header: no clock; user menu stays; the user dropdown gains a permanent "Back to original UI" item at the bottom navigating to `/wf_inventory`.
- The classic WF inventory page (`/wf_inventory`, vertical tabs) remains untouched and reachable.
- Entry: the existing "WF Inventory" sidebar nav item opens the new layout; the sidebar itself is hidden in the new layout.
- Content: Rivens + Mods only.

## Direction contract
THESIS: the inventory is one continuous edge-to-edge sheet of cards; all chrome recedes into a left facet drawer and a slim header with search — the wall is the surface. This refuses the current design's default arrangement (vertical tab rail + per-tab toolbars duplicating chrome).
OWN-WORLD: the project's established Mantine dark theme (dark-7/8 surfaces, blue primary, tabular numerals; `PreviewCard` headers/footers) carries the sheet; no decoration beyond the theme.
STORY: "I open WF Inventory, land on my mods wall (or rivens), pull a category facet, watch the wall refilter, search when I know the name, page through results; when done I leave through the user menu back to the original UI."
FIRST VIEWPORT: Mantine AppShell: header 58px (Logo, search, UserMenu) with no clock; navbar 250px persistent facet drawer — section toggle (Mods/Rivens) and facets (category, market, veiled/unveiled for rivens); main = toolbar strip (sort) + 4-column `SimpleGrid` card wall + PaginationFooter under ScrollArea. Primary action: the search field, top center-right.
FORM: chosen structure "Sheets Drawer", dealt lead of surface roll 2 (dealt indices 5,3,7), THIRD dealt structure on the table after user re-rolled hand 1. seed:05aa08eee3ba2041.
FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance.

## Unresolved decisions
- Live facet counts in the drawer (the sketch showed them) are deferred: backend has no count endpoint; drawer ships without counts.
- Wall cards for rivens reuse the Riven tab's cards, simplified presentation reuse; deep stock-work actions stay with the classic Rivens tab until the replacement fully replaces it.

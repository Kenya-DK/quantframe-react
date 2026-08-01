## Features

- ✨ Debugging Live Item Entry form now supports editing existing entries and canceling — added edit/update/cancel buttons, a Syndicate operation option, and `wfm_id` field.
- ✨ Debugging panel now shows item operations as badges and allows inline editing of existing entries.
- ✨ Added `ItemMarketInfo` (lowest/highest price, price range, volume) to each `ItemEntry` for order processing and debugging.
- ✨ Added a global knapsack pass that runs after all items are processed, deleting excess buy orders beyond the total price cap (newly created orders are spared until the next cycle).

## Fixes

- 🛠️ Removed the `999` hard cap on the Max Standing Cost input in the Syndicate settings.

## Refactors

- ♻️ Renamed `operation` to `operations` across the live scraper — `ItemEntry`, helpers, item module, frontend types, and the debugging panel.
- ♻️ `get_order_info` now returns the existing order id/price alongside properties; order lifecycle logic moved into `progress_order` / new `delete_order`.
- ♻️ `populate_order_properties` no longer reads back the operation set from properties — operations are merged from the entry and trade operations instead.

## Dev Notes

- 🔧 Re-enabled syndicate interesting-item collection (still WIP — the syndicate processing pipeline remains disabled).
- 🔧 Switched `wf-market` dependency to the local path for development.

## Icons

- ⏰ TODO
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors

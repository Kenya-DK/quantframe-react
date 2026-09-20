## Features

- ⏰ Added an order cooldown system to the live scraper that prevents rapid-fire Warframe Market order updates: a 20-minute cooldown when re-posting at the same price and a 5-minute cooldown after a price change. Cooldown state is persisted per item and surfaced in the UI (clock icon on row actions and a live countdown in the price history popover). Currently disabled by default behind a `ORDER_COOLDOWN_ENABLED` flag.
- ✨ Added a minimum profit threshold logic to the live scraper, allowing users to set a minimum profit percentage or flat profit amount for their trades.
- ✨ Added a full Syndicate trading pipeline: a new `syndicate_item` database table and API, a redesigned WTS settings panel with per-syndicate standing tracking and an "ignore standing" toggle, importing syndicate items from Warframe Market, and a live scraper Syndicate tab with sell/edit/delete/export operations.
- ✨ Reworked WF Inventory to support pluggable sources — Warframe profile or AlecaFrame (`lastData.dat` watcher) — selectable in Settings → Advanced → WF Inventory, with a manual refresh button and syndicate standing import.

## Fixes

- 🛠️ Fixed the unit price being recorded as the total trade price instead of the per-item unit price (closes #124).
- 🛠️ Fixed a crash ("unexpected value for StockStatus enum") caused by stale status values in the database (e.g. `cooldown_price_change`); the stock status now falls back to `Unknown` instead of erroring.
- 🛠️ Fixed `nullify_zeroed_properties` erroring on non-numeric properties by supporting `f64`, `i64`, `i32`, `u64`, `String`, and array types instead of assuming everything is a float.
- 🛠️ Fixed syndicate items failing to post when the standing check errored; the check now degrades gracefully instead of aborting the pipeline.
- 🛠️ Fixed the users activity linking to to thw wrong endpoint
- 🛠️ Fixed `stock_riven_sell` logging to the wrong file (`stock_riven_create.log`).

## Refactors

- ♻️ Reworked analytics from a single `add_metric` call into a structured, batched event system. Events are now named (`ApplicationEvent`, e.g. `app_start`, `stock_item_create`, `page_view`, `switch_theme`) and carry structured string properties such as `success`, `error_type`, and `count`. Events are queued in memory and flushed to the backend every 10 seconds in batches of up to 50, and `track_event` instrumentation was added across commands, the live scraper, the log parser, and theme switching.
- ♻️ Reworked the live scraper's selling, wishlist, and syndicate pipelines so the order summary is only logged after a successful Warframe Market order update, and moved the "mark as live" persistence to run only on success.
- ♻️ Items are now marked as Live before the order update is attempted (so a cooldown-skipped update no longer flips them to Error), and cooldown state is tracked per item via a new `cooldown` property.
- ♻️ Reworked wishlist change tracking to use a list of changed fields (`Vec<String>`) instead of a single value.
- ♻️ Reworked the stock riven pipeline (create/delete/sell) to use `OperationSet` flags instead of `&[&str]`, tagged operations with a `StockRiven_` prefix, and added a unified logging helper that respects per-call disable flags (`DisableCreatedLog`, `DisableDeletedLog`, `DisableUpdatedLog`, `DisableCompleteLog`, `DisableNotFoundLog`) plus `ReturnOn:` early-exit handling. Renamed the `bought` parameter to `price` in the `stock_riven_sell` command.

## Dev Notes

## Icons

- ⏰ Cooldown / Timed
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors

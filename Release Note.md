## Features

- ✨ Purchased Wishlist items are now routed to the Wishlist handler instead of being added to the Selling List — they are removed/updated on the Wishlist as expected.
- ✨ Added Max Price Drop / Min Listings Below settings for both WTS (selling) and WTB (buying) to control auto-pricing follow behavior — prevents following large price drops unless enough competition is present (closes #108).
- ✨ Added Syndicate live scraper mode — new TradeMode, filtering by standing cost/volume/syndicate/rank, and dedicated processing pipeline for syndicate items (hidden behind dev flag).
- ✨ Added `operations` filtering to WFM order pagination — allows filtering backend orders by their current operation set.

## Fixes

- 🛠️ Moved `notify_gui!` call inside the Critical/Error check in `live_scraper/client.rs` to prevent GUI notifications for non-critical log levels.
- 🛠️ Fixed purchased Wishlist items appearing in the Selling List instead of being handled on the Wishlist (closes #116).
- 🛠️ Fixed Auto Delete deleting blacklisted item orders on Live Scraper start — blacklist now checks sub_type (e.g. riven rank) so items like "Longbow Sharpshot" are properly skipped (closes #114).
- 🛠️ Fixed analytics crash on startup by guarding `set_last_user_activity` behind `HAS_STARTED` check.
- 🛠️ Added `[Info]: OnTradeAccepted failed` detection — trade state is now properly reset when the trade dialog is not accepted, preventing the auto-pricer from getting stuck (closes #117).

## Refactors

- ♻️ Consolidated `SubType` struct into the `utils` crate — removed duplicate definitions from `entity::dto` and `qf_api::types`, unified all imports across the codebase.
- ♻️ Moved `OperationSet` into `ItemEntry` for live scraper progress stages — operations are now tracked on the entry itself and passed by mutable reference, enabling syndicate and future modes to update operation state.

## Dev Notes

- 🔧 Improved frontend error handling with typed error handlers for `PromiseRejectionEvent`, `ErrorEvent`, and generic `Error` — errors now include structured cause, component, and properties for better debugging.
- 🔧 Syndicate live scraper tab and settings panel are hidden behind `import.meta.env.DEV` flag.

## Icons

- ⏰ TODO
- ✨ Features
- 🛠️ Fixes

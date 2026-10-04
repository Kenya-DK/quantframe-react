## Features

## Fixes

- 🛠️ Fixed trade processing handling every item twice (the wish-list/stock handler plus a duplicate `handle_item` call), which created duplicate stock entries and transactions. Purchases now reliably land in stock, including purchased wish-list items.

## Refactors

- ♻️ Removed unused imports and dead code across the Rust workspace, so `cargo check` runs without warnings.
- ♻️ Analytics now reliably reports shutdown: the `app_exit` event is tracked and flushed on `RunEvent::ExitRequested` (previously the app was killed before the queue flushed), and the Warframe Market client's per-route API call counts are reported as `wfm_api_tracking` events on exit.

## Dev Notes

## Icons

- ⏰ Cooldown / Timed
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors

## Features

- ✨ Added riven price prediction powered by an ONNX model (`price_model.onnx` + vocabularies in the cache `models` folder), exposed through new Tauri commands and the `api.riven` client. The WF Inventory → Rivens tab now has a button on each identified riven card to fetch an estimated price.
- ✨ Live Scraper: added a **Use Price Prediction** option for rivens. When enabled, the listing price is set from the price model instead of the live auction average, and the riven list marks prices that came from the prediction with a 🤖 icon.

## Fixes

- 🛠️ Fixed trade processing handling every item twice (the wish-list/stock handler plus a duplicate `handle_item` call), which created duplicate stock entries and transactions. Purchases now reliably land in stock, including purchased wish-list items.

## Refactors

- ♻️ Removed unused imports and dead code across the Rust workspace, so `cargo check` runs without warnings.
- ♻️ Analytics now reliably reports shutdown: the `app_exit` event is tracked and flushed on `RunEvent::ExitRequested` (previously the app was killed before the queue flushed), and the Warframe Market client's per-route API call counts are reported as `wfm_api_tracking` events on exit.
- ♻️ Moved the riven price model into the cache module structure (`cache::modules::riven_pricer` and `cache::types::riven::riven_price`), matching the other cache modules.

## Dev Notes

## Icons

- ⏰ Cooldown / Timed
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors

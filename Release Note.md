## Release Notes - 2026-07-14

## Features

- ✨ Migrated stock item, stock riven, and wish list pricing fields (minimum_price, minimum_profit, minimum_sma, maximum_price) into a unified Properties system for better extensibility.
- ✨ Added new Properties utility methods: has_property, remove_property_value, remove_property_values, keep_property_values, nullify_zeroed_properties, is_type, and enhanced merge_properties with null-removal support.
- ✨ Implemented TryGetable and ValueType for sea-orm integration, enabling direct JSON column mapping in the database.

## Fixes

- 🛠️ Fixed log cleanup string comparison in core.rs.
- 🛠️ Fixed GetProperty helper in edit modals to read from form values instead of the original value reference.

## Dev Notes

- Refactored entity DTOs (create, update, pagination) to use Properties-based field changes instead of individual numeric fields.
- Added two new database migrations: add_properties column and drop_min_price_columns.
- Updated live scraper modules (item, riven, wishlist) to read pricing thresholds from properties.
- Cleaned up frontend edit forms to use properties.* field paths (min_price, min_profit, min_sma, max_price).
- Removed nested Group wrapper in SelectSubType component.

## Release Notes - 2026-07-03

## Features

- ✨ Improved logging workflow and logging-related groundwork.
- ✨ Add login feedback to the user during startup.
- ✨ Export logs will now have the state of the app
- ✨ Add raw lines of logs to the trade for debugging purposes.
- ✨ Add a delete button to the trade processing popup to allow users to delete trades from the queue.
- ✨ Add Lookup for syndicate items in the trading analytics tab. (Patreon T2+)

## Fixes

- 🛠️ Fix the error was showing when the websocket was down
- 🛠️ Fix the parsing errors in the trade parsing.
- 🛠️ Fix slow loading in the chat tab
- 🛠️ Fix the Price History when hovering over the WFM log in the order info
- 🛠️ Fix a error when y try to import a auction

## Dev Notes

- Clean up the settings in the UI and the backend

## Dev Notes

- Logging updates are the primary focus of this revision.
- 🛠️ Look into this: https://github.com/knoellle/wfinfo-ng

## TODO

- ⏰ Fix chat message generation.
- ⏰ Add a riven preview image to the riven mod details page.
- ⏰ Dump all logs to the app folder when the app crashes so users can send logs for debugging.

## Icons

- ⏰ TODO
- ✨ Feature
- 🛠️ Fix

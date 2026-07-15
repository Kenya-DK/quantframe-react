## Release Notes - 2026-07-14

## Features

- ✨ Added ScrollAreaAutosize wrapper to the Update Available modal for better scroll handling of changelog content.

## Fixes

- 🛠️ Sorted and cleaned up imports in UpdateAvailable modal.
- 🛠️ Made `properties` column nullable in `stock_riven` and `wish_list` tables to gracefully handle null data.
- 🛠️ Fixed migration 6 (`add_uuid_to_stock_riven`) using `Entity::find()` which referenced `properties` column not yet created — switched to raw SQL.
- 🛠️ Fixed `ResponseError.properties` type to be optional.

## Dev Notes

- Cleared out stale release notes history for a cleaner file.
- Fixed icon legend entries (Feature → Features, Fix → Fixes).

## Icons

- ⏰ TODO
- ✨ Features
- 🛠️ Fixes

## Features

## Fixes

- 🛠️ Fixed trade processing handling every item twice (the wish-list/stock handler plus a duplicate `handle_item` call), which created duplicate stock entries and transactions. Purchases now reliably land in stock, including purchased wish-list items.

## Refactors

- ♻️ Removed unused imports and dead code across the Rust workspace, so `cargo check` runs without warnings.

## Dev Notes

## Icons

- ⏰ Cooldown / Timed
- ✨ Features
- 🛠️ Fixes
- ♻️ Refactors

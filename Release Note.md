## Features

- ✨ Purchased Wishlist items are now routed to the Wishlist handler instead of being added to the Selling List — they are removed/updated on the Wishlist as expected.
- ✨ Added Max Price Drop / Min Listings Below settings for both WTS (selling) and WTB (buying) to control auto-pricing follow behavior — prevents following large price drops unless enough competition is present (closes #108).

## Fixes

- 🛠️ Moved `notify_gui!` call inside the Critical/Error check in `live_scraper/client.rs` to prevent GUI notifications for non-critical log levels.
- 🛠️ Fixed purchased Wishlist items appearing in the Selling List instead of being handled on the Wishlist (closes #116).
- 🛠️ Fixed Auto Delete deleting blacklisted item orders on Live Scraper start — blacklist now checks sub_type (e.g. riven rank) so items like "Longbow Sharpshot" are properly skipped (closes #114).

## Dev Notes

## Icons

- ⏰ TODO
- ✨ Features
- 🛠️ Fixes

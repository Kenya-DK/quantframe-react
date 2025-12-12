⚠️⚠️⚠️WARNING: Make a backup of your database before updating!⚠️⚠️⚠️

How to back up your database:

1. Close Quantframe.
2. Open the folder where your Quantframe database is stored.
   - Windows (default):
     C:\Users\<YourUsername>\AppData\Local\dev.kenya.quantframe
3. Copy the following files to a safe backup location:
   - quantframeV2.sqlite
   - quantframeV2.sqlite_backup
   - quantframeV2_debug.sqlite
   - settings.json

After updating, some settings may be reset to their defaults.
You can restore your previous settings by copying from your backup settings.json file.

## TODO's

- ⏱️ Fix so the knapsack algorithm doesn't delete akk items after a cycle.
- ⏱️ Add Riven ranking
- ⏱️ Add a sound or a discord message if the livesraper dies
- ⏱️ Fix Riven select

## Feature's

- ✨ You can now save and load templates for the trade message generator.
- ✨ Add multiple language support (Require a y contribution to help with the translations NO AI)
- ✨ Add generate chat messages with custom templates for ...
- ✨ Add generate WTS messages for items, rivens and wishlists
- ✨ Add a preview for items to to buy.
- ✨ Add Stock Riven Details Modal and wishList
- ✨ Add sorting for the auctions and orders tab
- ✨ Add export to JSON for transactions, Stock items, WishList, Stock Rivens, WishList, Items Prices, Rivens Prices Require patron T1+
- ✨ Add theme support for the app
- ✨ Overhaul the Blacklist system now you can add items to the blacklist each trade method
- ✨ You can now fully customize the notifications you get from the app
- ✨ Add a new notification webhook type where it wil send a object. (WIP)
- ✨ Add a edit modal for the stock items
- ✨ Add Min Profit & Min SMA per item in the live trader stock items
- ✨ Add min max filter for the trading analysis
- ✨ Add Max Price for WTB orders Fx Arcane XX wil not put up a price for more than 100p if it is set to 100 but i can go below 100p
- ✨ Add date filter for the trading analysis
- ✨ Add show item parts for item sets
- ✨ Add a http server so you can create a riven by sending a post request to the app
- ✨ Add bulk update for stock items, stock rivens and wishlists
- ✨ Add profit in transactions, Wil work like this When you sell a item it will look for the last bought price and show the profit you made

## Dev Notes

- 🛠️ Add a delay for riven stock updates to reduce api calls on the livec
- 🛠️ Using a new system for riven so pls can for duplication on warframe market
- 🛠️ Look into this https://github.com/knoellle/wfinfo-ng

## Fix's

- 🛠️ Fix wired behavior when trading of mutable of the same item
- 🛠️ Some Riven weapons are not showing up in create a new riven Tombfinger, Verglas
- 🛠️ Fix so the overview over the stuck wil be updated properly
- 🛠️ Fix so the stock items don't disappear when the list is updated
- 🛠️ Fix Ranks is not show in the wfm tab
- 🛠️ The Stock items selling listed price is not using the bought price
- 🛠️ Fix the dashboard not loading the last X days properly
- 🛠️ Fix items in not reported to wfm

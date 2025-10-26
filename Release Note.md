## Overview

Note: All previous versions are not supported anymore.
How To migrate:
[Read More](https://quantframe.app/upgrading/1.2.X)

## TODO's

- [ ] Fix when you didn't put in a user id in the discord notification it just says <MENTION>
- [ ] Add show parts for item set
- [ ] Fix items in not reported to wfm
- [ ] The Stock items selling listed price is not using the bought price
- [ ] Add date filter for the trading analysis

## Feature's

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

## Dev Notes

- 🛠️ Add a delay for riven stock updates to reduce api calls on the livec
- 🛠️ Using a new system for riven so pls can for duplication on warframe market
- 🛠️ Using a new system for riven so pls check for duplication on warframe market
- 🛠️ Look into this https://github.com/knoellle/wfinfo-ng

## Fix's

- 🛠️ Some Riven weapons are not showing up in create a new riven Tombfinger, Verglas
- 🛠️ Fix so the overview over the stuck wil be updated properly
- 🛠️ Fix so the stock items don't disappear when the list is updated
- 🛠️ Fix Ranks is not show in the wfm tab

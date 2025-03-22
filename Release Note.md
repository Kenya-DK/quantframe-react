# Quantframe v1.2.6

## Overview

Remove the whitelist for a new wishlist system.
How To migrate:
[Read More](https://quantframe.app/upgrading/1.2.X)

//TODO: Add a message if the user is not verified.
//TODO: Quantity of goods WTB Wil not update if a order i all ready placed.
//TODO: Add better search for auction.
//TODO: Fix wfm order not beding deletede but auto trade when it is wishlist
//TODO: Fix auction [LiveScraper:WarframeMarket:Auctions:Create] src\wfm_client\modules\auction.rs:197:21, There was an error creating the auction The request failed with status code 400 to the url: https://api.warframe.market/v1/auctions/create with the following message: item: Object {"attributes": Array [String("app.auctions.errors.too_many_positives")]}, {"ApiError":{"statusCode":400,"error":"ApiError","messages":["item: Object {\"attributes\": Array [String(\"app.auctions.errors.too_many_positives\")]}"],"raw_response":"{\"error\": {\"item\": {\"attributes\": [\"app.auctions.errors.too_many_positives\"]}}}","url":"https://api.warframe.market/v1/auctions/create","body":{"note":"","starting_price":88,"buyout_price":88,"minimal_reputation":0,"minimal_increment":1,"private":false,"item":{"type":"riven","re_rolls":0,"attributes":[{"positive":true,"value":1.06,"url_name":"damage_vs_infested"},{"positive":true,"value":24.9,"url_name":"critical_chance"},{"positive":true,"value":8.4,"url_name":"chance_to_gain_extra_combo_count"},{"positive":false,"value":13.6,"url_name":"impact_damage"}],"name":"Pura-lacicron","weapon_url_name":"kogake","mod_rank":0,"polarity":"naramon","mastery_level":10}},"method":"POST"}}

## Features

- ✅ Add Wishlist: You can now add items to your wishlist.
- ✅ Add Alert if soothing in wrong
- ✅ Add Chats: You can now chat with other users.
- ✅ Add Team of service (TOS) popup on start

## Fix's

- 🛠️ Fix a issue where the bot wut ignore the price range of a item

# Quantframe v1.2.3 (Release Date)

## Overview

Total rewrite of the UI and the backend
Note: The database was upgrade to V2
So all your data needs to be migrated, this be be done in the Debug Tab and under the migrate tab.

## Fix's

- 🛠️ Order limit not updating.
- 🛠️ A bug in migrate dataBase.
- 🛠️ Fix a bug in export logs.
- 🛠️ Fix some items not sowing in select item.
- 🛠️ Fix min profit.

## Features

- ✅ Add Import Riven from wfm auctions
- ✅ Add Create Riven
- ✅ Add Better Update box
- ✅ Add Cat!!

# Quantframe v1.2.0 (Release Date)

## Overview

Total rewrite of the UI and the backend.
Note: The database was upgrade to V2
So all your data needs to be migrated, this be be done in the Debug Tab and under the migrate tab.

## Features

- Add Rest: Your can now reset all transaction.
- Add minium profit in settings.
- Add minium sma.
- Remove the price scraper (It is now server).
- Add listed price history.
- Add mass edit on stock Rivens.
- Add Trading Tax Cap.
- Add Auto Trade: Note this can be buggy so use it at y own risk...

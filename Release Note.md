# How to use parser

1. Get your data from DE
   - Go to [zendesk](https://digitalextremes.zendesk.com/hc/en-us)
   - Submit new request in category "My Account"
   - Select subcategory "CCPA or GDPR - General Data Protection Regulation"
   - Write something in Subject and Description fields
   - Wait 1+ days
2. Unpack archive in some new folder.
3. After you get your .txt files, Drag and drop one file at a time here.

## Add Feature's

- ✨ Add a notification sound's
- ✨ Add a sound or a discord message if the live scraper dies
- ✨ Add Riven ranking
- ✨ Add Minimum price for wishlist items
- ✨ Add a logout prompt to confirm user wants to logout (Yurii-IvoryFace)
- ✨ Add https://remoraid.dev link to as a way to create a new design and to the settings page
- ✨ Add Warframe GDPR Parse tab to the trading analytics page (WIP)

## Fix's

- ⏱️ Fix a bug if y have buy quantity set to more the 1 and but it from then wfm tab it wut have the price
- ⏱️ Fix user can type in the chat box

## Dev Notes

- ✏️ Remove some logging that was only for testing
- ✏️ Refactor trade processing to be more modular and easier to read when debugging
- ✏️ If i item have no sellers y can set a minimum price for it to list

## TODO's

- ⏱️ Fix so the knapsack algorithm doesn't delete all items after a cycle, Cursing a buy is deleted when it was ppl is texting you
- ⏱️ The Log file path sometimes f\* \* \* up
- ⏱️ The Generate message still have the item if it was deleted by the backend
- ⏱️ Fix no attack speed on some melee weapon stats
- ⏱️ Fix Generate message not cursing error
- ⏱️ (Need Testing) - Add notification when Warframe gdpr log parser is done
- ⏱️ Add confirmation when user logs out
- ⏱️ Fix so y cant save a message template with the same name
- ⏱️ Fix Types errors in StockRiven Update

## Feature's

## Dev Notes

- 🛠️ Look into this https://github.com/knoellle/wfinfo-ng

## Fix's

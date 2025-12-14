## Fix's

- 🛠️ Fixed an issue where you can't logout

## ⚠️⚠️⚠️WARNING: Make a backup of your database before updating!⚠️⚠️⚠️

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

## Dev Notes

- 🛠️ Add a delay for riven stock updates to reduce api calls on the livec
- 🛠️ Using a new system for riven so pls can for duplication on warframe market
- 🛠️ Look into this https://github.com/knoellle/wfinfo-ng

## Fix's

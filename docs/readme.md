# Developer Documentation

How the app works, on a technical level.

The app requires you to signin to warframe.market, as the API is a pretty fundamental part of the app.

The first thing the app does is get you to sign into WFM and store the Access Token. This gets stored in the `settings.dat` file, which also stores other info, **but not the user's password** (we only use it once, to sign in).

## For YelBuzz
My hope is that YelBuzz contributes back and that this project allows his good ideas reach more people. I'm clueless for the finance stuff, but I do believe I build a mean app architecture.

This section is to help him map key parts of his app to mine.

> But its also kinda for me as well, since this is a reverse engineering effort.

|[WF-Algo-trader](https://github.com/akmayer/Warframe-Algo-Trader)|[QuantFrame](https://github.com/metruzanca/quantframe)|
|--|--|
| [getWFMtoken.py](https://github.com/akmayer/Warframe-Algo-Trader/blob/main/getWFMtoken.py)<br>[AccessingWFMarket.py](https://github.com/akmayer/Warframe-Algo-Trader/blob/main/AccessingWFMarket.py) | [wfm_client.rs](https://github.com/metruzanca/quantframe/blob/main/src/lib/wfmClient.ts) |
| [inventoryApi.py](https://github.com/akmayer/Warframe-Algo-Trader/blob/main/inventoryApi.py)| WIP |
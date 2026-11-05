# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Warframe players who trade mods on the Warframe.Market platform: they source stock (rivens, mods), list it cheaply, flip/repair it, and resell for profit. They manage this from their desktop while the game is open.

## Product Purpose

Quantframe is a desktop trading assistant for the Warframe player market. It sandwiches Warframe.Market: live scrapers feed it market data, it tracks the user's game inventory (AlecaFrame integration), manages trade messages, stock, and pricing tools. Success = catching profitable trades before competitors and reducing the manual bookkeeping between game and market.

## Positioning

It sits between the user's actual game inventory and Warframe.Market's market data — sources and keeps both synced locally (encrypted AlecaFrame reads, live scrapers, local price models) rather than being a wrapper website.

## Operating Context

Desktop app (Tauri) running alongside the game. Authenticated users connect Warframe.Market credentials; inventory sources are configurable in Settings (AlecaFrame/file/plain). The app surface is one coherent dark, dense trading UI built on Mantine.

## Capabilities and Constraints

- WF inventory feature: reads RawUpgrades/Upgrades from the configured inventory source, exposes Rivens (search/filters/stock/price model) and Mods (search/category filters/sort). Server-side (Rust) filtering + pagination via Tauri commands.
- Feature is currently in development; hidden from production sidebar (`hide: !import.meta.env.DEV`).
- Backend mirror structure: `src-tauri/src/wf_inventory/` (modules, types, inv_sources) + `src-tauri/src/commands/wf_inventory.rs`.
- Open decision: which inventory sections exist beyond Rivens and Mods (undecided by the user; add later).

## Brand Commitments

- Name: Quantframe. Existing logo asset/color token (`theme.other.logoColor`).
- The incumbent visual system is the shared Mantine dark world of the rest of the app (Shared components, CSS module patterns, `data-has-alert` contract).

## Evidence on Hand

- Warframe inventory data: user's AlecaFrame export (`LastDataRaw.json`) shows ~1228 mod entries, ~800 unique, 46 types missing from the local mods cache (mostly Railjack cards); prompts a fallback-name display.
- i18n: `public/lang/en.json`, all UI copy translated via `useTranslate*` hooks; missing keys must be added there.

## Product Principles

1. Trading speed first: scanability and quick search/filter beat presentation.
2. One coherent app: new surfaces must read as Quantframe, not a separate product.
3. Server-side truth: list state (query, pagination, sort) flows through Rust, mirrored by client-side result caching.
4. Escape hatches must be obvious and reversible (new UI is navigable back to the classic UI in one action).

## Accessibility & Inclusion

Inherit Mantine defaults + existing conventions (tooltips on icon-only actions, translated labels). No project-specific requirement established.

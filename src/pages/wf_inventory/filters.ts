/**
 * Shared inventory facet definitions.
 *
 * These arrays drive the query `item_types` filter for the user's inventory, in
 * every WF inventory surface (classic Rivens/Mods tabs and the "next" layout):
 * the backend matches each entry against the item's unique_name via contains.
 */
export const RIVEN_ITEM_TYPES: Record<string, string[]> = {
  rifle: ["/Lotus/Upgrades/Mods/Randomized/LotusRifleRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawRifleRandomMod"],
  shotgun: ["/Lotus/Upgrades/Mods/Randomized/LotusShotgunRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawShotgunRandomMod"],
  pistol: ["/Lotus/Upgrades/Mods/Randomized/LotusPistolRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawPistolRandomMod"],
  melee: ["/Lotus/Upgrades/Mods/Randomized/LotusMeleeRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawMeleeRandomMod"],
  zaw: ["/Lotus/Upgrades/Mods/Randomized/LotusZawRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawZawRandomMod"],
  arch: ["/Lotus/Upgrades/Mods/Randomized/LotusArchgunRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawArchgunRandomMod"],
};


/** Fallback name for mods missing from the local cache: path segment with camel case split into words. */
export const readableModPath = (unique_name: string) =>
  unique_name.split("/").pop()?.replace(/(?<!^)([A-Z])/g, " $1") || unique_name;

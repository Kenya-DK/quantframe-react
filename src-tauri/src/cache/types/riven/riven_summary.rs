use std::collections::HashMap;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use utils::*;

use crate::{
    cache::{RivenSingleAttribute, RivenStatWithWeapon},
    enums::*,
    utils::modules::states,
};

static MODIFIERS: LazyLock<HashMap<String, (f64, f64)>> = LazyLock::new(|| {
    HashMap::from([
        ("B2|C0".to_string(), (0.99, 0.0)),
        ("B2|C1".to_string(), (1.2375, -0.495)),
        ("B3|C0".to_string(), (0.75, 0.0)),
        ("B3|C1".to_string(), (0.9375, -0.75)),
    ])
});
const TWO_DIGIT_TAGS: &[&str] = &[
    "WeaponFactionDamageGrineer",
    "WeaponFactionDamageCorpus",
    "WeaponFactionDamageInfested",
    "WeaponMeleeFactionDamageGrineer",
    "WeaponMeleeFactionDamageCorpus",
    "WeaponMeleeFactionDamageInfested",
];
#[derive(Debug, Serialize, Deserialize)]
pub struct RivenSummary {
    stat_with_weapons: Vec<RivenStatWithWeapon>,
}

impl RivenSummary {
    pub fn new(
        unique_name: impl Into<String>,
        _mastery_rank: i64,
        _rerolls: i64,
        rank: i64,
        _polarity: impl Into<String>,
        attributes: Vec<(String, f64, bool)>,
    ) -> Result<Self, Error> {
        let cache = states::cache_client()?;
        let riven_lookup = cache.riven();
        let unique_name = unique_name.into();
        let mut rank = rank as f64;
        let weapon = riven_lookup
            .get_riven_by(FindBy::new(FindByType::UniqueName, &unique_name))
            .map_err(|e| e.with_location(get_location!()))?
            .ok_or_else(|| {
                Error::new(
                    "RivenSummary::New",
                    format!("Weapon not found for unique name {}", unique_name),
                    get_location!(),
                )
            })?;

        let total_buffs = attributes
            .iter()
            .filter(|(_, _, positive)| *positive)
            .count();
        let total_curses = attributes
            .iter()
            .filter(|(_, _, positive)| !*positive)
            .count();
        let modifier_key = format!("B{}|C{}", total_buffs, total_curses);
        let multipliers = match MODIFIERS.get(&modifier_key) {
            Some(vals) => *vals,
            None => {
                return Err(Error::new(
                    "RivenSummary::New",
                    format!("Modifier not found for key {}", modifier_key),
                    get_location!(),
                ))
            }
        };
        let mut stats = vec![];
        for (stat_tag, rolled_value, is_positive) in attributes.iter() {
            let upgrade = cache
                .riven_parser()
                .get_riven_upgrade_by(&weapon.upgrade_type, stat_tag)?;

            let mut base_stat = 90.0
                * upgrade.value
                * weapon.disposition
                * if *is_positive {
                    multipliers.0
                } else {
                    multipliers.1
                };

            if upgrade.localization_string.contains("|val|%")
                || upgrade.localization_string.contains("|STAT1|%")
            {
                base_stat *= 100.0;
            }

            let mut adjusted_value = *rolled_value;
            if TWO_DIGIT_TAGS.contains(&stat_tag.as_str()) {
                adjusted_value -= 1.0;
            }

            // Smart rank fallback when no rank is provided but the value fits an R8 curve
            if rank == 0.0 && (adjusted_value - base_stat).abs() < 0.5 * adjusted_value {
                rank = 8.0;
            }

            let scaled_value = adjusted_value / (rank + 1.0) * 9.0;

            let random_factor_raw =
                ((scaled_value - base_stat * 0.9) / (base_stat * 0.2)).clamp(0.0, 1.0);

            let final_value = (scaled_value * 10.0).round() / 10.0;

            let (min_roll, max_roll) = if *is_positive {
                (base_stat * 0.9, base_stat * 1.1)
            } else {
                (base_stat * 1.1, base_stat * 0.9)
            };
            if *is_positive {
                stats.push(RivenSingleAttribute::new(
                    stat_tag.clone(),
                    upgrade.localization_string.clone(),
                    final_value,
                    min_roll * 0.9,
                    max_roll * 1.1,
                    random_factor_raw,
                ));
            } else {
                stats.push(RivenSingleAttribute::new(
                    stat_tag.clone(),
                    upgrade.localization_string.clone(),
                    final_value,
                    min_roll * 1.0,
                    max_roll * 0.9,
                    1.0 - random_factor_raw,
                ));
            }
        }

        let mut weapons = weapon.variants.clone();
        weapons.push(crate::cache::CacheRivenWeaponVariant::from(&weapon));

        let mut stat_with_weapons = vec![];
        for wea in weapons.iter() {
            let mut stat_with_weapon = RivenStatWithWeapon::new(wea.name.clone());
            for i in 0..=8 {
                let new_stats: Vec<RivenSingleAttribute> = stats
                    .iter()
                    .map(|attr| {
                        let mut new_attr = attr.clone();
                        new_attr
                            .apply_rank_multiplier(wea.disposition / weapon.disposition, i as f64);
                        new_attr
                    })
                    .collect();
                stat_with_weapon.add_level_attributes(i, new_stats);
            }
            stat_with_weapons.push(stat_with_weapon);
        }
        println!("Generated {:?} stats for Riven Summary", stats);
        Ok(RivenSummary { stat_with_weapons })
    }
}

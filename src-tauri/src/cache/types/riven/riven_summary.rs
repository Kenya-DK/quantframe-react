use std::collections::HashMap;
use std::sync::LazyLock;

use entity::{
    enums::{RivenAttributeGrade, RivenGrade},
    stock_riven,
};
use serde::Serialize;
use utils::*;

use crate::{
    cache::{
        CacheRivenRolls, RivenFinancialSummary, RivenRollEvaluation, RivenSingleAttribute,
        RivenStatWithWeapon,
    },
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
#[derive(Debug, Serialize)]
pub struct RivenSummary {
    mastery_rank: i64,
    weapon_name: String,
    unique_name: String,
    sub_name: String,
    rerolls: i64,
    rank: i32,
    stat_with_weapons: Vec<RivenStatWithWeapon>,
    polarity: String,
    image: String,
    endo: i64,
    kuva: i64,
    roll_evaluation: RivenRollEvaluation,
    grade: RivenGrade,
    financial_summary: RivenFinancialSummary,
}

impl RivenSummary {
    pub fn new(
        unique_name: impl Into<String>,
        mastery_rank: i64,
        rerolls: i64,
        mut rank: i32,
        polarity: impl Into<String>,
        attributes: Vec<(String, f64, bool)>,
    ) -> Result<Self, Error> {
        let cache = states::cache_client()?;
        let riven_lookup = cache.riven();
        let unique_name = unique_name.into();
        let weapon = riven_lookup
            .get_weapon_by(&unique_name)
            .map_err(|e| e.with_location(get_location!()))?;

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
        let mut grads = vec![];
        let mut name_info = vec![];
        for (stat_tag, rolled_value, is_positive) in attributes.iter() {
            let upgrade = cache
                .riven()
                .get_upgrade_by(&weapon.upgrade_type, stat_tag)
                .map_err(|e| e.with_location(get_location!()))?;

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
            if rank == 0 && (adjusted_value - base_stat).abs() < 0.5 * adjusted_value {
                rank = 8;
            }

            let scaled_value = adjusted_value / (rank as f64 + 1.0) * 9.0;
            let mut random_factor_raw =
                ((scaled_value - base_stat * 0.9) / (base_stat * 0.2)).clamp(0.0, 1.0);

            let final_value = (scaled_value * 10.0).round() / 10.0;

            let (min_roll, max_roll) = if *is_positive {
                name_info.push((
                    upgrade.value,
                    upgrade.prefix.clone(),
                    upgrade.suffix.clone(),
                ));
                (base_stat * 0.9, base_stat * 1.1)
            } else {
                random_factor_raw = 1.0 - random_factor_raw;
                (base_stat * 1.1, base_stat * 0.9)
            };

            let mut grade = RivenAttributeGrade::Unknown;
            if let Some(god_roll) = &weapon.god_roll {
                grade = god_roll.get_graded_attribute(&upgrade.modifier_tag, *is_positive);
            }
            grads.push((*is_positive, grade.clone(), upgrade.modifier_tag.clone()));
            stats.push(RivenSingleAttribute::new(
                stat_tag.clone(),
                final_value,
                min_roll,
                max_roll,
                random_factor_raw,
                *is_positive,
                grade,
            ));
        }
        let mut weapons = vec![crate::cache::CacheRivenWeaponVariant::from(&weapon)];
        for variant_name in weapon.variants.iter() {
            let variant = riven_lookup.get_weapon_by(&variant_name.unique_name)?;
            weapons.push(crate::cache::CacheRivenWeaponVariant::from(&variant));
        }

        let mut stat_with_weapons = vec![];
        for wea in weapons.iter() {
            let mut stat_with_weapon = RivenStatWithWeapon::new(
                &wea.name,
                &weapon.unique_name,
                wea.disposition,
                wea.disposition_rank,
            );
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

        // Calculate Endo value from dissolving the Riven
        // Formula: (100 × (Mastery Rank - 8)) + ⌊22.5 × 2^Mod Rank⌋ + (200 × Rerolls) - 7
        let endo = (100 * (mastery_rank - 8))
            + ((22.5 * 2_f64.powi(rank as i32)).floor() as i64)
            + (200 * rerolls)
            - 7;

        // Calculate total Kuva cost based on rerolls
        // Kuva cost per cycle: 900, 1000, 1200, 1400, 1700, 2000, 2350, 2750, 3150, 3500 (10+)
        let kuva = if rerolls == 0 {
            0
        } else {
            let costs = [900, 1000, 1200, 1400, 1700, 2000, 2350, 2750, 3150];
            let mut total = 0;
            for i in 0..rerolls as usize {
                if i < costs.len() {
                    total += costs[i];
                } else {
                    total += 3500; // 10+ cycles cost 3500 each
                }
            }
            total
        };

        let riven_grade = match weapon.god_roll {
            Some(ref rolls) => rolls.get_graded_riven(grads),
            None => RivenGrade::Unknown,
        };

        // 0 is lowest value, 1 is prefix, 2 is suffix
        name_info.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let name = if total_buffs == 2 {
            format!(
                "{}{}",
                name_info[0].1.clone().unwrap_or_default(),
                name_info[1].2.clone().unwrap_or_default()
            )
        } else if total_buffs == 3 {
            format!(
                "{}-{}{}",
                name_info[1].1.clone().unwrap_or_default(),
                name_info[0].1.clone().unwrap_or_default(),
                name_info[2].2.clone().unwrap_or_default()
            )
        } else {
            "Unnamed Riven".to_string()
        };
        Ok(RivenSummary {
            weapon_name: weapon.name.clone(),
            unique_name: weapon.unique_name.clone(),
            sub_name: name,
            stat_with_weapons,
            mastery_rank,
            rerolls,
            polarity: polarity.into(),
            endo,
            rank,
            image: weapon.wfm_icon.clone(),
            kuva,
            grade: riven_grade,
            roll_evaluation: RivenRollEvaluation::default(),
            financial_summary: RivenFinancialSummary::default(),
        })
    }

    pub async fn generate_financial_summary(&mut self) -> Result<(), Error> {
        self.financial_summary =
            RivenFinancialSummary::new(&self.stat_with_weapons[0].unique_name).await?;
        Ok(())
    }
    pub fn evaluate_rolls(&mut self) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let starts = self.stat_with_weapons[0].by_level[&0]
            .iter()
            .map(|a| (a.url_name.clone(), a.positive))
            .collect::<Vec<_>>();

        self.roll_evaluation = cache
            .riven()
            .fill_roll_evaluation(&self.stat_with_weapons[0].unique_name, starts)?;
        Ok(())
    }
    pub fn grade_riven(&mut self) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let god_roll = if self.unique_name == "Unknown" {
            let weapon = cache
                .riven()
                .get_weapon_by(&self.unique_name)
                .map_err(|e| e.with_location(get_location!()))?;
            weapon.god_roll.clone()
        } else {
            let weapon = cache
                .riven()
                .get_weapon_by(&self.unique_name)
                .map_err(|e| e.with_location(get_location!()))?;
            weapon.god_roll.clone()
        };
        if let Some(rolls) = god_roll {
            let mut grads = vec![];
            for wea in self.stat_with_weapons.iter_mut() {
                for attr in wea.by_level.iter_mut() {
                    // let grade = rolls.get_graded_attribute(&attr.url_name, attr.positive);
                    // grads.push((attr.positive, grade, attr.url_name.clone()));
                }
                // let grade = rolls.get_graded_attribute(&attr.url_name, attr.positive);
                // grads.push((attr.positive, grade, attr.url_name.clone()));
            }
            self.grade = rolls.get_graded_riven(grads);
        } else {
            self.grade = RivenGrade::Unknown;
        }
        Ok(())
    }
}

impl Default for RivenSummary {
    fn default() -> Self {
        RivenSummary {
            weapon_name: "Unknown".to_string(),
            unique_name: "Unknown".to_string(),
            sub_name: "Unknown".to_string(),
            stat_with_weapons: vec![],
            mastery_rank: 0,
            rerolls: 0,
            polarity: "".to_string(),
            endo: 0,
            rank: 0,
            image: "".to_string(),
            kuva: 0,
            roll_evaluation: RivenRollEvaluation::default(),
            grade: RivenGrade::Unknown,
            financial_summary: RivenFinancialSummary::default(),
        }
    }
}

impl From<&stock_riven::Model> for RivenSummary {
    fn from(item: &stock_riven::Model) -> Self {
        let attributes = item
            .attributes
            .0
            .iter()
            .map(|a| (a.url_name.clone(), a.value, a.positive))
            .collect::<Vec<_>>();

        let rank = if let Some(sub_type) = &item.sub_type {
            if sub_type.rank.is_none() {
                0
            } else {
                sub_type.rank.unwrap_or(0) as i32
            }
        } else {
            0
        };

        match RivenSummary::new(
            &item.weapon_unique_name,
            item.mastery_rank,
            item.re_rolls,
            rank,
            item.polarity.clone(),
            attributes,
        ) {
            Ok(summary) => summary,
            Err(e) => {
                e.log("RivenSummary::from.log");
                RivenSummary::default()
            }
        }
    }
}

use entity::enums::{RivenAttributeGrade, RivenGrade};
use utils::{get_location, Error};
use wf_market::enums::polarity;

use crate::{
    cache::{modules::RivenModule, types::*, CacheState},
    wf_inventory::UpgradeFingerprint,
};

const BASE_RIVEN: f64 = 90.0;
const RIVEN_DENOM: f64 = 53687091.0;
const TWO_DIGIT_BONUS: f64 = 1.0;

// slice lookup is fine (small set)
const TWO_DIGIT_TAGS: &[&str] = &[
    "WeaponFactionDamageGrineer",
    "WeaponFactionDamageCorpus",
    "WeaponFactionDamageInfested",
    "WeaponMeleeFactionDamageGrineer",
    "WeaponMeleeFactionDamageCorpus",
    "WeaponMeleeFactionDamageInfested",
];

// --------------------------------------------------
// COSTS
// --------------------------------------------------

pub fn compute_riven_endo_cost(mr: i64, rerolls: i64, rank: i32) -> i64 {
    100 * (mr - 8) + (22.5 * 2_f64.powi(rank)).floor() as i64 + 200 * rerolls - 7
}

pub fn compute_riven_kuva_cost(rerolls: i64) -> i64 {
    const COSTS: [i64; 9] = [900, 1000, 1200, 1400, 1700, 2000, 2350, 2750, 3150];

    (0..rerolls as usize)
        .map(|i| COSTS.get(i).copied().unwrap_or(3500))
        .sum()
}

// --------------------------------------------------
// MULTIPLIERS
// --------------------------------------------------

pub fn lookup_riven_multipliers(buffs: usize, curses: usize) -> Result<&'static Modifier, Error> {
    let key = format!("B{}|C{}", buffs, curses);

    MODIFIERS.get(&key).ok_or_else(|| {
        Error::new(
            "RivenHelper::LookupRivenMultipliers",
            format!("Modifier not found for key {}", key),
            get_location!(),
        )
    })
}

// --------------------------------------------------
// MOD NAME
// --------------------------------------------------

pub fn build_riven_mod_name(attrs: &Vec<RivenSingleAttribute>, buffs: usize) -> String {
    let prefix = |i: usize| -> String { attrs[i].get_property_value("prefix", "NO_PREFIX".into()) };
    let suffix = |i: usize| -> String { attrs[i].get_property_value("suffix", "NO_SUFFIX".into()) };

    match buffs {
        2 => format!("{}{}", prefix(0), suffix(1)),
        3 => format!("{}-{}{}", prefix(0), prefix(1), suffix(2)),
        _ => "Unknown".into(),
    }
}

// --------------------------------------------------
// ATTRIBUTE BUILDERS
// --------------------------------------------------

pub fn build_riven_attributes_from_fingerprint(
    cache: &RivenModule,
    weapon: &CacheRivenWeapon,
    fingerprint: &UpgradeFingerprint,
    multipliers: &Modifier,
) -> Result<Vec<RivenSingleAttribute>, Error> {
    let mut out = Vec::with_capacity(fingerprint.buffs.len() + fingerprint.curses.len());

    for (stats, is_buff) in [(&fingerprint.buffs, true), (&fingerprint.curses, false)] {
        for raw in stats {
            let upgrade = cache
                .get_upgrade_by(&weapon.upgrade_type, &raw.tag)
                .map_err(|e| e.with_location(get_location!()))?;

            let value = derive_riven_roll_value(
                raw.value as f64,
                &raw.tag,
                upgrade.value,
                &upgrade.localization_string,
                weapon,
                multipliers,
                is_buff,
            );

            let mut attr = RivenSingleAttribute::new_base(
                &upgrade.wfm_url,
                &raw.tag,
                value,
                raw.value as f64,
                is_buff,
            );

            attr.set_property_value("suffix", upgrade.suffix.unwrap_or_default());
            attr.set_property_value("prefix", upgrade.prefix.unwrap_or_default());
            attr.set_property_value("display", format_riven_stat_value(attr.value));
            out.push(attr);
        }
    }

    Ok(out)
}

// --------------------------------------------------
// ATTRIBUTE MATH (core engine)
// --------------------------------------------------

pub fn derive_riven_roll_value(
    raw_value: f64,
    tag: &str,
    upgrade_base: f64,
    localization: &str,
    weapon: &CacheRivenWeapon,
    multipliers: &Modifier,
    is_buff: bool,
) -> f64 {
    let normalized = (0.9 + raw_value / RIVEN_DENOM / 100.0).clamp(0.9, 1.1);

    let mut value = BASE_RIVEN
        * upgrade_base
        * weapon.disposition
        * if is_buff {
            multipliers.good
        } else {
            multipliers.bad
        };

    if !is_buff && tag == "WeaponMeleeComboPointsOnHitMod" && value > 0.0 {
        value = -value;
    }

    if localization.contains('%') {
        value *= 100.0;
    }

    value *= normalized;

    if TWO_DIGIT_TAGS.contains(&tag) {
        value += TWO_DIGIT_BONUS;
    }

    value
}

// --------------------------------------------------
// DISPLAY
// --------------------------------------------------

pub fn format_riven_stat_value(v: f64) -> String {
    let r = (v * 10.0).round() / 10.0;
    if r.fract().abs() < 0.05 {
        (r.round() as i64).to_string()
    } else {
        format!("{:.1}", r)
    }
}

// --------------------------------------------------
// HELPERS
// --------------------------------------------------

pub fn count_riven_positive_and_negative_stats(attrs: &[(String, f64, bool)]) -> (usize, usize) {
    let buffs = attrs.iter().filter(|(_, _, p)| *p).count();
    (buffs, attrs.len() - buffs)
}

pub fn derive_riven_summary_attributes(
    cache: &CacheState,
    weapon: &CacheRivenWeapon,
    attributes: &[(String, f64, bool)],
    multipliers: &Modifier,
    rank: &mut i32,
) -> Result<Vec<RivenSingleAttribute>, Error> {
    let riven_cache = cache.riven();
    let mut out = Vec::with_capacity(attributes.len());

    for (tag, rolled, positive) in attributes {
        let upgrade = riven_cache
            .get_upgrade_by(&weapon.upgrade_type, tag)
            .map_err(|e| e.with_location(get_location!()))?;

        let mut base = BASE_RIVEN
            * upgrade.value
            * weapon.disposition
            * if *positive {
                multipliers.good
            } else {
                multipliers.bad
            };

        if upgrade.localization_string.contains('%') {
            base *= 100.0;
        }

        if *rank == 0 && (rolled - base).abs() < 0.5 * rolled {
            *rank = 8;
        }

        let scaled = rolled / (*rank as f64 + 1.0) * 9.0;

        let mut factor = ((scaled - base * 0.9) / (base * 0.2)).clamp(0.0, 1.0);

        let (min, max) = if *positive {
            (base * 0.9, base * 1.1)
        } else {
            factor = 1.0 - factor;
            (base * 1.1, base * 0.9)
        };

        let mut attr = RivenSingleAttribute::new(
            tag.clone(),
            upgrade.modifier_tag.clone(),
            scaled,
            min,
            max,
            factor,
            *positive,
            RivenAttributeGrade::Unknown,
        );

        attr.set_property_value("suffix", upgrade.suffix.unwrap_or_default());
        attr.set_property_value("prefix", upgrade.prefix.unwrap_or_default());

        out.push(attr);
    }

    Ok(out)
}

pub fn normalize_weapon_unique_name(unique_name: impl Into<String>) -> String {
    let unique_name = unique_name.into();
    match unique_name.as_str() {
        "/Lotus/Weapons/Tenno/Melee/Dagger/DarkDaggerBase" => {
            "/Lotus/Weapons/Tenno/Melee/Dagger/DarkDagger".to_string()
        }
        "/Lotus/Weapons/Tenno/Shotgun/QuadShotgunBase" => {
            "/Lotus/Weapons/Tenno/Shotgun/QuadShotgun".to_string()
        }
        _ => unique_name,
    }
}
pub fn normalize_polarity(polarity: impl Into<String>) -> String {
    let polarity = polarity.into();
    match polarity.as_str() {
        "AP_ATTACK" => "madurai".to_string(),
        "AP_DEFENSE" => "vazarin".to_string(),
        "AP_TACTIC" => "naramon".to_string(),
        _ => polarity,
    }
}

pub fn grade_riven(
    roll: &CacheRivenRolls,
    attrs: &Vec<RivenSingleAttribute>,
) -> (RivenGrade, Vec<(bool, RivenAttributeGrade, String)>) {
    let mut grads = vec![];
    for attr in attrs.iter() {
        let grade = roll.get_graded_attribute(&attr.tag, attr.positive);
        grads.push((attr.positive, grade.clone(), attr.tag.clone()));
    }
    (roll.get_graded_riven(&grads), grads)
}

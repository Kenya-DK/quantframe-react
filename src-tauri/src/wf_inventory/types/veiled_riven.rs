use entity::enums::RivenGrade;
use serde::{Deserialize, Serialize};
use utils::*;

use crate::{
    cache::{
        build_riven_attributes_from_fingerprint, build_riven_mod_name, compute_riven_endo_cost,
        compute_riven_kuva_cost, grade_riven, lookup_riven_multipliers, normalize_polarity,
        normalize_weapon_unique_name, CacheState, RivenSingleAttribute,
    },
    wf_inventory::UpgradeFingerprint,
};
static COMPONENT: &str = "VeiledRiven";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VeiledRiven {
    weapon_name: String,
    unique_name: String,
    mod_name: String,
    mastery_rank: i64,
    rank: i32,
    rerolls: i64,
    polarity: String,
    attributes: Vec<RivenSingleAttribute>,
    endo: i64,
    kuva: i64,
    grade: RivenGrade,
        // Default UUID
    pub uuid: String,
}

impl VeiledRiven {
    pub fn try_from_fingerprint(
        fingerprint: &UpgradeFingerprint,
        cache: &CacheState,
    ) -> Result<Self, Error> {
        let riven_cache = cache.riven();
        let mut raw_uuid = String::new();
        raw_uuid.push_str(&format!("type:{};", "0"));
        let weapon = match riven_cache.get_weapon_by(&normalize_weapon_unique_name(
            fingerprint.compatibility.clone(),
        )) {
            Ok(w) => w,
            Err(e) => {
                warning(
                    format!("{}:GetVeiledRivens", COMPONENT),
                    format!(
                        "Weapon not found for compatibility: {}",
                        normalize_weapon_unique_name(fingerprint.compatibility.clone())
                    ),
                    &LoggerOptions::default(),
                );
                return Err(e.with_location(get_location!()));
            }
        };
        raw_uuid.push_str(&format!("weapon:{};", weapon.wfm_weapon_url));
        let (buffs_total, curses_total) = fingerprint.riven_stat_totals();
        let multipliers = lookup_riven_multipliers(buffs_total, curses_total)?;
        let mut attributes = build_riven_attributes_from_fingerprint(
            &riven_cache,
            &weapon,
            &fingerprint,
            multipliers,
        )?;

        attributes.sort_by(|a, b| {
            b.raw_value
                .partial_cmp(&a.raw_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mod_name = build_riven_mod_name(&attributes, fingerprint.buffs.len());
        attributes.sort_by(|a, b| {
            b.positive
                .partial_cmp(&a.positive)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let grade = if let Some(rolls) = weapon.god_roll {
            let (grade, _) = grade_riven(&rolls, &attributes);
            grade
        } else {
            RivenGrade::Unknown
        };

        let polarity = normalize_polarity(fingerprint.polarity.clone());


        raw_uuid.push_str(&format!("mod_name:{};", mod_name));
        raw_uuid.push_str(&format!("re_rolls:{};", fingerprint.rerolls));
        raw_uuid.push_str(&format!("mastery:{};", fingerprint.mastery_rank));
        raw_uuid.push_str(&format!("polarity:{};", polarity));

        let mut sorted_attrs = attributes.clone();
        sorted_attrs.sort_by_key(|a| a.url_name.clone());
        for a in sorted_attrs {
            raw_uuid.push_str(&format!("attr:{}:{}:{};", a.get_property_value("wfm_url", String::new()), a.positive, a.value));
        }

        Ok(VeiledRiven {
            weapon_name: weapon.name.clone(),
            unique_name: weapon.unique_name.clone(),
            mod_name,
            mastery_rank: fingerprint.mastery_rank,
            rank: fingerprint.mod_rank as i32,
            rerolls: fingerprint.rerolls,
            attributes,
            polarity: normalize_polarity(fingerprint.polarity.clone()),
            endo: compute_riven_endo_cost(
                fingerprint.mastery_rank,
                fingerprint.rerolls,
                fingerprint.mod_rank as i32,
            ),
            kuva: compute_riven_kuva_cost(fingerprint.rerolls),
            grade,
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, raw_uuid.as_bytes()).to_string(),
        })
    }
}

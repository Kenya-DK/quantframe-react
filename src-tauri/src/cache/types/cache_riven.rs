use entity::enums::{RivenAttributeGrade, RivenGrade};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CacheRiven {
    #[serde(rename = "weapons")]
    pub weapons: Vec<CacheRivenWeapon>,
    #[serde(rename = "upgrade_types_dict")]
    pub upgrade_types: HashMap<String, Vec<CacheRivenUpgrade>>,
    #[serde(rename = "attributes")]
    pub attributes: Vec<CacheRivenWFMAttribute>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenWeapon {
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,
    #[serde(rename = "wfm_url_name")]
    pub wfm_url_name: String,
    #[serde(rename = "wfm_group")]
    pub wfm_group: String,
    #[serde(rename = "riven_type")]
    pub riven_type: String,
    #[serde(rename = "wfm_icon")]
    pub wfm_icon: String,
    #[serde(rename = "is_variant")]
    pub is_variant: bool,
    #[serde(rename = "wfm_icon_format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfm_icon_format: Option<String>,
    #[serde(rename = "wfm_thumb")]
    pub wfm_thumb: String,
    #[serde(rename = "unique_name")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "disposition")]
    pub disposition: f64,
    #[serde(rename = "disposition_rank")]
    pub disposition_rank: i64,
    #[serde(rename = "upgrade_type")]
    pub upgrade_type: String,
    #[serde(rename = "rolls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub god_roll: Option<CacheRivenRolls>,
    #[serde(default)]
    pub variants: Vec<CacheRivenWeaponVariant>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenWeaponVariant {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "disposition", default)]
    pub disposition: f64,
    #[serde(rename = "disposition_rank", default)]
    pub disposition_rank: i64,
}
impl From<&CacheRivenWeapon> for CacheRivenWeaponVariant {
    fn from(item: &CacheRivenWeapon) -> Self {
        CacheRivenWeaponVariant {
            unique_name: item.unique_name.clone(),
            name: item.name.clone(),
            disposition: item.disposition,
            disposition_rank: item.disposition_rank,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenGoodRoll {
    #[serde(rename = "required")]
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(rename = "optional")]
    #[serde(default)]
    pub optional: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenRolls {
    #[serde(rename = "weapon_url_name")]
    pub weapon_url_name: String,
    #[serde(rename = "good_rolls")]
    pub good_rolls: Vec<CacheRivenGoodRoll>,
    #[serde(rename = "negative_attributes")]
    pub negative_attributes: Vec<String>,
}
impl CacheRivenRolls {
    pub fn get_graded_attribute(
        &self,
        tag: impl Into<String>,
        positive: bool,
    ) -> RivenAttributeGrade {
        let tag = tag.into();
        if !positive {
            if self.negative_attributes.contains(&tag) {
                RivenAttributeGrade::Good
            } else if self
                .good_rolls
                .iter()
                .any(|r| r.required.contains(&tag) || r.optional.contains(&tag))
            {
                RivenAttributeGrade::Bad
            } else {
                RivenAttributeGrade::NotHelping
            }
        } else {
            if self.good_rolls.iter().any(|r| r.required.contains(&tag)) {
                RivenAttributeGrade::Decisive
            } else if self.good_rolls.iter().any(|r| r.optional.contains(&tag)) {
                RivenAttributeGrade::Good
            } else {
                RivenAttributeGrade::NotHelping
            }
        }
    }
    pub fn get_graded_riven(
        &self,
        grades: &Vec<(bool, RivenAttributeGrade, String)>,
    ) -> RivenGrade {
        let buffs_tags: HashSet<String> = grades
            .iter()
            .filter(|(positive, _, _)| *positive)
            .map(|(_, _, tag)| tag.clone())
            .collect();
        let matching_rolls = self
            .good_rolls
            .iter()
            .filter(|roll| {
                // All required attributes must be present in buffs_tags
                roll.required.iter().all(|tag| buffs_tags.contains(tag))
            })
            .filter(|roll| {
                // Count of (required + optional) attributes that match must equal total buffs count
                let required_match_count = roll
                    .required
                    .iter()
                    .filter(|tag| buffs_tags.contains(*tag))
                    .count();
                let optional_match_count = roll
                    .optional
                    .iter()
                    .filter(|tag| buffs_tags.contains(*tag))
                    .count();
                required_match_count + optional_match_count == buffs_tags.len()
            })
            .collect::<Vec<_>>();

        let has_full_pattern_match = !matching_rolls.is_empty();

        let useful_positive_count = grades
            .iter()
            .filter(|(positive, grade, _)| {
                *positive
                    && matches!(
                        grade,
                        RivenAttributeGrade::Decisive | RivenAttributeGrade::Good
                    )
            })
            .count();

        let has_bad_negatives = grades
            .iter()
            .any(|(positive, grade, _)| !*positive && matches!(grade, RivenAttributeGrade::Bad));

        let has_not_helping_negatives = grades.iter().any(|(positive, grade, _)| {
            !*positive && matches!(grade, RivenAttributeGrade::NotHelping)
        });

        let negative_grades: Vec<_> = grades
            .iter()
            .filter(|(positive, _, _)| !*positive)
            .collect();

        if has_bad_negatives {
            if (has_full_pattern_match && useful_positive_count >= 2) || useful_positive_count >= 3
            {
                return RivenGrade::HasPotential;
            } else {
                return RivenGrade::Bad;
            }
        }

        if has_not_helping_negatives {
            if has_full_pattern_match || useful_positive_count >= 2 {
                return RivenGrade::Good;
            } else if useful_positive_count >= 1 {
                return RivenGrade::HasPotential;
            } else {
                return RivenGrade::Bad;
            }
        }

        if has_full_pattern_match {
            if useful_positive_count >= 2 && !negative_grades.is_empty() {
                return RivenGrade::Perfect;
            } else {
                return RivenGrade::Good;
            }
        }

        if useful_positive_count >= 2 {
            return RivenGrade::Good;
        }

        if useful_positive_count >= 1 {
            return RivenGrade::HasPotential;
        } else {
            return RivenGrade::Bad;
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenUpgrade {
    #[serde(rename = "wfm_url")]
    pub wfm_url: String,
    #[serde(rename = "modifier_tag")]
    pub modifier_tag: String,
    #[serde(rename = "prefixTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suffixTag")]
    pub suffix: Option<String>,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "localization_string")]
    pub localization_string: String,
    #[serde(rename = "short_string")]
    pub short_string: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenWFMAttribute {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "uniqueName")]
    pub unique_name: String,

    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "prefix")]
    pub prefix: String,

    #[serde(rename = "suffix")]
    pub suffix: String,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unit")]
    pub unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exclusiveTo")]
    pub exclusive_to: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positiveIsNegative")]
    pub positive_is_negative: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "positiveOnly")]
    pub positive_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "negativeOnly")]
    pub negative_only: Option<bool>,

    #[serde(rename = "full", default)]
    pub full: String,

    #[serde(rename = "short", default)]
    pub short: String,

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "text", default)]
    pub text: String,
}

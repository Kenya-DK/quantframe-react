use std::collections::HashSet;

use entity::enums::{RivenAttributeGrade, RivenGrade};
use serde::{Deserialize, Serialize};
use utils::Error;

use crate::cache::{
    AttributeMatch, CacheRivenGoodRoll, CacheState, RivenRollEvaluation, RollCriteria,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheRivenRoll {
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "goodRolls")]
    pub good_rolls: Vec<CacheRivenGoodRoll>,
    #[serde(rename = "negativeAttributes")]
    pub negative_attributes: Vec<String>,
}

impl CacheRivenRoll {
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

    pub fn fill_roll_evaluation(
        &self,
        upgrade_type: impl Into<String>,
        stats: Vec<(String, f64, bool)>,
        cache: &CacheState,
    ) -> Result<RivenRollEvaluation, Error> {
        let mut summary = RivenRollEvaluation::default();
        let upgrade_type = upgrade_type.into();
        let stat_lookup: HashSet<(&str, bool)> =
            stats.iter().map(|(s, _, p)| (s.as_str(), *p)).collect();

        let has_stat =
            |name: &str, positive: bool| -> bool { stat_lookup.contains(&(name, positive)) };

        let resolve_attr = |attr: &str, positive: bool| -> AttributeMatch {
            match cache.mods().get_stat_tag_by(&upgrade_type, attr) {
                Ok(att) => AttributeMatch::new(&att.label, has_stat(&att.wfm_url, positive)),
                Err(e) => AttributeMatch::new(e.to_string(), false),
            }
        };

        // Negative attributes
        for bad in &self.negative_attributes {
            let att = resolve_attr(bad, false);
            summary.add_negative_attribute(att.label, att.matches);
        }

        // Good rolls
        for roll in self.good_rolls.iter() {
            let mut roll_summary = RollCriteria::default();

            roll_summary.required = roll
                .required
                .iter()
                .map(|r| resolve_attr(r, true))
                .collect();

            roll_summary.optional = roll
                .optional
                .iter()
                .map(|o| resolve_attr(o, true))
                .collect();

            summary.add_valid_roll(roll_summary);
        }

        Ok(summary)
    }
}

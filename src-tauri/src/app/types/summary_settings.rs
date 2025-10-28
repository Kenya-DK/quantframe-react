use serde::{Deserialize, Serialize};

use crate::app::SummaryCategorySetting;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummarySettings {
    pub recent_days: i64,         // How many days to keep the summary
    pub recent_transactions: i64, // How many transactions to keep in the summary
    pub categories: Vec<SummaryCategorySetting>,
}

impl Default for SummarySettings {
    fn default() -> Self {
        SummarySettings {
            recent_days: 7,
            recent_transactions: 10,
            categories: vec![
                SummaryCategorySetting::new(
                    "/imgs/categories/mods.png",
                    "Mod",
                    vec![],
                    vec!["mod"],
                ),
                SummaryCategorySetting::new(
                    "/imgs/categories/arcane.png",
                    "Arcane",
                    vec![],
                    vec!["arcane_enhancement"],
                ),
                SummaryCategorySetting::new("/imgs/categories/set.png", "Set", vec![], vec!["set"]),
                SummaryCategorySetting::new(
                    "/imgs/categories/prime.png",
                    "Prime",
                    vec![],
                    vec!["prime"],
                ),
                SummaryCategorySetting::new(
                    "/imgs/categories/axi-nectar.png",
                    "Relic",
                    vec![],
                    vec!["relic"],
                ),
                SummaryCategorySetting::new(
                    "/imgs/categories/rivenIcon2.png",
                    "Riven",
                    vec!["riven"],
                    vec![],
                ),
            ],
        }
    }
}

use entity::{
    dto::PriceHistory,
    enums::{RivenGrade, StockStatus},
    stock_riven,
};
use serde::Serialize;
use serde_json::json;
use utils::*;
use wf_market::types::AuctionLike;

static COMPONENT: &str = "RivenHelper";

use crate::{
    cache::*,
    utils::{modules::states, ErrorFromExt},
    wf_inventory::UpgradeFingerprint,
};
#[derive(Debug, Serialize, Clone)]
pub struct RivenSummary {
    // Base Info
    weapon_name: String,
    unique_name: String,
    mod_name: String,
    mastery_rank: i64,
    rank: i32,
    rerolls: i64,
    polarity: String,
    attributes: Vec<RivenSingleAttribute>,
    // Extra Info
    stock_status: Option<StockStatus>,
    stat_with_weapons: Vec<RivenStatWithWeapon>,
    image: String,
    endo: i64,
    kuva: i64,
    roll_evaluation: RivenRollEvaluation,
    grade: RivenGrade,
    financial_summary: RivenFinancialSummary,
    similarly_auctions: Vec<wf_market::types::AuctionWithOwner>,
    price_history: Vec<PriceHistory>,
}

impl RivenSummary {
    pub fn new(
        cache: &CacheState,
        unique_name: impl Into<String>,
        mastery_rank: i64,
        rerolls: i64,
        mut rank: i32,
        polarity: impl Into<String>,
        attributes: Vec<(String, f64, bool)>,
    ) -> Result<Self, Error> {
        let unique_name = unique_name.into();

        let weapon = cache
            .riven()
            .get_weapon_by(&unique_name)
            .map_err(|e| e.with_location(get_location!()))?;

        // -----------------------------
        // Count buffs / curses
        // -----------------------------
        let (total_buffs, total_curses) = count_riven_positive_and_negative_stats(&attributes);

        // -----------------------------
        // Multipliers
        // -----------------------------
        let multipliers = lookup_riven_multipliers(total_buffs, total_curses)?;

        // -----------------------------
        // Build attributes
        // -----------------------------
        let stats =
            derive_riven_summary_attributes(&cache, &weapon, &attributes, multipliers, &mut rank)?;

        // -----------------------------
        // Build summary
        // -----------------------------
        Ok(RivenSummary {
            weapon_name: weapon.name.clone(),
            unique_name: weapon.unique_name.clone(),
            polarity: polarity.into(),
            mod_name: build_riven_mod_name(&stats, total_buffs),
            mastery_rank,
            rank,
            rerolls,
            attributes: stats.to_vec(),
            image: weapon.wfm_icon.clone(),
            endo: compute_riven_endo_cost(mastery_rank, rerolls, rank),
            kuva: compute_riven_kuva_cost(rerolls),
            ..Default::default()
        })
    }

    pub fn evaluate_weapon_variants(&mut self, cache: &CacheState) -> Result<(), Error> {
        let riven_lookup = cache.riven();
        let weapon = riven_lookup
            .get_weapon_by(&self.unique_name)
            .map_err(|e| e.with_location(get_location!()))?;

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
                let new_stats: Vec<RivenSingleAttribute> = self
                    .attributes
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
        self.stat_with_weapons = stat_with_weapons;
        Ok(())
    }
    pub fn evaluate_rolls(&mut self, cache: &CacheState) -> Result<(), Error> {
        let starts = self
            .attributes
            .iter()
            .map(|a| (a.url_name.clone(), a.positive))
            .collect::<Vec<_>>();

        self.roll_evaluation = cache
            .riven()
            .fill_roll_evaluation(&self.stat_with_weapons[0].unique_name, starts)?;
        Ok(())
    }
    pub fn grade_riven(&mut self, cache: &CacheState) -> Result<(), Error> {
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
            let (grade, grads) = grade_riven(&rolls, &self.attributes);

            for wea in self.stat_with_weapons.iter_mut() {
                for by_level in wea.by_level.iter_mut() {
                    let mut index = 0;
                    for attr in by_level.1.iter_mut() {
                        attr.grade = grads[index].1.clone();
                        index += 1;
                    }
                }
            }
            self.grade = grade;
        } else {
            self.grade = RivenGrade::Unknown;
        }
        Ok(())
    }
    pub async fn find_similar_auctions(&mut self, cache: &CacheState) -> Result<(), Error> {
        let app = states::app_state().map_err(|e| e.with_location(get_location!()))?;

        let attributes = self.stat_with_weapons[0].by_level.get(&0).ok_or_else(|| {
            Error::new(
                "RivenSummary::grade_riven",
                "No level 0 attributes found",
                get_location!(),
            )
        })?;

        let weapon = cache
            .riven()
            .get_weapon_by(&self.unique_name)
            .map_err(|e| e.with_location(get_location!()))?;

        let mut filter = wf_market::types::AuctionFilter::new(
            wf_market::enums::AuctionType::Riven,
            &weapon.wfm_url_name,
        );
        filter.similarity_attributes = Some(
            attributes
                .iter()
                .map(|att| {
                    wf_market::types::ItemAttribute::new(
                        att.url_name.clone(),
                        att.positive,
                        att.value,
                    )
                })
                .collect(),
        );
        filter.similarity = Some(34);

        let mut auctions = app
            .wfm_client
            .auction()
            .search_auctions(filter)
            .await
            .map_err(|e| {
                Error::from_wfm(
                    "RivenSummary::FindSimilarAuctions",
                    "Failed to search auctions",
                    e,
                    get_location!(),
                )
            })?;
        auctions.sort_by_similarity(false);
        let mut auctions = auctions.to_vec();
        for auction in auctions.iter_mut().map(|auction| auction.to_auction_mut()) {
            let similarity = auction.item.similarity.clone();
            if let Some(attrs) = &mut auction.item.attributes {
                for attr in attrs.iter_mut() {
                    attr.properties =
                        Some(json!({"matched": similarity.has_attribute(&attr.url_name)}));
                }
            }
        }

        self.similarly_auctions = auctions;
        Ok(())
    }
    pub async fn try_from_model(
        item: &stock_riven::Model,
        cache: &CacheState,
    ) -> Result<Self, Error> {
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
            cache,
            &item.weapon_unique_name,
            item.mastery_rank,
            item.re_rolls,
            rank,
            item.polarity.clone(),
            attributes,
        ) {
            Ok(mut summary) => {
                summary.financial_summary = RivenFinancialSummary::try_from_model(item).await?;
                summary.evaluate_weapon_variants(cache)?;
                summary.evaluate_rolls(cache)?;
                summary.grade_riven(cache)?;
                summary.find_similar_auctions(cache).await?;
                summary.price_history = item.price_history.0.clone();
                summary.stock_status = Some(item.status.clone());
                Ok(summary)
            }
            Err(e) => {
                e.log("RivenSummary::from.log");
                Err(e)
            }
        }
    }
}

impl Default for RivenSummary {
    fn default() -> Self {
        RivenSummary {
            weapon_name: "Unknown".to_string(),
            unique_name: "Unknown".to_string(),
            mod_name: "Unknown".to_string(),
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
            similarly_auctions: vec![],
            price_history: vec![],
            stock_status: None,
            attributes: vec![],
        }
    }
}

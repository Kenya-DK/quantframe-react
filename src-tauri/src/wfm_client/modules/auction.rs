use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::{
    error::{self, AppError},
    helper, logger,
    structs::{
        Auction, AuctionItem, AuctionOwner, Item, ItemDetails, RivenAttribute, RivenAttributeInfo,
        RivenTypeInfo,
    },
    wfm_client::client::WFMClient,
};

pub struct AuctionModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> AuctionModule<'a> {
    pub async fn get_all_riven_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        let (payload, _headers) = self.client.get("riven/items", Some("items")).await?;
        Ok(payload)
    }
    pub async fn get_all_riven_attribute_types(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        let (payload, _headers) = self
            .client
            .get("riven/attributes", Some("attributes"))
            .await?;
        Ok(payload)
    }

    // User methods (sea-query)
    pub async fn get_user_auctions(
        &self,
        ingame_name: &str,
    ) -> Result<Vec<Auction<String>>, AppError> {
        let url = format!("profile/{}/auctions", ingame_name);
        match self.client.get(&url, Some("auctions")).await {
            Ok((orders, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("From User Auctions: {}", ingame_name).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                Ok(orders)
            }
            Err(e) => {
                error::create_log_file("wfm.log".to_string(), &e);
                Err(e)
            }
        }
    }

    pub async fn get_my_auctions(&self) -> Result<Vec<Auction<String>>, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let auctions = self.get_user_auctions(auth.ingame_name.as_str()).await?;
        Ok(auctions)
    }

    pub async fn get_auction_by_id(
        &self,
        auction_id: &str,
    ) -> Result<Option<Auction<String>>, AppError> {
        let auctions = self.get_my_auctions().await?;

        let auction = auctions
            .iter()
            .find(|auction| auction.id == auction_id)
            .clone();
        Ok(auction.cloned())
    }
    pub async fn create(
        &self,
        auction_type: &str,
        note: &str,
        starting_price: i64,
        buyout_price: i64,
        minimal_reputation: i64,
        minimal_increment: i64,
        private: bool,
        item: AuctionItem,
    ) -> Result<Auction<String>, AppError> {
        // Construct any JSON body
        let mut body = json!({
            "note": note,
            "starting_price": starting_price,
            "buyout_price": buyout_price,
            "minimal_reputation": minimal_reputation,
            "minimal_increment": minimal_increment,
            "private": private
        });

        if auction_type == "riven" {
            let item_riven = json!({
                "type": "riven",
                "re_rolls": item.re_rolls,
                "attributes": item.attributes,
                "name": item.name,
                "weapon_url_name": item.weapon_url_name,
                "mod_rank": item.mod_rank,
                "polarity": item.polarity,
                "mastery_level": item.mastery_level,
            });
            body["item"] = item_riven;
        } else if auction_type == "item" {
        }

        match self
            .client
            .post("auctions/create", Some("auction"), body)
            .await
        {
            Ok((auction, _headers)) => {
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&auction).unwrap());
                Ok(auction)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update(
        &self,
        auction_id: &str,
        buyout_price: i32,
        minimal_reputation: i32,
        note: &str,
        starting_price: i32,
        visible: bool,
    ) -> Result<Auction<String>, AppError> {
        // Construct any JSON body
        let body = json!({
            "buyout_price": buyout_price,
            "minimal_reputation": minimal_reputation,
            "note": note,
            "starting_price": starting_price,
            "visible": visible
        });
        let url = format!("auctions/entry/{}", auction_id);
        match self.client.put(&url, Some("auction"), Some(body)).await {
            Ok((order, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("Update Auction: {}", auction_id).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&order).unwrap());
                Ok(order)
            }
            Err(e) => Err(e),
        }
    }
    pub async fn search(
        &self,
        auction_type: &str,
        weapon_url_name: &str,
        positive_stats: Option<Vec<RivenAttribute>>,
        negative_stats: Option<RivenAttribute>,
        polarity: Option<&str>,
        mastery_rank_min: Option<u8>,
        mastery_rank_max: Option<u8>,
        re_rolls_min: Option<u8>,
        re_rolls_max: Option<u8>,
        buyout_policy: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<Vec<Auction<AuctionOwner>>, AppError> {
        let base_url = format!("auctions/search?type={}", auction_type);

        let mut query_params = Vec::new();
        query_params.push(format!("weapon_url_name={}", weapon_url_name));

        if let Some(stats) = positive_stats {
            if stats.len() > 1 {
                let mut stats_vec = Vec::new();
                for stat in stats {
                    stats_vec.push(stat.url_name);
                }
                query_params.push(format!("positive_stats={}", stats_vec.join(",")));
            }
        }
        if let Some(stats) = negative_stats {
            query_params.push(format!("negative_stats={}", stats.url_name));
        }
        if let Some(p) = polarity {
            query_params.push(format!("polarity={}", p));
        }
        if let Some(rank) = mastery_rank_min {
            query_params.push(format!("mastery_rank_min={}", rank));
        }
        if let Some(rank) = mastery_rank_max {
            query_params.push(format!("mastery_rank_max={}", rank));
        }
        if let Some(rolls) = re_rolls_min {
            query_params.push(format!("re_rolls_min={}", rolls));
        }
        if let Some(rolls) = re_rolls_max {
            query_params.push(format!("re_rolls_max={}", rolls));
        }
        if let Some(policy) = buyout_policy {
            query_params.push(format!("buyout_policy={}", policy));
        }
        if let Some(sort) = sort_by {
            query_params.push(format!("sort_by={}", sort));
        }

        let full_query = query_params.join("&");
        let url = format!("{}&{}", base_url, full_query);
        match self.client.get(&url, Some("auctions")).await {
            Ok((order, _headers)) => {
                logger::info_con(
                    "WarframeMarket",
                    format!("Auctions Search: {}", url).as_str(),
                );
                Ok(order)
            }
            Err(e) => Err(e),
        }
    }
    pub async fn delete(&self, auction_id: &str) -> Result<Option<String>, AppError> {
        let url = format!("auctions/entry/{}/close", auction_id);
        match self.client.put(&url, Some("auction_id"), None).await {
            Ok((id, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("Delete Auction: {}", auction_id).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                self.emit("DELETE", json!(auction_id));
                Ok(id)
            }
            Err(e) => Err(e),
        }
    }
    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("auctions", operation, Some(data));
    }
}

// Taurei Commands
#[tauri::command]
pub async fn auction_search(
    auction_type: &str,
    weapon_url_name: &str,
    positive_stats: Option<Vec<RivenAttribute>>,
    negative_stats: Option<RivenAttribute>,
    polarity: Option<&str>,
    mastery_rank_min: Option<u8>,
    mastery_rank_max: Option<u8>,
    re_rolls_min: Option<u8>,
    re_rolls_max: Option<u8>,
    buyout_policy: Option<&str>,
    sort_by: Option<&str>,
    wfm: tauri::State<'_, Arc<Mutex<WFMClient>>>,
) -> Result<Vec<Auction<AuctionOwner>>, AppError> {
    let wfm = wfm.lock()?.clone();
    let live_auctions = wfm
        .auction()
        .search(
            auction_type,
            weapon_url_name,
            positive_stats,
            negative_stats,
            polarity,
            mastery_rank_min,
            mastery_rank_max,
            re_rolls_min,
            re_rolls_max,
            buyout_policy,
            sort_by,
        )
        .await?;
    Ok(live_auctions)
}

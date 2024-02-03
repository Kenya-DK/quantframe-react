use std::sync::{Arc, Mutex};

use eyre::eyre;
use serde_json::json;

use crate::{
    error::{ApiResult, AppError},
    helper, logger,
    structs::{
        Auction, AuctionItem, AuctionOwner, RivenAttribute, RivenAttributeInfo, RivenTypeInfo,
    },
    wfm_client::client::WFMClient,
};

pub struct AuctionModule<'a> {
    pub client: &'a WFMClient,
    pub debug_id: String,
}

impl<'a> AuctionModule<'a> {
    pub async fn get_all_riven_types(&self) -> Result<Vec<RivenTypeInfo>, AppError> {
        match self
            .client
            .get::<Vec<RivenTypeInfo>>("riven/items", Some("items"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Auction:GetAllRivenTypes",
                    format!("Found {} riven types", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Auction:GetAllRivenTypes",
                    error,
                    eyre!("There was an error getting all riven types"),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    pub async fn get_all_riven_attribute_types(&self) -> Result<Vec<RivenAttributeInfo>, AppError> {
        match self
            .client
            .get::<Vec<RivenAttributeInfo>>("riven/attributes", Some("attributes"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Auction:GetAllRivenAttributeTypes",
                    format!("Found {} attributes", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Auction:GetAllRivenAttributeTypes",
                    error,
                    eyre!("There was an error getting all riven attribute types"),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    // User methods (sea-query)
    pub async fn get_user_auctions(
        &self,
        ingame_name: &str,
    ) -> Result<Vec<Auction<String>>, AppError> {
        let url = format!("profile/{}/auctions", ingame_name);

        match self
            .client
            .get::<Vec<Auction<String>>>(&url, Some("auctions"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Auction:GetUsersAuctions",
                    format!("Found {} auctions", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Auction:GetUsersAuctions",
                    error,
                    eyre!(
                        "There was an error getting all auctions for user: {}",
                        ingame_name
                    ),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn get_my_auctions(&self) -> Result<Vec<Auction<String>>, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let auctions = self.get_user_auctions(auth.ingame_name.as_str()).await?;
        Ok(auctions)
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
            logger::warning_con(
                "WarframeMarket:Auction:Create",
                "Item auctions are not yet supported",
            );
        }

        match self
            .client
            .post("auctions/create", Some("auction"), body)
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&payload).unwrap());
                self.client.debug(
                    &self.debug_id,
                    "Auction:Create",
                    format!(
                        "Created auction for type: {} for item: {}",
                        auction_type,
                        item.name.unwrap_or("None".to_string())
                    )
                    .as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Auction:Create",
                    error,
                    eyre!("There was an error creating the auction"),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
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
            Ok(ApiResult::Success(payload, _headers)) => {
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&payload).unwrap());
                self.client.debug(
                    &self.debug_id,
                    "Auction:Update",
                    format!(
                        "Updated auction: {} to buyout price: {}",
                        auction_id, buyout_price
                    )
                    .as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Auction:Update",
                    error,
                    eyre!("There was an error updating the auction"),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
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

        match self
            .client
            .get::<Vec<Auction<AuctionOwner>>>(&url, Some("auctions"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Auction:Search",
                    format!(
                        "Found {} auctions using query: {}",
                        &payload.len(),
                        full_query
                    )
                    .as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.status_code {
                    400 => crate::enums::LogLevel::Warning,
                    _ => crate::enums::LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    "Auction:Search",
                    error,
                    eyre!("There was an error searching for auctions."),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    pub async fn delete(&self, auction_id: &str) -> Result<Option<String>, AppError> {
        let url = format!("auctions/entry/{}/close", auction_id);

        match self.client.put(&url, Some("auction_id"), None).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&payload).unwrap());
                self.client.debug(
                    &self.debug_id,
                    "Auction:Delete",
                    format!("Deleted auction: {}", auction_id).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.messages.get(0) {
                    Some(message)
                        if message.contains("app.form.not_exist")
                            || message.contains("app.form.invalid") =>
                    {
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    "Auction:Delete",
                    error,
                    eyre!("There was an error deleting the auction"),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
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

use entity::stock::riven::attribute::RivenAttribute;
use eyre::eyre;
use serde_json::json;

use crate::{
    logger,
    utils::{
        enums::log_level::LogLevel,
        modules::error::{self, ApiResult, AppError},
    },
    wfm_client::{
        client::WFMClient,
        types::{auction::Auction, auction_item::AuctionItem, auction_owner::AuctionOwner},
    },
};
#[derive(Clone, Debug)]
pub struct AuctionModule {
    pub client: WFMClient,
    pub debug_id: String,
    pub total_auctions: i64,
    component: String,
}

impl AuctionModule {
    pub fn new(client: WFMClient) -> Self {
        AuctionModule {
            client,
            debug_id: "wfm_client_auction".to_string(),
            total_auctions: 0,
            component: "Auctions".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_auction_module(self.clone());
    }
    pub fn set_auction_count(&mut self, increment: i64) -> Result<(), AppError> {
        let ref mut count = self.total_auctions;
        *count = increment;
        self.update_state();
        Ok(())
    }

    pub fn subtract_auction_count(&mut self, increment: i64) -> Result<(), AppError> {
        let ref mut count = self.total_auctions;
        *count -= increment;
        if *count < 0 {
            *count = 0;
        }
        self.update_state();
        Ok(())
    }

    pub fn add_auction_count(&mut self, increment: i64) -> Result<(), AppError> {
        let ref mut count = self.total_auctions;
        *count += increment;
        self.update_state();
        Ok(())
    }

    // User methods (sea-query)
    pub async fn get_user_auctions(
        &mut self,
        ingame_name: &str,
    ) -> Result<Vec<Auction<String>>, AppError> {
        let url = format!("profile/{}/auctions", ingame_name);
        self.client.auth().is_logged_in()?;
        match self
            .client
            .get::<Vec<Auction<String>>>(&url, Some("auctions"))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetUsersAuctions"),
                    format!("Found {} auctions", payload.len()).as_str(),
                    None,
                );
                self.set_auction_count(payload.len() as i64)?;
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetUsersAuctions"),
                    error,
                    eyre!(
                        "There was an error getting all auctions for user: {}",
                        ingame_name
                    ),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn get_my_auctions(&mut self) -> Result<Vec<Auction<String>>, AppError> {
        self.client.auth().is_logged_in()?;
        let auth = self.client.auth.lock()?.clone();
        let auctions = self.get_user_auctions(auth.ingame_name.as_str()).await?;
        Ok(auctions)
    }

    pub async fn get_auction_by_id(
        &self,
        auction_id: &str,
    ) -> Result<Option<Auction<String>>, AppError> {
        match self.client.auction().get_my_auctions().await {
            Ok(auctions) => {
                for auction in auctions {
                    if auction.id == auction_id {
                        return Ok(Some(auction));
                    }
                }
                return Ok(None);
            }
            Err(e) => {
                error::create_log_file("get_auction_by_id.log".to_string(), &e);
                return Err(e);
            }
        }
    }

    pub async fn create(
        &mut self,
        auction_type: &str,
        note: &str,
        starting_price: i64,
        buyout_price: i64,
        minimal_reputation: i64,
        minimal_increment: i64,
        private: bool,
        item: AuctionItem,
    ) -> Result<Auction<String>, AppError> {
        self.client.auth().is_logged_in()?;
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
                self.add_auction_count(1)?;
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Create"),
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
                    &self.get_component("Create"),
                    error,
                    eyre!("There was an error creating the auction"),
                    LogLevel::Error,
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
        self.client.auth().is_logged_in()?;
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
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Update"),
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
                    &self.get_component("Update"),
                    error,
                    eyre!("There was an error updating the auction"),
                    LogLevel::Error,
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
        positive_stats: Option<Vec<String>>,
        negative_stats: Option<String>,
        polarity: Option<&str>,
        mastery_rank_min: Option<i64>,
        mastery_rank_max: Option<i64>,
        re_rolls_min: Option<i64>,
        re_rolls_max: Option<i64>,
        buyout_policy: Option<&str>,
        sort_by: Option<&str>,
        attributes: Option<Vec<RivenAttribute>>,
    ) -> Result<Vec<Auction<AuctionOwner>>, AppError> {
        let base_url = format!("auctions/search?type={}", auction_type);

        let mut query_params = Vec::new();
        query_params.push(format!("weapon_url_name={}", weapon_url_name));

        if let Some(stats) = positive_stats {
            if stats.len() > 1 {
                query_params.push(format!("positive_stats={}", stats.join(",")));
            }
        }
        if let Some(stats) = negative_stats {
            query_params.push(format!("negative_stats={}", stats));
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
                    &self.get_component("Search"),
                    format!(
                        "Found {} auctions using query: {}",
                        &payload.len(),
                        full_query
                    )
                    .as_str(),
                    None,
                );
                if attributes.is_none() {
                    return Ok(payload);
                }
                let mut new_payload = vec![];
                for mut auction in payload.into_iter() {
                    let (similarity, extra_attributes, missing_attributes) = self
                        .calculate_shared_attribute_percentages(
                            attributes.clone().unwrap(),
                            &auction.item,
                        );
                    auction.item.similarity = Some(similarity);
                    auction.item.extra_attributes = Some(extra_attributes);
                    auction.item.missing_attributes = Some(missing_attributes);
                    new_payload.push(auction);
                }
                return Ok(new_payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.status_code {
                    400 => LogLevel::Warning,
                    _ => LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    &self.get_component("Search"),
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
    pub fn calculate_shared_attribute_percentages(
        &self,
        attributes: Vec<RivenAttribute>,
        item2: &AuctionItem,
    ) -> (f64, Vec<RivenAttribute>, Vec<RivenAttribute>) {
        let mut shared_count = 0;

        let right_attributes = item2.attributes.as_ref().unwrap();

        let extra_attributes: Vec<_> = right_attributes
            .iter()
            .filter(|attr| {
                !attributes
                    .iter()
                    .any(|attr2| attr2.url_name == attr.url_name && attr2.positive == attr.positive)
            })
            .cloned()
            .collect();

        let missing_attributes: Vec<_> = attributes
            .iter()
            .filter(|attr| {
                !right_attributes
                    .iter()
                    .any(|attr2| attr2.url_name == attr.url_name && attr2.positive == attr.positive)
            })
            .cloned()
            .collect();

        let mut unique_attributes = std::collections::HashSet::new();
        attributes.iter().for_each(|attr| {
            unique_attributes.insert(format!("{}_{}", attr.url_name, attr.positive));
        });

        let total_unique_attributes_count = unique_attributes.len();

        for attr2 in right_attributes {
            let key = format!("{}_{}", attr2.url_name, attr2.positive);
            if unique_attributes.contains(&key) {
                shared_count += 1;
                unique_attributes.remove(&key); // Ensure the attribute is only counted once
            }
        }

        (
            shared_count as f64 / total_unique_attributes_count as f64 * 100.0,
            extra_attributes,
            missing_attributes,
        )
    }
    pub async fn delete(&mut self, auction_id: &str) -> Result<Option<String>, AppError> {
        let url = format!("auctions/entry/{}/close", auction_id);

        self.client.auth().is_logged_in()?;
        match self.client.put(&url, Some("auction_id"), None).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.subtract_auction_count(1)?;
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Delete"),
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
                        LogLevel::Warning
                    }
                    _ => LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    &self.get_component("Delete"),
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
}

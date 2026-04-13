use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::log_parser::{apply_item_info, TradeItem};

use super::{Login, PlayerTrade, Purchase, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Account {
    pub oid: String,
    pub email: String,
    pub display_name: String,
    pub activated: bool,
    pub subscribed_to_emails: bool,
    pub signup_language: String,
    pub signup_country_code: String,
    pub country_code: String,
    pub signup_page: String,
    pub ips: Vec<String>,
    pub account_creation_date: Option<DateTime<Utc>>,
    pub last_login_date: Option<DateTime<Utc>>,
    pub language: String,
    pub trades: Vec<PlayerTrade>,
    pub logins: Vec<Login>,
    pub purchases: Vec<Purchase>,
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn add_trade(&mut self, mut trade: PlayerTrade) {
        trade.calculate();
        trade.calculate_items();
        apply_item_info(&mut trade);

        let items = trade
            .offered_items
            .iter_mut()
            .chain(trade.received_items.iter_mut())
            .collect::<Vec<&mut TradeItem>>();
        for item in items {
            let name = item
                .properties
                .get_property_value("item_name", String::new());
            if name.is_empty() {
                item.properties.set_property_value("item_name", &item.raw);
            }
        }
        self.trades.push(trade);
    }
}

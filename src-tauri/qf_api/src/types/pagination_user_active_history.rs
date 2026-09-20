use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::enums::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActiveHistoryQueryDto {
    from_date: String,
    to_date: String,
    group_by: TimeGroup,
}
impl UserActiveHistoryQueryDto {
    pub fn new(from_date: String, to_date: String, group_by: TimeGroup) -> Self {
        Self {
            from_date,
            to_date,
            group_by,
        }
    }
    pub fn get_query(&self) -> String {
        let mut query: Vec<String> = Vec::new();
        query.push(format!("from_date={}", self.from_date));
        query.push(format!("to_date={}", self.to_date));
        query.push(format!("group_by={}", self.group_by.to_string()));

        query.join("&")
    }
    pub fn set_from_date(&mut self, from_date: String) {
        self.from_date = from_date;
    }
    pub fn set_to_date(&mut self, to_date: String) {
        self.to_date = to_date;
    }
    pub fn set_group_by(&mut self, group_by: TimeGroup) {
        self.group_by = group_by;
    }
}

impl Display for UserActiveHistoryQueryDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "From Date: {}, To Date: {}, Group By: {}",
            self.from_date, self.to_date, self.group_by.to_string()
        )
    }
}
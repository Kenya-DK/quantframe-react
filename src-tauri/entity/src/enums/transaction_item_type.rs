use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(15))")]
#[derive(Eq)]
pub enum TransactionItemType {
    #[sea_orm(string_value = "item")]
    Item,
    #[sea_orm(string_value = "riven")]
    Riven,
}

impl TransactionItemType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "item" => Self::Item,
            "riven" => Self::Riven,
            _ => panic!("Invalid transaction type"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Item => "item".to_string(),
            Self::Riven => "riven".to_string(),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::Item => "item",
            Self::Riven => "riven",
        }
    }
}
impl Serialize for TransactionItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TransactionItemType::Item => "item",
            TransactionItemType::Riven => "riven",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TransactionItemType {
    fn deserialize<D>(deserializer: D) -> Result<TransactionItemType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "item" => TransactionItemType::Item,
            "riven" => TransactionItemType::Riven,
            _ => panic!("Invalid transaction type"),
        })
    }
}

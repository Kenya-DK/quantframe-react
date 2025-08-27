use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(15))")]
#[derive(Eq)]
pub enum TransactionType {
    #[sea_orm(string_value = "sale")]
    Sale,
    #[sea_orm(string_value = "purchase")]
    Purchase,
}

impl TransactionType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "sale" => Self::Sale,
            "purchase" => Self::Purchase,
            _ => panic!("Invalid transaction type"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            TransactionType::Sale => "sale".to_string(),
            TransactionType::Purchase => "purchase".to_string(),
        }
    }
}
impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TransactionType::Purchase => "purchase",
            TransactionType::Sale => "sale",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> Result<TransactionType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "purchase" => TransactionType::Purchase,
            "sale" => TransactionType::Sale,
            _ => panic!("Invalid transaction type"),
        })
    }
}

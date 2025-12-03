use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
#[derive(Eq)]
pub enum RivenAttributeGrade {
    #[sea_orm(string_value = "Decisive")]
    Decisive,
    #[sea_orm(string_value = "good")]
    Good,
    #[sea_orm(string_value = "not_helping")]
    NotHelping,
    #[sea_orm(string_value = "bad")]
    Bad,
    #[sea_orm(string_value = "unknown")]
    Unknown,
}
impl RivenAttributeGrade {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            RivenAttributeGrade::Decisive => "decisive",
            RivenAttributeGrade::Good => "good",
            RivenAttributeGrade::NotHelping => "not_helping",
            RivenAttributeGrade::Bad => "bad",
            RivenAttributeGrade::Unknown => "unknown",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            RivenAttributeGrade::Decisive => RivenAttributeGrade::Decisive.as_str().to_string(),
            RivenAttributeGrade::Good => RivenAttributeGrade::Good.as_str().to_string(),
            RivenAttributeGrade::NotHelping => RivenAttributeGrade::NotHelping.as_str().to_string(),
            RivenAttributeGrade::Bad => RivenAttributeGrade::Bad.as_str().to_string(),
            RivenAttributeGrade::Unknown => RivenAttributeGrade::Unknown.as_str().to_string(),
        }
    }
    pub fn from_string(s: &str) -> RivenAttributeGrade {
        match s {
            "decisive" => RivenAttributeGrade::Decisive,
            "good" => RivenAttributeGrade::Good,
            "not_helping" => RivenAttributeGrade::NotHelping,
            "bad" => RivenAttributeGrade::Bad,
            "unknown" => RivenAttributeGrade::Unknown,
            _ => RivenAttributeGrade::Unknown,
        }
    }
}
impl Default for RivenAttributeGrade {
    fn default() -> Self {
        RivenAttributeGrade::Unknown
    }
}
impl Serialize for RivenAttributeGrade {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            RivenAttributeGrade::Decisive => RivenAttributeGrade::Decisive.as_str(),
            RivenAttributeGrade::Good => RivenAttributeGrade::Good.as_str(),
            RivenAttributeGrade::NotHelping => RivenAttributeGrade::NotHelping.as_str(),
            RivenAttributeGrade::Bad => RivenAttributeGrade::Bad.as_str(),
            RivenAttributeGrade::Unknown => RivenAttributeGrade::Unknown.as_str(),
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for RivenAttributeGrade {
    fn deserialize<D>(deserializer: D) -> Result<RivenAttributeGrade, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "decisive" => RivenAttributeGrade::Decisive,
            "good" => RivenAttributeGrade::Good,
            "not_helping" => RivenAttributeGrade::NotHelping,
            "bad" => RivenAttributeGrade::Bad,
            "unknown" => RivenAttributeGrade::Unknown,
            _ => RivenAttributeGrade::Unknown,
        })
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
#[derive(Eq)]
pub enum RivenGrade {
    #[sea_orm(string_value = "perfect")]
    Perfect,
    #[sea_orm(string_value = "good")]
    Good,
    #[sea_orm(string_value = "has_potential")]
    HasPotential,
    #[sea_orm(string_value = "bad")]
    Bad,
    #[sea_orm(string_value = "unknown")]
    Unknown,
}
impl RivenGrade {
    // Create method to convert `OrderType` to a `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            RivenGrade::Perfect => "perfect",
            RivenGrade::Good => "good",
            RivenGrade::HasPotential => "has_potential",
            RivenGrade::Bad => "bad",
            RivenGrade::Unknown => "unknown",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            RivenGrade::Perfect => RivenGrade::Perfect.as_str().to_string(),
            RivenGrade::Good => RivenGrade::Good.as_str().to_string(),
            RivenGrade::HasPotential => RivenGrade::HasPotential.as_str().to_string(),
            RivenGrade::Bad => RivenGrade::Bad.as_str().to_string(),
            RivenGrade::Unknown => RivenGrade::Unknown.as_str().to_string(),
        }
    }
    pub fn from_string(s: &str) -> RivenGrade {
        match s {
            "perfect" => RivenGrade::Perfect,
            "good" => RivenGrade::Good,
            "has_potential" => RivenGrade::HasPotential,
            "bad" => RivenGrade::Bad,
            "unknown" => RivenGrade::Unknown,
            _ => RivenGrade::Unknown,
        }
    }
}
impl Serialize for RivenGrade {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            RivenGrade::Perfect => RivenGrade::Perfect.as_str(),
            RivenGrade::Good => RivenGrade::Good.as_str(),
            RivenGrade::HasPotential => RivenGrade::HasPotential.as_str(),
            RivenGrade::Bad => RivenGrade::Bad.as_str(),
            RivenGrade::Unknown => RivenGrade::Unknown.as_str(),
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for RivenGrade {
    fn deserialize<D>(deserializer: D) -> Result<RivenGrade, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "perfect" => RivenGrade::Perfect,
            "good" => RivenGrade::Good,
            "has_potential" => RivenGrade::HasPotential,
            "bad" => RivenGrade::Bad,
            "unknown" => RivenGrade::Unknown,
            _ => RivenGrade::Unknown,
        })
    }
}

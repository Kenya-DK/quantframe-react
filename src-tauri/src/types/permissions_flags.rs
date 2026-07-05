use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum PermissionsFlags {
    ALL,
    ExportData,
    None,
    RivenPricesSearch,
    WFMUserActiveHistory,
    FindInterestingRivens,
    SyndicatePricesSearch,
}
impl PermissionsFlags {
    pub fn as_str(&self) -> &str {
        match *self {
            PermissionsFlags::ALL => "all",
            PermissionsFlags::ExportData => "export_data",
            PermissionsFlags::RivenPricesSearch => "riven_prices_search",
            PermissionsFlags::WFMUserActiveHistory => "wfm_user_active_history",
            PermissionsFlags::FindInterestingRivens => "find_interesting_rivens",
            PermissionsFlags::SyndicatePricesSearch => "syndicate_prices_search",
            PermissionsFlags::None => "none",
        }
    }
    pub fn from_str(s: &str) -> PermissionsFlags {
        match s {
            "all" => PermissionsFlags::ALL,
            "export_data" => PermissionsFlags::ExportData,
            "riven_prices_search" => PermissionsFlags::RivenPricesSearch,
            "wfm_user_active_history" => PermissionsFlags::WFMUserActiveHistory,
            "find_interesting_rivens" => PermissionsFlags::FindInterestingRivens,
            "syndicate_prices_search" => PermissionsFlags::SyndicatePricesSearch,
            _ => PermissionsFlags::None,
        }
    }
}
impl Display for PermissionsFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

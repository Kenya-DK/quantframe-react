

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LastAdded {
    #[serde(rename = "oid", default)]
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiscItemItemId {
    #[serde(rename = "$oid", default)]
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiscItem {
    #[serde(rename = "ItemId", default)]
    pub id: MiscItemItemId,

    #[serde(rename = "ItemCount", default)]
    pub quantity: i64,

    #[serde(rename = "ItemType", default)]
    pub unique_name: String,

    #[serde(rename = "UpgradeFingerprint", default)]
    pub upgrade_fingerprint: String,

    #[serde(rename = "LastAdded", default)]
    pub last_added: LastAdded,

    #[serde(rename = "XP", default)]
    pub xp: i64,

    #[serde(rename = "ItemCount", default)]
    pub quantity: i64,

}

impl MiscItem {
    pub fn is_empty(&self) -> bool {
        self.quantity == 0
    }
}
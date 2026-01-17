use serde::Deserialize;

use crate::enums::*;
#[derive(Deserialize, Clone, Debug)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "moderator")]
    Moderator,
    #[serde(rename = "admin")]
    Admin,
}
#[derive(Deserialize, Clone, Debug)]
pub enum Tier {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "bronze")]
    Bronze,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gold")]
    Gold,
    #[serde(rename = "diamond")]
    Diamond,
    #[serde(rename = "platinum")]
    Platinum,
}

impl Tier {
    pub fn is_none(&self) -> bool {
        matches!(self, Tier::None)
    }

    pub fn is_premium(&self) -> bool {
        matches!(
            self,
            Tier::Bronze | Tier::Silver | Tier::Gold | Tier::Diamond | Tier::Platinum
        )
    }
}
#[derive(Deserialize, Clone, Debug)]
pub struct LinkedAccounts {
    #[serde(rename = "steamProfile")]
    pub steam_profile: bool,
    #[serde(rename = "patreonProfile")]
    pub patreon_profile: bool,
    #[serde(rename = "xboxProfile")]
    pub xbox_profile: bool,
    #[serde(rename = "discordProfile")]
    pub discord_profile: bool,
    #[serde(rename = "githubProfile")]
    pub github_profile: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserPrivate {
    /// Unique identifier of the user.
    pub id: String,
    /// Role assigned to the user (e.g., moderator, user).
    pub role: Role,
    /// In-game name.
    #[serde(rename = "ingameName")]
    pub ingame_name: String,
    /// Optional avatar image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Optional background image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Optional about text in HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    /// Optional about text in raw markdown.
    #[serde(rename = "aboutRaw", skip_serializing_if = "Option::is_none")]
    pub about_raw: Option<String>,
    /// Reputation score.
    pub reputation: i16,
    /// In-game mastery level.
    #[serde(rename = "masteryRank")]
    pub mastery_rank: i8,
    /// In-game currency balance.
    pub credits: i32,

    /// Gaming platform.
    pub platform: String,
    /// Crossplay enabled / disabled.
    pub crossplay: bool,
    /// Preferred communication language.
    pub locale: String,
    /// Preferred color scheme for UI.
    pub theme: String,

    /// List of achievements the user chose to showcase.
    // #[serde(
    //     rename = "achievementShowcase",
    //     skip_serializing_if = "Option::is_none"
    // )]
    // pub achievement_showcase: Option<Vec<Achievement>>,

    /// Verification status.
    pub verification: bool,
    /// Unique check code for the user.
    #[serde(rename = "checkCode")]
    pub check_code: String,

    /// Subscription tier.
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    /// Subscription status.
    pub subscription: bool,

    /// Warning status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warned: Option<bool>,
    /// Warning message.
    #[serde(rename = "warnMessage", skip_serializing_if = "Option::is_none")]
    pub warn_message: Option<String>,
    /// Ban status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned: Option<bool>,
    /// End date of the ban.
    #[serde(rename = "banUntil", skip_serializing_if = "Option::is_none")]
    pub ban_until: Option<String>,
    /// Reason for the ban.
    #[serde(rename = "banMessage", skip_serializing_if = "Option::is_none")]
    pub ban_message: Option<String>,

    /// How many reviews the user can still write today (reset at midnight UTC).
    #[serde(rename = "reviewsLeft")]
    pub reviews_left: i16,
    /// Count of unread messages.
    #[serde(rename = "unreadMessages", default)]
    pub unread_messages: i16,
    /// List of ignored users.
    #[serde(rename = "ignoreList", default)]
    pub ignore_list: Vec<String>,

    /// Flag for pending deletion of the account.
    #[serde(rename = "deleteInProgress", skip_serializing_if = "Option::is_none")]
    pub delete_in_progress: Option<bool>,
    /// Scheduled deletion date.
    #[serde(rename = "deleteAt", skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<String>,

    /// If the user has an email address.
    #[serde(rename = "hasEmail")]
    pub has_email: bool,

    /// Timestamp of the last online presence.
    #[serde(rename = "lastSeen")]
    pub last_seen: String,
    /// Account creation date.
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

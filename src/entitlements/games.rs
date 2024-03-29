use serde::{Deserialize, Serialize};

/// Games
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Games {
    /// A Boolean value that indicates whether users of the app may see and compare
    /// achievements on a leaderboard, invite friends, and start multiplayer games.
    ///
    /// To add this entitlement to your app, enable the Game Center capability in Xcode.
    ///
    /// ## Availability
    /// * macOS 10.8+
    ///
    /// ## Framework
    /// * GameKit
    #[serde(
        rename = "com.apple.developer.game-center",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub game_center: Option<bool>,
}

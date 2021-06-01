use serde::{Deserialize, Serialize};

/// Car Play
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct CarPlay {
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "com.apple.developer.carplay-audio"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_audio: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "com.apple.developer.carplay-charging"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_charging: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "com.apple.developer.carplay-communication"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_communication: Option<bool>,
    /// ## Availability
    /// * iOS 12.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "com.apple.developer.carplay-maps"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_maps: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "com.apple.developer.carplay-parking"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_parking: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "com.apple.developer.carplay-quick-ordering"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_quick_ordering: Option<bool>,
    /// ## Availability
    /// * iOS 12.0–14.0
    ///
    /// ## Framework
    /// * CarPlay
    #[deprecated(since = "iOS 12.0–14.0")]
    #[serde(
        rename(serialize = "com.apple.developer.carplay-messaging"),
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_messaging: Option<bool>,
    /// ## Availability
    /// * iOS 12.0–14.0
    ///
    /// ## Framework
    /// * CarPlay
    #[deprecated(since = "iOS 12.0–14.0")]
    #[serde(
        rename(serialize = "com.apple.developer.playable-content"),
        skip_serializing_if = "Option::is_none"
    )]
    pub playable_content: Option<bool>,
}

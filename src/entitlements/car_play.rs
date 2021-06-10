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
        rename = "com.apple.developer.carplay-audio",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_audio: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "com.apple.developer.carplay-charging",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_charging: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "com.apple.developer.carplay-communication",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_communication: Option<bool>,
    /// ## Availability
    /// * iOS 12.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "com.apple.developer.carplay-maps",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_maps: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "com.apple.developer.carplay-parking",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub carplay_parking: Option<bool>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "com.apple.developer.carplay-quick-ordering",
        serialize_with = "crate::serialize_option",
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
        rename = "com.apple.developer.carplay-messaging",
        serialize_with = "crate::serialize_option",
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
        rename = "com.apple.developer.playable-content",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub playable_content: Option<bool>,
}

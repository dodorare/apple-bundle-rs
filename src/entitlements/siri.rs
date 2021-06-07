use serde::{Deserialize, Serialize};

/// Siri
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Siri {
    /// A Boolean value that indicates whether the app handles Siri requests.
    ///
    /// The App Store requires the presence of this entitlement for iOS or watchOS apps
    /// containing Intents app extensions that handle any Siri requests other than
    /// shortcut requests. To add the entitlement to your app, enable the Siri
    /// capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * watchOS 3.2+
    ///
    /// ## Framework
    /// * Intents
    #[serde(
        rename = "com.apple.developer.siri",
        skip_serializing_if = "Option::is_none"
    )]
    pub siri: Option<bool>,
}

use serde::{Deserialize, Serialize};

/// Home Automation
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct HomeAutomation {
    /// A Boolean value that indicates whether users of the app may manage
    /// HomeKit-compatible accessories.
    ///
    /// This key Indicates whether the users of an app may manage HomeKit-compatible
    /// accessories,such as switches, lights, fans, locks, and thermostats.
    ///
    /// To add this entitlement to your app, enable the HomeKit capability in Xcode. For
    /// more information, see Enabling HomeKit in Your App.
    ///
    /// ## Availability
    /// * iOS 8.0+
    /// * tvOS 10.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * HomeKit
    #[serde(
        rename = "com.apple.developer.homekit",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub homekit: Option<bool>,
}

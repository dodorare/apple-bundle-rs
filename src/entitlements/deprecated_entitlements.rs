use serde::{Deserialize, Serialize};

/// Deprecated Entitlements
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct DeprecatedEntitlements {
    /// A Boolean value that indicates whether the app may provide directions beyond what Maps supports, such as subway routes, hiking trails, and bike paths.
    ///
    /// To add this entitlement to your app, enable the Maps capability in Xcode.
    ///
    /// ## Availability
    /// * macOS 10.9–10.11
    ///
    /// ## Framework
    /// * MapKit
    #[deprecated(
        since = "macOS 10.9–10.11",
        note = "Using Maps no longer requires an entitlement."
    )]
    #[serde(
        rename = "com.apple.developer.maps",
        skip_serializing_if = "Option::is_none"
    )]
    pub maps: Option<bool>,
    /// A Boolean value that indicates whether the app may exchange audio with other Inter-App Audio-enabled apps.
    ///
    /// Enabling Inter-App Audio allows your app to send and receive audio from other Inter-App Audio enabled apps and provides access to Audio Unit extensions.
    ///
    /// To add this entitlement to your app, enable the Inter-App Audio capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 2.2–13.0
    ///
    /// ## Framework
    /// * AVFoundation
    #[deprecated(
        since = "iOS 2.2–13.0",
        note = "Inter-App Audio is deprecated in iOS 13 and is unavailable when running iPad apps in macOS."
    )]
    #[serde(rename = "inter-app-audio", skip_serializing_if = "Option::is_none")]
    pub inter_app_audio: Option<bool>,
}

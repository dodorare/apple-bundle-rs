use serde::{Deserialize, Serialize};

/// Wallet
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Wallet {
    /// A list of identifiers that specify pass types that your app can access in Wallet.
    ///
    /// The value for this key is an array of pass type identifiers.
    ///
    /// To add this entitlement to your app, enable the Wallet capability in Xcode.
    /// If your provisioning profile is associated with multiple pass type identifiers,
    /// specify which of the identifiers your app can interact with.
    /// Use $(TeamIdentifierPrefix)* to access all of the passes for your team.
    ///
    /// For more information, see Configure Wallet (iOS, watchOS).
    ///
    /// ### Note
    /// The Wallet capability isn’t available to app clips.
    /// For information on functionality that’s available to app clips, see Choosing the
    /// Right Functionality for Your App Clip.
    ///
    /// ## Availability
    /// * iOS 6.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * PassKit (Apple Pay and Wallet)
    #[serde(
        rename = "com.apple.developer.pass-type-identifiers",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pass_type_ids: Option<Vec<String>>,
    /// A list of merchant IDs your app uses for Apple Pay support.
    ///
    /// The value for this key is an array of strings containing the merchant
    /// IDs—typically in reverse domain name notation, starting with the string
    /// 'merchant'.
    ///
    /// To add this entitlement, enable the Apple Pay capability in Xcode and select the
    /// merchant IDs you want to use in your app. Alternatively, see Setting Up Apple
    /// Pay Requirements for how to create merchant IDs in your developer account.
    ///
    /// ## Availability
    /// * iOS 6.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * PassKit (Apple Pay and Wallet)
    #[serde(
        rename = "com.apple.developer.in-app-payments",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub merchant_ids: Option<Vec<String>>,
}

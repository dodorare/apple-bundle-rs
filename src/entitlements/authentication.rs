use serde::{Deserialize, Serialize};

/// Authentication
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Authentication {
    /// A Boolean value that indicates whether the app may, with user permission, provide
    /// user names and passwords for AutoFill in Safari and other apps.
    ///
    /// To add this entitlement to a target, enable the AutoFill Credential Provider
    /// capability in Xcode. Do this for both your Password AutoFill extension and its
    /// host app.
    ///
    /// ## Availability
    /// * iOS 12.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(
        rename = "com.apple.developer.authentication-services.autofill-credential-provider",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_fill_credential_provider: Option<bool>,
    /// An entitlement that lets your app use Sign in with Apple.
    ///
    /// To add this entitlement to your app with the correct associated value, enable the
    /// Sign in with Apple capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 13.0+
    /// * macOS 10.15+
    /// * tvOS 13.0+
    /// * watchOS 6.0+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(
        rename = "com.apple.developer.applesignin",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_vec_enum_option"
    )]
    pub sign_in_with_apple: Option<Vec<SignInWithAppleEntitlement>>,
}

/// Sign in with Apple Entitlement
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum SignInWithAppleEntitlement {
    /// The value used for normal operation.
    #[serde(rename = "Default")]
    Default,
}

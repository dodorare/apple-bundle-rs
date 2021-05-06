use super::{serialize_enum_option, serialize_vec_enum_option};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Authentication {
    /// A Boolean value that indicates whether the app may, with user permission, provide user names and passwords for AutoFill in Safari and other apps.
    ///
    /// To add this entitlement to a target, enable the AutoFill Credential Provider capability in Xcode. Do this for both your Password AutoFill extension and its host app.
    ///
    /// ## Availability
    /// * iOS 12.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(
        rename(
            serialize = "com.apple.developer.authentication-services.autofill-credential-provider"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_fill_credential_provider: Option<bool>,
}

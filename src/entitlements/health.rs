use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Health {
    /// A Boolean value that indicates whether the app may request user authorization to access health and activity data that appears in the Health app.
    ///
    /// To add this entitlement to your app, enable the HealthKit capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 8.0+
    ///
    /// ## Framework
    /// * HealthKit
    #[serde(
        rename(serialize = "com.apple.developer.healthkit"),
        skip_serializing_if = "Option::is_none"
    )]
    pub healthkit: Option<bool>,
    /// Health data types that require additional permission.
    ///
    /// The HealthKit Entitlement provides access to most HealthKit data types. However, because of their highly sensitive nature, some data types require additional entitlements.
    /// The HealthKit Capabilities Entitlement provides access to these data types.
    ///
    /// To add this entitlement to your app, first enable the HealthKit capability in Xcode, and then check any values that you want to add to the HealthKit Capabilities Entitlement.
    /// 
    /// Only add values for data types that your app needs to access.
    /// App Review may reject apps that don’t use the data appropriately.
    /// For more information, see the Health and Health Research section of the App Store Review Guidelines.
    /// 
    /// ## Availability
    /// * iOS 8.0+
    ///
    /// ## Framework
    /// * HealthKit
    #[serde(
        rename(serialize = "com.apple.developer.healthkit.access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub healthkit_access: Option<Vec<String>>,
}
545
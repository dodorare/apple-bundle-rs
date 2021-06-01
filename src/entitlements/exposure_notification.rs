use serde::{Deserialize, Serialize};

/// Exposure Notification
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct ExposureNotification {
    /// A Boolean value that indicates whether the app may use exposure notification.
    ///
    /// Before you can develop an app that uses exposure notifications, you need this entitlement.
    /// For more information on this entitlement, see Exposure Notification APIs Addendum.
    /// To get permission to use this entitlement, see Exposure Notification Entitlement Request.
    ///
    /// ## Availability
    /// * iOS 13.5+
    ///
    /// ## Framework
    /// * Exposure Notification
    #[serde(
        rename(serialize = "com.apple.developer.exposure-notification"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exposure_notification: Option<bool>,
}

use crate::serialize_vec_enum_option;
use serde::{Deserialize, Serialize};

/// TV
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Tv {
    /// The entitlement for distinguishing between multiple user accounts on Apple TV.
    ///
    /// To configure the entitlement, add the User Management capability on your app’s
    /// target in Xcode and select the checkbox for each privilege your app requires.
    /// For more details about adding a capability, see Adding Capabilities to Your App.
    ///
    /// For guidance on choosing a data management strategy for your app, see
    /// Personalizing Your App for Each User on Apple TV.
    ///
    /// ### Note
    /// You can enable runs-as-current-user if your app’s minimum version is earlier than
    /// tvOS 14, but the app will behave as if the privilege isn’t set when running on the
    /// earlier version.
    ///
    /// ## Availability
    /// * tvOS 13.0+
    ///
    /// ## Framework
    /// * TV Services
    #[serde(
        rename = "com.apple.developer.user-management",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub user_management: Option<Vec<UserManagement>>,
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.14+
    /// * tvOS 10.0+
    ///
    /// ## Framework
    /// * Video Subscriber Account
    #[serde(
        rename = "com.apple.developer.video-subscriber-single-sign-on",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_subscriber_single_sign_on: Option<bool>,
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.14+
    /// * tvOS 10.0+
    ///
    /// ## Framework
    /// * Video Subscriber Account
    #[serde(
        rename = "com.apple.smoot.subscriptionservice",
        skip_serializing_if = "Option::is_none"
    )]
    pub smoot_subscriptionservice: Option<bool>,
}

/// User Management
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserManagement {
    /// The value that grants access to TVUserManager, so you can map your own profiles to
    /// users in the system.
    #[serde(rename = "get-current-user")]
    GetCurrentUser,
    /// The value that grants access to a separate set of data for your app for each user
    /// from GameCenter, iCloud, and local storage. Available in tvOS 14 or later.
    #[serde(rename = "runs-as-current-user")]
    RunsAsCurrentUser,
}

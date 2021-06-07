use crate::serialize_enum_option;
use serde::{Deserialize, Serialize};

/// Push Notifications
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct PushNotifications {
    /// The environment for push notifications.
    ///
    /// This key specifies whether to use the development or production Apple Push
    /// Notification service (APNs) environment when registering for push notifications.
    ///
    /// Xcode sets the value of the entitlement based on your app's current provisioning
    /// profile. For example, if you're using a development provisioning profile,
    /// Xcode sets the value to development. Production provisioning profile and
    /// Prerelease Versions and Beta Testers use production. These default settings
    /// can be modified. The development environment is also referred to as the
    /// sandbox environment.
    ///
    /// Use this entitlement for both the UserNotifications and PushKit frameworks.
    ///
    /// To add this entitlement to your app, enable the Push Notifications capability in
    /// Xcode.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * tvOS 10.0+
    /// * watchOS 3.0+
    ///
    /// ## Framework
    /// * User Notifications
    #[serde(
        rename = "aps-environment",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub aps_environment: Option<APSEnvironment>,
    /// The environment for push notifications in macOS apps.
    ///
    /// This key specifies whether to use the development or production Apple Push
    /// Notification service (APNs) environment when registering for push notifications
    /// with registerForRemoteNotifications().
    ///
    /// Xcode sets the value of the entitlement based on your app's current provisioning
    /// profile. For example, if you're using a development provisioning profile,
    /// Xcode sets the value to development.
    ///
    /// To add this entitlement to your app, enable the Push Notifications capability in
    /// Xcode.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * User Notifications
    #[serde(
        rename = "com.apple.developer.aps-environment",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub aps_environment_macos: Option<APSEnvironment>,
    /// Enable receiving notifications without displaying the notification to the user.
    ///
    /// This entitlement allows a notification service extension to receive remote
    /// notifications without displaying the notification to the user. To apply for
    /// this entitlement, see Request Notification Service Entitlement.
    ///
    /// After you receive permission to use the entitlement, add
    /// com.apple.developer.usernotifications.filtering to the entitlements file in the
    /// Notification Service Extension target. This allows you to silence push
    /// notifications after your extension receives them.
    ///
    /// ### Silence Push Notifications
    /// To suppress a notification’s alert, create an empty UNNotificationContent object
    /// in your extension’s didReceive(_:withContentHandler:) method, and pass it to the
    /// content handler. Don’t specify a title, subtitle, body, attachments, or sound
    /// for the content.
    ///
    /// ```swift
    /// override func didReceive(_ request: UNNotificationRequest, withContentHandler
    /// contentHandler: @escaping (UNNotificationContent) -> Void) {
    ///
    ///     // Determine whether you should suppress the notification.
    ///     let suppress = myShouldSuppressNotification(request: request)
    ///     
    ///     if suppress {
    ///         // Don't deliver the notification to the user.
    ///         contentHandler(UNNotificationContent())
    ///         
    ///     } else {
    ///         // Deliver the notification.
    ///         guard let updatedContent = request.content.mutableCopy() as?
    /// UNMutableNotificationContent else {             // This error should never
    /// occur.             fatalError("Unable to create a mutable copy of the
    /// content")         }
    ///         
    ///         // Update the notification's content, such as decrypting the body, here.
    ///         contentHandler(updatedContent)
    ///     }
    /// }
    /// ```
    ///
    /// ### Note
    /// To silence a remote notification, you must set the apns-push-type header field to
    /// alert when you send the notification to the APNS server. Otherwise, the system
    /// always displays the notification banner to the user.
    ///
    /// ## Availability
    /// * iOS 13.3+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * User Notifications
    #[serde(
        rename = "com.apple.developer.usernotifications.filtering",
        skip_serializing_if = "Option::is_none"
    )]
    pub usernotifications_filtering: Option<bool>,
}

/// APS Environment
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APSEnvironment {
    /// The APNs development environment.
    #[serde(rename = "development")]
    Development,
    /// The APNs production environment.
    #[serde(rename = "production")]
    Production,
}

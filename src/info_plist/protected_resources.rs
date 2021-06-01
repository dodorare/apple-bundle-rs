//! # Protected Resources
//!
//! Control an app’s access to protected system services and user data.
//!
//! ### Overview
//! Before your app can access certain protected resources, like the Bluetooth interface, location information, or the user’s photos, the system asks the user for permission on behalf of your app.
//! To signal that your app needs the access, you add a UsageDescription key to your app’s Information Property List.
//! You set the value associated with the key to a string that explains why your app needs access.
//! The system displays this string when prompting the user, as described in Requesting Access to Protected Resources.

use crate::serialize_vec_enum_option;
use serde::{Deserialize, Serialize};

/// Bluetooth
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Bluetooth {
    /// A message that tells the user why the app needs access to Bluetooth.
    ///
    /// This key is required if your app uses the device’s Bluetooth interface.
    ///
    /// ### Important
    /// If your app has a deployment target earlier than iOS 13, add the NSBluetoothPeripheralUsageDescription key to your app’s Information Property List file in addition to this key.
    ///
    /// ## Availability
    /// * iOS 13.0+
    /// * tvOS 13.0+
    /// * watchOS 6.0+
    ///
    /// ## Framework
    /// * Core Bluetooth
    #[serde(
        rename(serialize = "NSBluetoothAlwaysUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bluetooth_always_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting the ability to connect to Bluetooth peripherals.
    ///
    /// For apps with a deployment target of iOS 13 and later, use NSBluetoothAlwaysUsageDescription instead.
    ///
    /// For deployment targets earlier than iOS 13, add both NSBluetoothAlwaysUsageDescription and NSBluetoothPeripheralUsageDescription to your app’s Information Property List file.
    /// Devices running earlier versions of iOS rely on NSBluetoothPeripheralUsageDescription, while devices running later versions rely on NSBluetoothAlwaysUsageDescription.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access Bluetooth peripherals and has a deployment target earlier than iOS 13.
    ///
    /// ## Availability
    /// * iOS 6.0–13.0
    ///
    /// ## Framework
    /// * Core Bluetooth
    #[deprecated(since = "iOS 6.0–13.0")]
    #[serde(
        rename(serialize = "NSBluetoothPeripheralUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bluetooth_peripheral_usage_description: Option<String>,
}

/// Calendar and Reminders
///
/// [Accessing the Event Store](https://developer.apple.com/documentation/eventkit/accessing_the_event_store)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct CalendarAndReminders {
    /// A message that tells the user why the app is requesting access to the user’s calendar data.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s calendar data.
    ///
    /// ## Availability
    /// * iOS 6.0+
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * EventKit
    #[serde(
        rename(serialize = "NSCalendarsUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub calendars_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting access to the user’s reminders.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s reminders.
    ///
    /// ## Availability
    /// * iOS 6.0+
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * EventKit
    #[serde(
        rename(serialize = "NSRemindersUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub reminders_usage_description: Option<String>,
}

/// Camera and Microphone
///
/// ## Articles
/// * [Requesting Authorization for Media Capture on iOS](https://developer.apple.com/documentation/avfoundation/cameras_and_media_capture/requesting_authorization_for_media_capture_on_ios)
/// * [Requesting Authorization for Media Capture on macOS](https://developer.apple.com/documentation/avfoundation/cameras_and_media_capture/requesting_authorization_for_media_capture_on_macos)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct CameraAndMicrophone {
    /// A message that tells the user why the app is requesting access to the device’s camera.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the device’s camera.
    ///
    /// ## Availability
    /// * iOS 7.0+
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * AVFoundation
    #[serde(
        rename(serialize = "NSCameraUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub camera_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting access to the device’s microphone.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the device’s microphone.
    ///
    /// ## Availability
    /// * iOS 7.0+
    /// * macOS 10.14+
    /// * watchOS 4.0+
    ///
    /// ## Framework
    /// * AVFoundation
    #[serde(
        rename(serialize = "NSMicrophoneUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub microphone_usage_description: Option<String>,
}

/// Contacts
///
/// [Requesting Authorization to Access Contacts](https://developer.apple.com/documentation/contacts/requesting_authorization_to_access_contacts)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Contacts {
    /// A message that tells the user why the app is requesting access to the user’s contacts.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s contacts.
    ///
    /// ## Availability
    /// * iOS 6.0+
    /// * macOS 10.8+
    ///
    /// ## Framework
    /// * Contacts
    #[serde(
        rename(serialize = "NSContactsUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub contacts_usage_description: Option<String>,
}

/// FaceID
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FaceID {
    /// A message that tells the user why the app is requesting the ability to authenticate with Face ID.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access Face ID.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Local Authentication
    #[serde(
        rename(serialize = "NSFaceIDUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub face_id_usage_description: Option<String>,
}

/// Files and Folders
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct FilesAndFolders {
    /// A message that tells the user why the app needs access to the user’s Desktop folder.
    ///
    /// The user implicitly grants your app access to a file in the Desktop folder when selecting the file in an Open or Save panel, dragging it onto your app, or opening it in Finder.
    /// Your app can access that file right away and any time in the future.
    /// In addition, if your app creates a new file in the Desktop folder, the app can access that file without user consent.
    ///
    /// The first time your app tries to access a file in the user’s Desktop folder without implied user consent, the system prompts the user for permission to access the folder’s contents.
    /// Add the NSDesktopFolderUsageDescription key to your app’s Information Property List file to provide a message that explains why your app needs access.
    /// The usage description is optional, but highly recommended.
    ///
    /// App Sandbox enforces stricter limits on Desktop folder access, so that policy may supersede this one if your app enables sandboxing. See App Sandbox for more information.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset SystemPolicyDesktopFolder <bundleID>
    /// ```
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSDesktopFolderUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub desktop_folder_usage_description: Option<String>,
    /// A message that tells the user why the app needs access to the user’s Documents folder.
    ///
    /// The user implicitly grants your app access to a file in the Documents folder when selecting the file in an Open or Save panel, dragging it onto your app, or opening it in Finder.
    /// Your app can access that file right away and any time in the future.
    /// In addition, if your app creates a new file in the Documents folder, the app can access that file without user consent.
    ///
    /// The first time your app tries to access a file in the user’s Documents folder without implied user consent, the system prompts the user for permission to access the folder’s contents.
    /// Add the NSDocumentsFolderUsageDescription key to your app’s Information Property List file to provide a message that explains why your app needs access.
    /// The usage description is optional, but highly recommended.
    ///
    /// App Sandbox enforces stricter limits on Documents folder access, so that policy may supersede this one if your app enables sandboxing.
    /// See App Sandbox for more information.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset SystemPolicyDocumentsFolder <bundleID>
    /// ```
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSDocumentsFolderUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub documents_folder_usage_description: Option<String>,
    /// A message that tells the user why the app needs access to the user’s Documents folder.
    ///
    /// The user implicitly grants your app access to a file in the Documents folder when selecting the file in an Open or Save panel, dragging it onto your app, or opening it in Finder.
    /// Your app can access that file right away and any time in the future.
    /// In addition, if your app creates a new file in the Documents folder, the app can access that file without user consent.
    ///
    /// The first time your app tries to access a file in the user’s Documents folder without implied user consent, the system prompts the user for permission to access the folder’s contents.
    /// Add the NSDocumentsFolderUsageDescription key to your app’s Information Property List file to provide a message that explains why your app needs access.
    /// The usage description is optional, but highly recommended.
    ///
    /// App Sandbox enforces stricter limits on Documents folder access, so that policy may supersede this one if your app enables sandboxing.
    /// See App Sandbox for more information.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset SystemPolicyDownloadsFolder <bundleID>
    /// ```
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSDownloadsFolderUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub downloads_folder_usage_description: Option<String>,
    /// A message that tells the user why the app needs access to files on a network volume.
    ///
    /// The user implicitly grants your app access to a file on a network volume when selecting the file in an Open or Save panel, dragging it onto your app, or opening it in Finder.
    /// Your app can access that file right away and any time in the future.
    /// In addition, if your app creates a new file on a network volume, the app can access that file without user consent.
    ///
    /// The first time your app tries to access a file on a network volume without implied user consent, the system prompts the user for permission to access network volumes.
    /// Add the NSNetworkVolumesUsageDescription key to your app’s Information Property List file to provide a string for the prompt that explains why your app needs access.
    /// The usage description is optional, but highly recommended.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset SystemPolicyNetworkVolumes <bundleID>
    /// ```
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSNetworkVolumesUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub network_volumes_usage_description: Option<String>,
    /// A message that tells the user why the app needs access to files on a removable volume.
    ///
    /// The user implicitly grants your app access to a file on a removable volume—like a USB thumb drive—when selecting the file in an Open or Save panel, dragging it onto your app, or opening it in Finder.
    /// Your app can access that file right away and any time in the future.
    /// In addition, if your app creates a new file on a removable volume, the app can access that file without user consent.
    ///
    /// The first time your app tries to access a file on a removable volume without implied user consent, the system prompts the user for permission to access removable volumes.
    /// Add the NSRemovableVolumesUsageDescription key to your app’s Information Property List file to provide a string for the prompt that explains why your app needs access.
    /// The usage description is optional, but highly recommended.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset SystemPolicyRemovableVolumes <bundleID>
    /// ```
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSRemovableVolumesUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub removable_volumes_usage_description: Option<String>,
    /// A message that tells the user why the app needs to be informed when other apps access files that it manages
    ///
    /// An app that adopts the File Provider framework can see when and with which other apps the user accesses managed files.
    /// Before providing this kind of information to a file provider, the system prompts the user to grant access.
    /// Add the NSFileProviderPresenceUsageDescription key to your file provider app’s Information Property List file to provide a string for the prompt that explains why your app needs this information.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset FileProviderPresence <bundleID>
    /// ```
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSFileProviderPresenceUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_provider_presence_usage_description: Option<String>,
    /// A message that tells the user why the app needs access to files managed by a file provider.
    ///
    /// The user implicitly grants your app access to a file managed by a file provider when selecting the file in an Open or Save panel, dragging it onto your app, or opening it in Finder.
    /// Your app can access that file right away and any time in the future.
    /// In addition, if your app creates a new file managed by a file provider, the app can access that file without user consent.
    ///
    /// The first time your app tries to access a file managed by a file provider without implied user consent, the system prompts the user for permission.
    /// Add the NSFileProviderDomainUsageDescription key to your app’s Information Property List file to provide a string for the prompt that explains why your app needs access.
    /// The usage description is optional, but highly recommended.
    ///
    /// After the user chooses whether to grant access, the system remembers the user’s choice.
    /// To reset permissions, use the tccutil command line utility with your app’s bundle ID:
    ///
    /// ```swift
    /// $ tccutil reset FileProviderDomain <bundleID>
    /// ```
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSFileProviderDomainUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_provider_domain_usage_description: Option<String>,
}

/// Game Center
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct GameCenter {
    /// A message that tells the user why the app needs access to their Game Center friends list.
    ///
    /// ## Availability
    /// * iOS 14.5+
    ///
    /// ## Framework
    /// * GameKit
    #[serde(
        rename(serialize = "NSGKFriendListUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub friend_list_usage_description: Option<String>,
}

/// Health
///
/// [Setting Up HealthKit](https://developer.apple.com/documentation/healthkit/setting_up_healthkit)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
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
    /// The HealthKit Entitlement provides access to most HealthKit data types.
    /// However, because of their highly sensitive nature, some data types require additional entitlements.
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
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub healthkit_access: Option<Vec<HealthKitCapabilities>>,
    /// A message to the user that explains why the app requested permission to read clinical records.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user's clinical records.
    ///
    /// ## Availability
    /// * iOS 12.0+
    ///
    /// ## Framework
    /// * HealthKit
    #[serde(
        rename(serialize = "NSHealthClinicalHealthRecordsShareUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub health_clinical_health_records_share_usage_description: Option<String>,
    /// A message to the user that explains why the app requested permission to read samples from the HealthKit store.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s heath data.
    ///
    /// ## Availability
    /// * iOS 8.0+
    ///
    /// ## Framework
    /// * HealthKit
    #[serde(
        rename(serialize = "NSHealthShareUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub health_share_usage_description: Option<String>,
    /// A message to the user that explains why the app requested permission to save samples to the HealthKit store.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that update the user’s health data.
    ///
    /// ## Availability
    /// * iOS 8.0+
    ///
    /// ## Framework
    /// * HealthKit
    #[serde(
        rename(serialize = "NSHealthUpdateUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub health_update_usage_description: Option<String>,
    /// The clinical record data types that your app must get permission to read.
    ///
    /// Use this key to indicate that your app requires access to specific clinical record data types to function properly.
    /// Set the value to an array of strings containing the type identifiers for your required types.
    /// For a list of type identifiers, see HKClinicalTypeIdentifier.
    ///
    /// To protect the user’s privacy, you must specify three or more required clinical record types.
    /// If the user denies authorization to any of the types, authorization fails with an HKError.Code.errorRequiredAuthorizationDenied error.
    /// Your app is not told the record types to which the user denied access.
    ///
    /// ## Availability
    /// * iOS 12.0+
    ///
    /// ## Framework
    /// * HealthKit
    #[serde(
        rename(serialize = "NSHealthRequiredReadAuthorizationTypeIdentifiers"),
        skip_serializing_if = "Option::is_none"
    )]
    pub health_required_read_authorization_type_identifiers: Option<Vec<String>>,
}

/// Home
///
/// [Enabling HomeKit in Your App](https://developer.apple.com/documentation/homekit/enabling_homekit_in_your_app)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Home {
    /// A message that tells the user why the app is requesting access to the user’s HomeKit configuration data.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s HomeKit configuration data.
    ///
    /// For more information about using HomeKit in your app, see Enabling HomeKit in Your App.
    ///
    /// ## Availability
    /// * iOS 8.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * HomeKit
    #[serde(
        rename(serialize = "NSHomeKitUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub home_kit_usage_description: Option<String>,
}

/// Location
///
/// [Choosing the Location Services Authorization to Request](https://developer.apple.com/documentation/corelocation/choosing_the_location_services_authorization_to_request)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Location {
    /// A message that tells the user why the app is requesting access to the user’s location information at all times.
    ///
    /// Use this key if your iOS app accesses location information while running in the background.
    /// If your app only needs location information when in the foreground, use NSLocationWhenInUseUsageDescription instead.
    /// For more information, see Choosing the Location Services Authorization to Request.
    ///
    /// If you need location information in a macOS app, use NSLocationUsageDescription instead.
    /// If your iOS app deploys to versions earlier than iOS 11, see NSLocationAlwaysUsageDescription.
    ///
    /// ### Important
    /// This key is required if your iOS app uses APIs that access the user’s location information at all times.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Core Location
    #[serde(
        rename(serialize = "NSLocationAlwaysAndWhenInUseUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_always_and_when_in_use_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting access to the user’s location information.
    ///
    /// Use this key in a macOS app that accesses the user’s location information.
    /// In an iOS app, use NSLocationWhenInUseUsageDescription or NSLocationAlwaysAndWhenInUseUsageDescription instead.
    ///
    /// ### Important
    /// This key is required if your macOS app uses APIs that access the user’s location information.
    ///
    /// ## Availability
    /// * iOS 6.0–8.0
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * Core Location
    #[deprecated(since = "iOS 6.0–8.0")]
    #[serde(
        rename(serialize = "NSLocationUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting access to the user’s location information while the app is running in the foreground.
    ///
    /// Use this key if your iOS app accesses location information only when running in the foreground.
    /// If your app needs location information when in the background, use NSLocationAlwaysAndWhenInUseUsageDescription instead.
    /// For more information, see Choosing the Location Services Authorization to Request.
    ///
    /// If you need location information in a macOS app, use NSLocationUsageDescription instead.
    ///
    /// ### Important
    /// This key is required if your iOS app uses APIs that access the user’s location information while the app is in use.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Core Location
    #[serde(
        rename(serialize = "NSLocationWhenInUseUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_when_in_use_usage_description: Option<String>,
    /// A collection of messages that explain why the app is requesting temporary access to the user’s location.
    ///
    /// Use this key if your app needs temporary access to full accuracy location information.
    /// Provide a dictionary of messages that address different use cases, keyed by strings that you define.
    /// For example, if your app suggests nearby coffee shops in one part of the app, and finds nearby friends in another, you could include two entries
    ///
    /// When you request access, select among the messages at run time by providing the associated key to the requestTemporaryFullAccuracyAuthorization(withPurposeKey:) method:
    /// ```swift
    /// // Request location access to find coffee shops.
    /// manager.requestTemporaryFullAccuracyAuthorization(withPurposeKey: "coffee")
    /// ````
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Core Location
    #[serde(
        rename(serialize = "NSLocationTemporaryUsageDescriptionDictionary"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_temporary_usage_description_dictionary: Option<DefaultDictionary>,
    /// A message that tells the user why the app is requesting access to the user's location at all times.
    ///
    /// Use this key if your iOS app accesses location information in the background, and you deploy to a target earlier than iOS 11.
    /// In that case, add both this key and NSLocationAlwaysAndWhenInUseUsageDescription to your app’s Info.plist file with the same message.
    /// Apps running on older versions of the OS use the message associated with NSLocationAlwaysUsageDescription, while apps running on later versions use the one associated with NSLocationAlwaysAndWhenInUseUsageDescription.
    ///
    /// If your app only needs location information when in the foreground, use NSLocationWhenInUseUsageDescription instead.
    /// For more information, see Choosing the Location Services Authorization to Request.
    ///
    /// If you need location information in a macOS app, use NSLocationUsageDescription instead.
    ///
    /// ### Important
    /// This key is required if your iOS app uses APIs that access the user’s location at all times and deploys to targets earlier than iOS 11.
    ///
    /// ## Availability
    /// * iOS 8.0–10.0
    ///
    /// ## Framework
    /// * Core Location
    #[deprecated(
        since = "iOS 8.0–10.0",
        note = "For apps deployed to targets in iOS 11 and later, use NSLocationAlwaysAndWhenInUseUsageDescription instead."
    )]
    #[serde(
        rename(serialize = "NSLocationAlwaysUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_always_usage_description: Option<String>,
    /// A Boolean value that indicates a widget uses the user’s location information.
    ///
    /// To access the user’s location information from a widget, set the value to true in the widget extension’s Info.plist file.
    ///
    /// Before a widget can access location information, the containing app must request authorization from the user.
    /// The containing app’s Info.plist file must also contain relevant purpose strings.
    /// For more information, see Requesting Authorization for Location Services.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * WidgetKit
    #[serde(
        rename(serialize = "NSWidgetWantsLocation"),
        skip_serializing_if = "Option::is_none"
    )]
    pub widget_wants_location: Option<bool>,
    /// A Boolean value that indicates whether the app requests reduced location accuracy by default.
    ///
    /// Include this key in your information property list to set your app’s default behavior for location accuracy when it calls the Core Location framework.
    /// Set the key value to true to prompt the user for reduced accuracy by default; set it to false to prompt for full location accuracy.
    /// If you don't include that key in your Info.plist, that's equivalent to setting it to false.
    ///
    /// Include the key pair in your Info.plist file as shown:
    ///
    /// <!-- Info.plist -->
    /// <key>NSLocationDefaultAccuracyReduced</key>
    /// <true/>
    ///
    /// When this key is set to true, all Core Location services (location updates, visit monitoring, significant location change, fence monitoring) receive service at the reduced-accuracy service level.
    /// Users will see that your app is asking for reduced accuracy because the location authorization prompt will show a map with an approximate location, and your app will have the Precise Location toggled off in Settings > Privacy > Location Services .
    /// These indicators of an app's improved privacy are ones that users may value.
    ///
    /// If you want to leverage the reduced-accuracy feature to improve privacy in a particular operation without setting this key, use the desiredAccuracy constant kCLLocationAccuracyReduced.
    /// This constant causes startUpdatingLocation() to deliver results as if the app were authorized for approximate location until you change the desiredAccuracy constant again.
    ///
    /// Setting NSLocationDefaultAccuracyReduced determines the default type of authorization your app gets, but users can override it any time in Settings.
    /// Users always control the level of location accuracy they want to share, and can change precision settings in Settings > Privacy > Location Services by selecting Precise Location for your app.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * watchOS 7.0+
    ///
    /// ## Framework
    /// * Core Location
    #[serde(
        rename(serialize = "NSLocationDefaultAccuracyReduced"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_default_accuracy_reduced: Option<bool>,
}

/// DefaultDictionary
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct DefaultDictionary {
    pub default: String,
}

/// Media Player
///
/// [Requesting Access to Apple Music Library](https://developer.apple.com/documentation/storekit/skcloudservicecontroller/requesting_access_to_apple_music_library)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct MediaPlayer {
    /// A message that tells the user why the app is requesting access to the user’s media library.
    ///
    /// Set the value of this key to a user-readable description of how you intend to use the user's media library.
    /// The first time your app access the user's media library, the system prompts the user to grant or deny authorization for your app to do so.
    /// The system includes this key's description in the dialog it displays to the user.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s media library.
    ///
    /// ## Availability
    /// * iOS 2.0+
    ///
    /// ## Framework
    /// * Media Player
    #[serde(
        rename(serialize = "NSAppleMusicUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub apple_music_usage_description: Option<String>,
}

/// Motion
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Motion {
    /// A message that tells the user why the app is requesting access to the device’s motion data.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the device’s motion data, including CMSensorRecorder, CMPedometer, CMMotionActivityManager, and CMMovementDisorderManager.
    /// If you don’t include this key, your app will crash when it attempts to access motion data.
    ///
    /// ## Availability
    /// * iOS 7.0+
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Core Motion
    #[serde(
        rename(serialize = "NSMotionUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub motion_usage_description: Option<String>,
    /// A message to the user that explains the app’s request for permission to access fall detection event data.
    ///
    /// ### Important
    /// If your app uses the CMFallDetectionManager, the app requires this key.
    ///
    /// ## Availability
    /// * watchOS 7.2+
    ///
    /// ## Framework
    /// * Core Motion
    #[serde(
        rename(serialize = "NSFallDetectionUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub fall_detection_usage_description: Option<String>,
}

/// Networking
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Networking {
    /// A message that tells the user why the app is requesting access to the local network.
    ///
    /// Any app that uses the local network, directly or indirectly, should include this description.
    /// This includes apps that use Bonjour and services implemented with Bonjour, as well as direct unicast or multicast connections to local hosts.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Network
    #[serde(
        rename(serialize = "NSLocalNetworkUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub local_network_usage_description: Option<String>,
    /// A request for user permission to begin an interaction session with nearby devices.
    ///
    /// Before an app starts an interaction session, the system requests permission to share the user’s relative distance and direction with a nearby peer.
    /// The framework presents a prompt that displays the string value of this key contained in your project’s Info.plist.
    /// Define text that explains your interaction session's purpose to the user.
    /// For more information, see Initiating and Maintaining a Session.
    ///
    /// This property is localizable.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * Nearby Interaction
    #[serde(
        rename(serialize = "NSNearbyInteractionAllowOnceUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub nearby_interaction_allow_once_usage_description: Option<String>,
}

/// NFC
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Nfc {
    /// A message that tells the user why the app is requesting access to the device’s NFC hardware.
    ///
    /// ### Important
    /// You’re required to provide this key if your app uses APIs that access the NFC hardware.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Core NFC
    #[serde(
        rename(serialize = "NFCReaderUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub nfc_reader_usage_description: Option<String>,
}

/// Photos
///
/// [Delivering a Great Privacy Experience in Your Photos App](https://developer.apple.com/documentation/photokit/delivering_a_great_privacy_experience_in_your_photos_app)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Photos {
    /// A message that tells the user why the app is requesting add-only access to the user’s photo library.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that have write access to the user’s photo library.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Photos
    #[serde(
        rename(serialize = "NSPhotoLibraryAddUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub photo_library_add_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting access to the user’s photo library.
    ///
    /// If your app only adds assets to the photo library and does not read assets, use the NSPhotoLibraryAddUsageDescription key instead.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that have read or write access to the user’s photo library.
    ///
    /// ## Availability
    /// * iOS 6.0+
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * Photos
    #[serde(
        rename(serialize = "NSPhotoLibraryUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub photo_library_usage_description: Option<String>,
}

/// Scripting
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Scripting {
    /// A Boolean value indicating whether AppleScript is enabled.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSAppleScriptEnabled"),
        skip_serializing_if = "Option::is_none"
    )]
    pub apple_script_enabled: Option<bool>,
}

/// Security
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Security {
    /// A message that informs the user why an app is requesting permission to use data for tracking the user or the device.
    ///
    /// If your app calls the App Tracking Transparency API, you must provide custom text, known as a usage-description string, which displays as a system-permission alert request.
    /// The usage-description string tells the user why the app is requesting permission to use data for tracking the user or the device.
    /// The user has the option to grant or deny the authorization request.
    /// If you don’t include a usage-description string, your app may crash when a user first launches it.
    ///
    /// Make sure your app requests permission to track sometime before tracking occurs.
    /// This could be at first launch or when using certain features in your app.
    /// For example, when signing on with a third-party SSO.
    ///
    /// Set the NSUserTrackingUsageDescription key in the Information Property List (Info.plist):
    ///
    /// 1. Select your project’s Info.plist file in Xcode Project navigator.
    ///
    /// 2. Modify the file using the Xcode Property List Editor: Privacy - Tracking Usage Description.
    ///
    /// * Use sentence-style capitalization and appropriate ending punctuation.
    /// Keep the text short and specific.
    /// You don’t need to include your app name because the system already identifies your app.
    ///
    /// * If the title is a sentence fragment, don’t add ending punctuation.
    ///
    /// See Apple’s Human Interface Guidelines for example usage descriptions.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSUserTrackingUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_tracking_usage_description: Option<String>,
    /// A message that tells the user why the app is requesting the ability to send Apple events.
    ///
    /// An app using Apple events to control another app might be able to gain access to sensitive user data.
    /// For example, the Mail app stores a lot of personal information in its local database that other apps can’t access directly.
    /// But because Mail can be automated with Apple events, other apps can use Mail to gain access to the data indirectly.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that send Apple events.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSAppleEventsUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub apple_events_usage_description: Option<String>,
    /// A message in macOS that tells the user why the app is requesting to manipulate the system configuration.
    ///
    /// Use this key if your app uses certain APIs that manipulate system configuration, like ODRecordSetValue(_:_:_:_:).
    ///
    /// ### Important
    /// This key is required if your app uses APIs that manipulate the system configuration.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSSystemAdministrationUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub system_administration_usage_description: Option<String>,
    /// A Boolean value indicating whether the app uses encryption.
    ///
    /// Set the value for this key to NO in your app’s Information Property List file to indicate that your app—including any third-party libraries you link against—either uses no encryption, or only uses encryption that’s exempt from export compliance requirements, as described in Determine your export compliance requirements.
    /// Set the value to YES to indicate that your app uses non-exempt encryption.
    ///
    /// If you set the value to YES, you typically also provide a value for the ITSEncryptionExportComplianceCode key.
    /// You set that key’s value using a code Apple provides after successfully reviewing your export compliance documentation.
    ///
    /// If you don’t have the ITSAppUsesNonExemptEncryption key in your app’s Info.plist file, App Store Connect walks you through an export compliance questionnaire every time you upload a new version of your app.
    /// Including the key streamlines the app submission process.
    ///
    /// For additional information, see Complying with Encryption Export Regulations.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "ITSAppUsesNonExemptEncryption"),
        skip_serializing_if = "Option::is_none"
    )]
    pub app_uses_non_exempt_encryption: Option<bool>,
    /// The export compliance code provided by App Store Connect for apps that require it.
    ///
    /// Include this key in your app’s Information Property List file if you set the ITSAppUsesNonExemptEncryption key’s value to YES.
    /// Set the value for this key to the code that Apple sends you after successfully reviewing export compliance documentation that you provide through App Store Connect.
    ///
    /// For additional information, see Complying with Encryption Export Regulations.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "ITSEncryptionExportComplianceCode"),
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_export_compliance_code: Option<String>,
}

/// Sensors
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Sensors {
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SensorKit
    #[serde(
        rename(serialize = "NSSensorKitUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_kit_usage_description: Option<String>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SensorKit
    #[serde(
        rename(serialize = "NSSensorKitUsageDetail"),
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_kit_usage_detail: Option<DefaultDictionary>,
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SensorKit
    #[serde(
        rename(serialize = "NSSensorKitPrivacyPolicyURL"),
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_kit_privacy_policy_url: Option<String>,
}

/// Siri
///
/// [Requesting Authorization to Use SiriKit](https://developer.apple.com/documentation/sirikit/requesting_authorization_to_use_sirikit)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Siri {
    /// A message that tells the user why the app is requesting to send user data to Siri.
    ///
    /// ### important
    /// This key is required if your app uses APIs that send user data to Siri
    ///
    /// ## Availability
    /// * iOS 10.0+
    ///
    /// ## Framework
    /// * Intents
    #[serde(
        rename(serialize = "NSSiriUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub siri_usage_description: Option<String>,
}

/// Speech
///
/// [Asking Permission to Use Speech Recognition](https://developer.apple.com/documentation/speech/asking_permission_to_use_speech_recognition)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Speech {
    /// A message that tells the user why the app is requesting to send user data to Apple’s speech recognition servers.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that send user data to Apple’s speech recognition servers.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Speech
    #[serde(
        rename(serialize = "NSSpeechRecognitionUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub speech_recognition_usage_description: Option<String>,
}

/// TV Resource
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TVResource {
    /// A message that tells the user why the app is requesting access to the user’s TV provider account.
    ///
    /// ### Important
    /// This key is required if your app uses APIs that access the user’s TV provider account.
    ///
    /// ## Availability
    /// * tvOS 12.0+
    ///
    /// ## Framework
    /// * TVUIKit
    #[serde(
        rename(serialize = "NSVideoSubscriberAccountUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub video_subscriber_account_usage_description: Option<String>,
}

/// Wi-Fi
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct WiFI {
    /// A Boolean value indicating whether the app requires a Wi-Fi connection.
    ///
    /// ## Availability
    /// * iOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename(serialize = "UIRequiresPersistentWiFi"),
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_persistent_wifi: Option<bool>,
}

/// Health Kit Capabilities
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum HealthKitCapabilities {
    /// The app can request access to FHIR-backed clinical records.
    #[serde(rename(serialize = "health-records"))]
    HealthRecords,
}

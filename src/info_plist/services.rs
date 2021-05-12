use crate::{serialize_enum_option, serialize_vec_enum_option};
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
    /// $ tccutil reset SystemPolicyDesktopFolder <bundleID>
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
    /// $ tccutil reset SystemPolicyDocumentsFolder <bundleID>
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
    /// $ tccutil reset SystemPolicyDownloadsFolder <bundleID>
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
    /// $ tccutil reset SystemPolicyNetworkVolumes <bundleID>
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
    /// $ tccutil reset SystemPolicyRemovableVolumes <bundleID>
    ///
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
    /// $ tccutil reset FileProviderPresence <bundleID>
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
    /// $ tccutil reset FileProviderDomain <bundleID>
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Health {
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
}

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
/// ## Accessing the Event Store
/// Request access to a user’s calendar data through the event store.
///
/// ### Overview
/// Your app must obtain permission from the user before accessing the calendar database: An app must never directly modify the calendar database on its own.
/// EKEventStore is the application’s point of contact for accessing calendar and reminder data.
///
/// Connect to the Event Store
/// To receive event or reminder data, you must request access to an entity type after initializing the event store with requestAccess(to:completion:).
///
/// Connect to the event store with the following initializer:
/// ```swift
/// // Initialize the store.
/// var store = EKEventStore()
///
/// // Request access to reminders.
/// store.requestAccess(to: .reminder) { granted, error in
///  // Handle the response to the request.
/// }
/// ```
///
/// Releasing an event store instance before other EventKit objects may result in an error.
///
/// ### Protect User Privacy with Information Property List Keys
/// An iOS app linked on or after iOS 10.0 must include in its Info.plist file the usage description keys for the types of data it needs to access.
/// To access the user's calendar events, reminders, and contacts through EventKitUI, you need to include descriptions for:
///
/// * NSCalendarsUsageDescription
///
/// * NSRemindersUsageDescription
///
/// * NSContactsUsageDescription
///
/// The EventKitUI framework may need to access Contacts data to choose the correct display name or avatar for a contact in a calendar.
///
/// ### Warning
/// If you don’t include these keys, your app will crash.
///
/// ### Else
/// Because these keys provide access to the event store, they protect the user's privacy by only allowing access to this information if the user grants permission explicitly in the app.
///
/// To access the user’s Calendar data, all sandboxed macOS apps must include the com.apple.security.personal-information key.
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
/// ## Requesting Authorization for Media Capture on iOS
/// Respect user privacy by seeking permission to capture and store photos, audio, and video.
///
/// ### Overview
/// In iOS, the user must explicitly grant permission for each app to access cameras and microphones.
/// Before your app can use the capture system for the first time, iOS shows an alert asking the user to grant your app access to the camera, as shown below.
/// iOS remembers the user’s response to this alert, so subsequent uses of the capture system don’t cause it to appear again
/// The user can change permission settings for your app in Settings > Privacy.
///
/// To make sure your app has permission before capturing media, follow the steps below.
///
/// ## Configure Your App's Info.plist File
/// iOS requires that your app provide static messages to display to the user when the system asks for camera or microphone permission:
///
/// * If your app uses device cameras, include the NSCameraUsageDescription key in your app’s Info.plist file.
///
/// * If your app uses device microphones, include the NSMicrophoneUsageDescription key in your app’s Info.plist file.
///
/// For each key, provide a message that explains to the user why your app needs to capture media, so that the user can feel confident granting permission to your app.
///
/// ### Important
/// If the appropriate key is not present in your app’s Info.plist file when your app requests authorization or attempts to use a capture device, the system terminates your app.
///
/// ### Verify and Request Authorization for Capture
/// Always test the AVCaptureDevice authorizationStatus(for:) method before setting up a capture session.
/// If the user has not yet granted or denied capture permission, the authorization status is AVAuthorizationStatus.notDetermined.
/// In this case, use the requestAccess(for:completionHandler:) method to tell iOS to prompt the user:
///
/// ```swift
/// switch AVCaptureDevice.authorizationStatus(for: .video) {
///    case .authorized: // The user has previously granted access to the camera.
///        self.setupCaptureSession()
///    
///    case .notDetermined: // The user has not yet been asked for camera access.
///        AVCaptureDevice.requestAccess(for: .video) { granted in
///            if granted {
///                self.setupCaptureSession()
///            }
///        }
///    
///    case .denied: // The user has previously denied access.
///        return
///
///    case .restricted: // The user can't grant access due to restrictions.
///        return
/// }
/// ```
///
/// The requestAccess(for:completionHandler:) method is asynchronous: Your app continues running while iOS shows the permission alert.
/// When the user responds, the system calls your completion handler. If the completion handler’s success parameter is true, you can proceed to set up and start a capture session.
///
/// ### Note
/// Call requestAccess(for:completionHandler:) before starting capture, but only at a time that’s appropriate for your app.
/// For example, if photo or video recording isn’t the main focus of your app, check for camera permission only when the user invokes your app’s camera-related features.
///
/// ### Request Authorization Before Saving Captured Media
/// After capturing photos or video, you may want to save them into the user’s Photos library.
/// Accessing the Photos library also requires user permission (separate from camera and microphone permission).
/// How and when you request permission depends on which features you use for saving media:
///
/// * For most photo and video capture workflows (including Live Photos and RAW format capture), use the PHPhotoLibrary and PHAssetCreationRequest classes.
/// These classes require read/write access to the Photos library, so you must use the use the NSPhotoLibraryUsageDescription key in your Info.plist to provide a message to the user when asking for access.
/// For details, see Saving Captured Photos.
///
/// * If your app needs only to save movie files to the Photos library, the UISaveVideoAtPathToSavedPhotosAlbum(_:_:_:_:) function provides a simpler alternative to PHPhotoLibrary.
/// This function requires only write access to the library, so use the NSPhotoLibraryAddUsageDescription key in your Info.plist to provide a message to the user when asking for permission to save to the Photos library.
///
/// ### Note
/// The UIImageWriteToSavedPhotosAlbum(_:_:_:_:) function is not recommended for use with photos captured with AVCapturePhotoOutput, because the UIImage class does not support the features and metadata included in photo output.
///
/// ## Requesting Authorization for Media Capture on macOS
/// Prompt the user to authorize access to the camera and microphone.
///
/// ### Overview
/// In macOS 10.14 and later, the user must explicitly grant permission for each app to access cameras and microphones.
/// Before your app can use the capture system for the first time, macOS shows an alert asking the user to grant your app access to the camera, as shown below.
/// macOS remembers the user’s response to this alert, so subsequent uses of the capture system don’t cause it to appear again.
/// The user can change permission settings for your app in System Preferences > Security & Privacy.
/// The request for authorization looks different from the alert UI in iOS.
///
/// To make sure your app has permission before capturing media, follow the steps below.
///
/// ### Configure Your Camera and Microphone Apps
/// The information property list keys for Camera and Microphone in macOS operate the same way as they do in iOS. macOS 10.14 and later populates the static messages with these strings when the system asks for camera or microphone permission:
///
/// * If your app uses device cameras, include the NSCameraUsageDescription key in your app’s Info.plist file.
///
/// * If your app uses device microphones, include the NSMicrophoneUsageDescription key in your app’s Info.plist file.
///
/// For each key, provide a message that explains to the user why your app needs to capture media, so that the user can feel confident granting permission to your app.
///
/// ### Important
/// If the appropriate key is not present in your app’s Info.plist file when your app requests authorization or attempts to use a capture device, the system terminates your app.
/// The Xcode debug console displays a message that explains the reason for the crash.
///
/// ### Verify and Request Authorization for Capture
/// Always test the AVCaptureDevice authorizationStatus(for:) method before setting up a capture session. If the user has not yet granted or denied capture permission, the authorization status is AVAuthorizationStatus.notDetermined.
/// In this case, use the requestAccess(for:completionHandler:) method to have macOS prompt the user:
///
/// ```swift
/// switch AVCaptureDevice.authorizationStatus(for: .video) {
///     case .authorized: // The user has previously granted access to the camera.
///         self.setupCaptureSession()
///     
///     case .notDetermined: // The user has not yet been asked for camera access.
///         AVCaptureDevice.requestAccess(for: .video) { granted in
///             if granted {
///                 self.setupCaptureSession()
///             }
///         }
///     
///     case .denied: // The user has previously denied access.
///         return
///
///     case .restricted: // The user can't grant access due to restrictions.
///         return
/// }
/// ```
///
/// The requestAccess(for:completionHandler:) method is asynchronous: Your app continues running while macOS shows the permission alert.
/// When the user responds, the system calls your completion handler.
/// If the completion handler’s success parameter is true, you can proceed to set up and start a capture session.
///
/// ### Note
/// Call requestAccess(for:completionHandler:) before starting capture, but only at a time that’s appropriate for your app.
/// For example, if photo or video recording isn’t the main focus of your app, check for camera permission only when the user invokes your app’s camera-related features.
///
/// ### Request Authorization Before Saving Captured Media
/// After capturing photos or video, you may want to save them into the user’s Photos library.
/// Accessing the Photos library also requires user permission (separate from camera and microphone permission).
/// For most photo and video capture workflows (including Live Photos and RAW format capture), use the PHPhotoLibrary and PHAssetCreationRequest classes.
/// These classes require read/write access to the Photos library, so you must use the use the NSPhotoLibraryUsageDescription key in your information property list to provide a message to the user when asking for access.
/// For details, see Saving Captured Photos.
///
/// ### Reset Authorization from Terminal
/// To reset the state of the user’s decision to grant or reject Microphone access so the prompt shows again, open Terminal and input the command:
/// ```swift
/// tccutil reset Microphone
/// ```
/// To reset the authorization state for camera access, type:
/// ```swift
/// tccutil reset Camera
/// ```
/// This command resets the access authorization settings for all apps, so other apps will prompt the user again.
/// Use this tool to debug the appearance of your privacy justification strings and their localized versions.
///
/// ### Tip
/// You can use tccutil to reset authorization access settings for other system services as well, such as AddressBook, Calendar, and Finder.
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
/// ## Requesting Authorization to Access Contacts
/// Request permission from the user to read, create, and modify their Contacts entries.
///
/// ### Overview
/// Your app can’t access Contacts entries until the user authorizes it to do so.
/// To check whether the user has authorized access, use authorizationStatus(for:).
/// When the authorization status is CNAuthorizationStatus.notDetermined, you can request authorization, which displays a prompt to the user.
/// For any other authorization status, requesting authorization doesn’t prompt the user again.
/// To provide context that helps users understand why your app needs access, request authorization close to when your app actually needs it.
///
/// ### Important
/// If your app is linked against iOS 13 or later and requests the note field of a contact, you must include the com.apple.developer.contacts.notes entitlement.
///
/// ### Configure Your Information Property List File
/// Add the required NSContactsUsageDescription key to your app’s Info.plist file.
/// The value for this key is a string that describes what your app does with the user’s contacts.
/// Be concise but clear, so users feel confident granting authorization.
/// Your app terminates if you request authorization without this key in the Info.plist file.
///
/// ### Request Authorization from Your App
/// Call the requestAccess(for:completionHandler:) class method of CNContactStore before accessing Contacts entries.
/// Specify CNEntityType.contacts in the call.
///
/// Users can approve or deny your app’s request for authorization, and can change your app’s authorization status after their initial response.
/// The system remembers your app’s authorization status so that subsequent calls to the requestAccess(for:completionHandler:) method don’t prompt the user again.
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
///
/// ## Setting Up HealthKit
/// Set up and configure your HealthKit store.
///
/// ### Overview
/// Before using HealthKit, you must perform the following steps:
/// 1. Enable HealthKit in your app.
/// 2. Ensure that HealthKit is available on the current device.
/// 3. Create your app’s HealthKit store.
/// 4. Request permission to read and share data.
///
/// The first three steps are described in detail below.
/// For more information on requesting authorization, see Authorizing Access to Health Data.
/// For a practical example of how to set up and use HealthKit, see SpeedySloth: Creating a Workout.
///
/// ### Enable HealthKit
/// Before you can use HealthKit, you must enable the HealthKit capabilities for your app.
/// In Xcode, select the project and add the HealthKit capability (see Figure 1).
/// Only select the Clinical Health Records checkbox if your app needs to access the user’s clinical records.
/// App Review may reject apps that enable the Clinical Health Records capability if the app doesn’t actually use the health record data. For more information, see Accessing Health Records.
///
/// For a detailed discussion about enabling capabilities, see Configure HealthKit in Xcode Help.
///
/// When you enable the HealthKit capabilities on an iOS app, Xcode adds HealthKit to the list of required device capabilities, which prevents users from purchasing or installing the app on devices that don’t support HealthKit.
///
/// If HealthKit isn’t required for the correct operation of your app, you can open the app’s Info.plist file and delete the healthkit entry from the Required device capabilities array.
/// The healthkit entry isn’t used by WatchKit extensions.
///
/// For more information on required device capabilities, see the UIRequiredDeviceCapabilities key in Information Property List Key Reference.
///
/// ### Ensure HealthKit’s Availability
/// Call the isHealthDataAvailable() method to confirm that HealthKit is available on the user's device.
/// ```swift
/// if HKHealthStore.isHealthDataAvailable() {
///    // Add code to use HealthKit here.
/// }
/// ```
/// Call this method before calling any other HealthKit methods.
/// If HealthKit is not available on the device (for example, on an iPad), other HealthKit methods fail with an errorHealthDataUnavailable error.
/// If HealthKit is restricted (for example, in an enterprise environment), the methods fail with an errorHealthDataRestricted error.
///
/// ### Create the HealthKit Store
/// If HealthKit is both enabled and available, instantiate an HKHealthStore object for your app as shown:
/// ```swift
/// let healthStore = HKHealthStore()
/// ```
/// You need only a single HealthKit store per app.
/// These are long-lived objects; you create the store once, and keep a reference for later use.
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
        rename(serialize = "NSLocationAlwaysAndWhenInUseUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location_always_and_when_in_use_usage_description: Option<String>,
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
pub struct NFC {
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
        rename(serialize = "NSSensorKitUsageDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub siri_usage_description: Option<String>,
}

/// Speech
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Speech {
    /// A message that tells the user why the app is requesting to send user data to Apple’s speech recognition servers.
    ///
    /// ### important
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

/// TV
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TV {
    /// A message that tells the user why the app is requesting access to the user’s TV provider account.
    ///
    /// ### important
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HealthKitCapabilities {
    /// The app can request access to FHIR-backed clinical records.
    #[serde(rename(serialize = "health-records"))]
    HealthRecords,
}

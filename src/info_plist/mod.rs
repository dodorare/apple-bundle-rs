//! # Information Property List.
//!
//! A resource containing key-value pairs that identify and configure a bundle.
//!
//! Bundles, which represent executables of different kinds, contain an information
//! property list file. This collection of key-value pairs specifies how the system should
//! interpret the associated bundle. Some key-value pairs characterize the bundle itself,
//! while others configure the app, framework, or other entity that the bundle represents.
//! Some keys are required, while others are specific to particular features of the
//! executable.
//!
//! The information property list file always has the name Info.plist. The file name is
//! case-sensitive and must begin with a capital letter I. Its location within the bundle
//! depends on both the bundle type and the platform. For example, iOS app bundles store
//! the file in the bundle’s root directory, whereas macOS app bundles place the
//! Info.plist file in the Contents directory.
//!
//! To access an information property list, you use an instance of the Bundle class, which
//! represents a bundle on disk. You can get the value for a few common keys by accessing
//! properties of the bundle instance. For example, the bundleIdentifier property contains
//! the value associated with the CFBundleIdentifier key. You can obtain the value for an
//! arbitrary key using the object(forInfoDictionaryKey:) method.
//!
//! Official documentation: https://developer.apple.com/documentation/bundleresources/information_property_list
//!
//! ## Availability
//! * iOS 2.0+
//! * macOS 10.0+
//! * tvOS 9.0+
//! * watchOS 2.0+
//!
//! ## Framework
//! Bundle Resources

pub mod app_execution;
pub mod app_services;
pub mod bundle_configuration;
pub mod data_and_storage;
pub mod kernel_and_drivers;
pub mod protected_resources;
pub mod user_interface;

pub mod prelude {
    pub use super::app_execution::*;
    pub use super::app_services::*;
    pub use super::bundle_configuration::*;
    pub use super::data_and_storage::*;
    pub use super::kernel_and_drivers::*;
    pub use super::protected_resources::*;
    pub use super::user_interface::*;
    pub use super::InfoPlist;
}

use prelude::*;
use serde::{Deserialize, Serialize};

/// Information property list
///
/// https://developer.apple.com/documentation/bundleresources/information_property_list
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct InfoPlist {
    // Bundle Configuration
    #[serde(flatten)]
    pub categorization: Categorization,
    #[serde(flatten)]
    pub identification: Identification,
    #[serde(flatten)]
    pub naming: Naming,
    #[serde(flatten)]
    pub bundle_version: BundleVersion,
    #[serde(flatten)]
    pub operating_system_version: OperatingSystemVersion,
    #[serde(flatten)]
    pub localization: Localization,
    #[serde(flatten)]
    pub help: Help,
    // User Interface
    #[serde(flatten)]
    pub main_user_interface: MainUserInterface,
    #[serde(flatten)]
    pub launch_interface: LaunchInterface,
    #[serde(flatten)]
    pub icons: Icons,
    #[serde(flatten)]
    pub orientation: Orientation,
    #[serde(flatten)]
    pub styling: Styling,
    #[serde(flatten)]
    pub status_bar: StatusBar,
    #[serde(flatten)]
    pub preferences: Preferences,
    #[serde(flatten)]
    pub graphics: Graphics,
    #[serde(flatten)]
    pub quick_look: QuickLook,
    // App Execution
    #[serde(flatten)]
    pub launch: Launch,
    #[serde(flatten)]
    pub launch_conditions: LaunchConditions,
    #[serde(flatten)]
    pub extensions_and_services: ExtensionsAndServices,
    #[serde(flatten)]
    pub app_clips: AppClips,
    #[serde(flatten)]
    pub background_execution: BackgroundExecution,
    #[serde(flatten)]
    pub endpoint_security: EndpointSecurity,
    #[serde(flatten)]
    pub plugin_support: PluginSupport,
    #[serde(flatten)]
    pub plugin_configuration: PluginConfiguration,
    #[serde(flatten)]
    pub termination: Termination,
    // Protected Resources
    #[serde(flatten)]
    pub bluetooth: Bluetooth,
    #[serde(flatten)]
    pub calendar_and_reminders: CalendarAndReminders,
    #[serde(flatten)]
    pub camera_and_microphone: CameraAndMicrophone,
    #[serde(flatten)]
    pub contacts: Contacts,
    #[serde(flatten)]
    pub face_id: FaceId,
    #[serde(flatten)]
    pub files_and_folders: FilesAndFolders,
    #[serde(flatten)]
    pub game_center: GameCenter,
    #[serde(flatten)]
    pub health: Health,
    #[serde(flatten)]
    pub home: Home,
    #[serde(flatten)]
    pub location: Location,
    #[serde(flatten)]
    pub media_player: MediaPlayer,
    #[serde(flatten)]
    pub motion: Motion,
    #[serde(flatten)]
    pub networking: Networking,
    #[serde(flatten)]
    pub nfc: Nfc,
    #[serde(flatten)]
    pub photos: Photos,
    #[serde(flatten)]
    pub scripting: Scripting,
    #[serde(flatten)]
    pub security: Security,
    #[serde(flatten)]
    pub sensors: Sensors,
    #[serde(flatten)]
    pub siri: Siri,
    #[serde(flatten)]
    pub speech: Speech,
    #[serde(flatten)]
    pub tv_resource: TvResource,
    #[serde(flatten)]
    pub wi_fi: WiFi,
    // Data and Storage
    #[serde(flatten)]
    pub documents: Documents,
    #[serde(flatten)]
    pub url_schemes: UrlSchemes,
    #[serde(flatten)]
    pub universal_type_identifiers: UniversalTypeIdentifiers,
    #[serde(flatten)]
    pub network: Network,
    #[serde(flatten)]
    pub storage: Storage,
    #[serde(flatten)]
    pub core_ml_models: CoreMlModels,
    #[serde(flatten)]
    pub java: Java,
    // App Services
    #[serde(flatten)]
    pub carplay: CarPlay,
    #[serde(flatten)]
    pub exposure_notification: ExposureNotification,
    #[serde(flatten)]
    pub pointer_interactions: PointerInteractions,
    #[serde(flatten)]
    pub games: Games,
    #[serde(flatten)]
    pub intents: Intents,
    #[serde(flatten)]
    pub maps: Maps,
    #[serde(flatten)]
    pub nfc_app_services: NfcAppServices,
    #[serde(flatten)]
    pub authentication: Authentication,
    #[serde(flatten)]
    pub external_accessories: ExternalAccessories,
    #[serde(flatten)]
    pub service_management: ServiceManagement,
    #[serde(flatten)]
    pub interprocess_communication: InterprocessCommunication,
    #[serde(flatten)]
    pub store: Store,
    // Kernel and Drivers
    #[serde(flatten)]
    pub driver_personalities: DriverPersonalities,
    #[serde(flatten)]
    pub kext_dependencies: KextDependencies,
    #[serde(flatten)]
    pub thunderbolt_compatibility: ThunderboltCompatibility,
}

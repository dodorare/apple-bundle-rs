//! # App Services
//!
//! Configure services provided by the app, like support for giving directions or using
//! game controllers.
//!
//! Add keys to your app’s Information Property List file that tell the system about
//! services that your app provides.
//!
//! ## Framework
//! * Bundle Resources

use super::DefaultDictionary;
use crate::{serialize_enum_option, serialize_vec_enum_option};
use serde::{Deserialize, Serialize};

/// Car Play
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct CarPlay {
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "CPSupportsDashboardNavigationScene",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_dashboard_navigation_scene: Option<bool>,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "CPTemplateApplicationDashboardSceneSessionRoleApplication",
        skip_serializing_if = "Option::is_none"
    )]
    pub template_application_dashboard: Option<Vec<TemplateApplicationDashboard>>,
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename = "CPTemplateApplicationSceneSessionRoleApplication",
        skip_serializing_if = "Option::is_none"
    )]
    pub template_application_scene_session_role: Option<Vec<TemplateApplicationSceneSessionRole>>,
}

/// Template Application Dashboard
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TemplateApplicationDashboard {
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename = "UISceneClassName")]
    pub scene_class_name: ClassName,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename = "UISceneConfigurationName")]
    pub scene_configuration_name: String,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename = "UISceneDelegateClassName")]
    pub scene_delegate_class_name: String,
}

/// Class Name
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ClassName {
    #[serde(rename = "CPTemplateApplicationDashboardScene")]
    TemplateApplicationDashboardScene,
}

impl Default for ClassName {
    fn default() -> Self {
        Self::TemplateApplicationDashboardScene
    }
}

/// Template Application Scene Session Role
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TemplateApplicationSceneSessionRole {
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename = "UISceneClassName")]
    pub scene_class_name: TemplateApplication,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename = "UISceneConfigurationName")]
    pub scene_configuration_name: String,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename = "UISceneDelegateClassName")]
    pub scene_delegate_class_name: String,
}

/// Template Application
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TemplateApplication {
    #[serde(rename = "CPTemplateApplicationScene")]
    Scene,
}

impl Default for TemplateApplication {
    fn default() -> Self {
        Self::Scene
    }
}

/// Exposure Notification
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ExposureNotification {
    /// A number that specifies the version of the API to use.
    ///
    /// ### Important
    /// This type is available in iOS 12.5, and in iOS 13.7 and later.
    ///
    /// iOS 13.7 introduces a new method to calculate the user’s Exposure Risk Value,
    /// described in ENExposureConfiguration. Set this value to 2 to use this new
    /// version and its calculation method, or set this value to 1 to use the earlier API
    /// and its calculation method. If you don’t explicitly set this value, the
    /// default is 1.
    ///
    /// ## Availability
    /// * iOS 13.7+
    ///
    /// ## Framework
    /// * Exposure Notification
    #[serde(
        rename = "ENAPIVersion",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub version: Option<Version>,
    /// A string that specifies the region that the app supports.
    ///
    /// ### Important
    /// This type is available in iOS 12.5, and in iOS 13.7 and later.
    ///
    /// All ExposureNotification apps must specify the region for which they work by
    /// adding this key to the app’s Info.plist file. The value is a string that
    /// represents the app’s region. This value can be an ISO 3166-1 country code
    /// (e.g. “CA” for Canada), or the ISO 3166-1/3166-2 country code plus subdivision
    /// code (“US-CA” for California).
    ///
    /// ## Availability
    /// * iOS 13.7+
    ///
    /// ## Framework
    /// * Exposure Notification
    #[serde(rename = "ENDeveloperRegion", skip_serializing_if = "Option::is_none")]
    pub developer_region: Option<String>,
}

/// Version
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Version {
    /// Use version 1 of the API.
    #[serde(rename = "1")]
    One,
    /// Use version 2 of the API.
    #[serde(rename = "2")]
    Two,
}

/// Pointer Interactions
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct PointerInteractions {
    /// A Boolean value indicating that the app generally supports indirect input
    /// mechanisms.
    ///
    /// If this key is not present or returns NO:
    /// * When the user clicks an indirect pointing device, UIKit generates a UITouch of
    ///   type UITouch.TouchType.direct.
    /// * When pinching or rotating using an indirect touch surface, UIKit creates touches
    ///   a fixed distance apart that simulate the gesture on the indirect touch surface.
    /// * Because these are normal UITouch events, they may incidentally activate other
    ///   gesture recognizers
    ///
    /// If the key is present and returns YES:
    /// * When the user clicks an indirect pointing device, UIKit generates a UITouch of
    ///   type UITouch.TouchType.indirectPointer.
    /// * When pinching or rotating using an indirect touch surface, UIKit drives
    ///   UIPinchGestureRecognizer and UIRotationGestureRecognizer with an event of type
    ///   UIEvent.EventType.transform.
    /// * Currently, only certain prepackaged gestures in UIKit, like
    ///   UIPinchGestureRecognizer and UIRotationGestureRecognizer, are capable of
    ///   handling this event.
    /// Other gestures may be added to this list in future releases.
    /// * Gestures that may have worked previously with the simulated touches no longer
    ///   work.
    /// * Be careful with certain UIGestureRecognizer APIs when gestures are driven by
    ///   events of type UIEvent.EventType.scroll or
    ///   UIEvent.EventType.transform.numberOfTouches returns 0, andlocation(ofTouch:in:)
    ///   raises an exception because there are no touches driving these gestures with
    ///   those events.
    ///
    /// For the case when exceptions might be raised, use either shouldReceive(_:) or the
    /// delegate call of gestureRecognizer(_:shouldReceive:) to determine that gesture
    /// recognizers are acting on a non-touch-based event.
    ///
    /// ### Important
    /// UIApplicationSupportsIndirectInputEvents is a compatibility affordance to ease the
    /// adoption of indirect input for a UIKit application. In a future release, this
    /// new behavior will become the default and this key will no longer be consulted.
    ///
    /// ## Availability
    /// * iOS 13.4+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIApplicationSupportsIndirectInputEvents",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_supports_indirect_input_events: Option<bool>,
}

/// Games
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Games {
    /// A Boolean value indicating whether GameKit can add badges to a turn-based game
    /// icon.
    ///
    /// ## Availability
    /// * iOS 7.0+
    ///
    /// ## Framework
    /// * GameKit
    #[serde(
        rename = "GKGameCenterBadgingDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub game_center_badging_disabled: Option<bool>,
    /// A Boolean value that indicates whether GameKit can display challenge banners in a
    /// game.
    ///
    /// ## Availability
    /// * iOS 7.0+
    ///
    /// ## Framework
    /// * GameKit
    #[serde(
        rename = "GKShowChallengeBanners",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_challenge_banners: Option<bool>,
    /// The types of game controllers allowed or required by the app.
    ///
    /// ## Availability
    /// * iOS 7.0+
    /// * macOS 10.9+
    /// * tvOS 9.0+
    ///
    /// ## Framework
    /// * Game Controller
    #[serde(
        rename = "GCSupportedGameControllers",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub supported_game_controllers: Option<Vec<ProfileName>>,
    /// A Boolean value indicating whether the app supports a game controller.
    ///
    /// To add this key to the Information Property List, enable the Game Controllers
    /// capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 7.0+
    /// * macOS 10.9+
    /// * tvOS 9.0+
    ///
    /// ## Framework
    /// * Game Controller
    #[serde(
        rename = "GCSupportsControllerUserInteraction",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_controller_user_interaction: Option<bool>,
    /// A Boolean value indicating whether the physical Apple TV Remote and the Apple TV
    /// Remote app operate as separate game controllers.
    ///
    /// ## Availability
    /// * tvOS 9.0+
    ///
    /// ## Framework
    /// * Game Controller
    #[serde(
        rename = "GCSupportsMultipleMicroGamepads",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_multiple_micro_gamepads: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProfileName {
    #[serde(rename = "ExtendedGamepad")]
    ExtendedGamepad,
    #[serde(rename = "MicroGamepad")]
    MicroGamepad,
}

/// Intents
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Intents {
    /// The names of the intent classes your app handles directly.
    ///
    /// Provide the class name of each INIntent subclass your app can handle.
    /// To specify this information in Xcode, add the class names in the Supported Intents
    /// section of your app target in the Project Editor.
    ///
    /// For more information on handling intents in your app, see
    /// application(_:handlerFor:).
    ///
    /// ### Tip
    /// You can start handling an intent in your app even if you want to support the
    /// intent in iOS 13. List the intent in the Supported Intents sections for both
    /// the app target and the extension target. For an app running on iOS 13, the
    /// system routes the intent with handler(for:), and for later iOS versions, it routes
    /// the intent with application(_:handlerFor:).
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Intents
    #[serde(rename = "INIntentsSupported", skip_serializing_if = "Option::is_none")]
    pub intents_supported: Option<Vec<String>>,
    /// The names of the intent classes your app can’t handle when the user locks the
    /// device.
    ///
    /// To specify this information in Xcode, add the intent class name to your app
    /// target’s Supported Intents in the Project Editor. Then set the Authentication
    /// level to Restricted While Locked.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Intents
    #[serde(
        rename = "INIntentsRestrictedWhileLocked",
        skip_serializing_if = "Option::is_none"
    )]
    pub intents_restricted_while_locked: Option<Vec<String>>,
    /// The names of the intent classes your app can’t handle when the user locks the
    /// device or the system blocks access to protected data.
    ///
    /// To specify this information in Xcode, add the intent class name to your app
    /// target’s Supported Intents in the Project Editor. Then set the Authentication
    /// level to Restricted While Locked or Protected Data Unavailable.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Intents
    #[serde(
        rename = "INIntentsRestrictedWhileProtectedDataUnavailable",
        skip_serializing_if = "Option::is_none"
    )]
    pub intents_restricted_while_protected_data_unavailable: Option<Vec<String>>,
    /// Types of media supported by your app’s media-playing intents.
    ///
    /// Specify one or more media categories to allow Siri to invoke your app’s intent
    /// handling when a user asks to play media. Use INMediaCategoryGeneral for media
    /// that doesn’t fit into any of the other categories, like white noise or sound
    /// effects.
    ///
    /// To specify this information in Xcode, add INPlayMediaIntent to your app’s list of
    /// Supported Intents. Then select the relevant media types in the list that
    /// appears.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Intents
    #[serde(
        rename = "INSupportedMediaCategories",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub supported_media_categories: Option<Vec<SupportedMediaCategories>>,
}

/// Supported Media Categories
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SupportedMediaCategories {
    /// Audiobooks
    #[serde(rename = "INMediaCategoryAudiobooks")]
    INMediaCategoryAudiobooks,
    /// Music
    #[serde(rename = "INMediaCategoryMusic")]
    INMediaCategoryMusic,
    /// General
    #[serde(rename = "INMediaCategoryGeneral")]
    INMediaCategoryGeneral,
    /// Podcasts
    #[serde(rename = "INMediaCategoryPodcasts")]
    INMediaCategoryPodcasts,
    /// Radio
    #[serde(rename = "INMediaCategoryRadio")]
    INMediaCategoryRadio,
}

/// Maps
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Maps {
    /// The modes of transportation for which the app is capable of giving directions.
    ///
    /// ## Availability
    /// * iOS 6.0+
    ///
    /// ## Framework
    /// * Intents
    #[serde(
        rename = "MKDirectionsApplicationSupportedModes",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub directions_application_supported_modes: Option<Vec<DirectionsApplicationSupportedModes>>,
}

/// Directions Application Supported Modes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DirectionsApplicationSupportedModes {
    #[serde(rename = "MKDirectionsModePlane")]
    MKDirectionsModePlane,
    #[serde(rename = "MKDirectionsModeBike")]
    MKDirectionsModeBike,
    #[serde(rename = "MKDirectionsModeBus")]
    MKDirectionsModeBus,
    #[serde(rename = "MKDirectionsModeCar")]
    MKDirectionsModeCar,
    #[serde(rename = "MKDirectionsModeFerry")]
    MKDirectionsModeFerry,
    #[serde(rename = "MKDirectionsModePedestrian")]
    MKDirectionsModePedestrian,
    #[serde(rename = "MKDirectionsModeRideShare")]
    MKDirectionsModeRideShare,
    #[serde(rename = "MKDirectionsModeStreetCar")]
    MKDirectionsModeStreetCar,
    #[serde(rename = "MKDirectionsModeSubway")]
    MKDirectionsModeSubway,
    #[serde(rename = "MKDirectionsModeTaxi")]
    MKDirectionsModeTaxi,
    #[serde(rename = "MKDirectionsModeTrain")]
    MKDirectionsModeTrain,
    #[serde(rename = "MKDirectionsModeOther")]
    MKDirectionsModeOther,
}

/// NFC
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct NfcAppServices {
    /// A list of FeliCa system codes that the app supports.
    ///
    /// Each system code must be a discrete value. The wild card value (0xFF) isn't
    /// allowed.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * Core NFC
    #[serde(
        rename = "com.apple.developer.nfc.readersession.felica.systemcodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub nfc_readersession_felica_systemcodes: Option<Vec<String>>,
    /// A list of application identifiers that the app supports.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * Core NFC
    #[serde(
        rename = "com.apple.developer.nfc.readersession.iso7816.select-identifiers",
        skip_serializing_if = "Option::is_none"
    )]
    pub nfc_readersession_iso7816_select_identifiers: Option<Vec<String>>,
}

/// Authentication
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Authentication {
    /// A Boolean value that indicates the system shouldn’t show security recommendation
    /// prompts when users sign in using the app.
    ///
    /// Each system code must be a discrete value. The wild card value (0xFF) isn't
    /// allowed.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(
        rename = "ASAccountAuthenticationModificationOptOutOfSecurityPromptsOnSignIn",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_authentication_modification_opt_out_of_security_prompts_on_sign_in: Option<bool>,
    /// A collection of keys that a browser app uses to declare its ability to handle
    /// authentication requests from other apps.
    ///
    /// Add a dictionary for this key to your app’s Information Property List if your app
    /// is a web browser and it supports web authentication. In the dictionary, include
    /// the keys IsSupported and EphemeralBrowserSessionIsSupported to indicate your
    /// browser app’s capabilities. For more information, see Supporting Single
    /// Sign-On in a Web Browser App.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(
        rename = "ASWebAuthenticationSessionWebBrowserSupportCapabilities",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_authentication_session_web_browser_support_capabilities:
        Option<WebAuthenticationSession>,
}

/// Web Authentication Session
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct WebAuthenticationSession {
    /// A Boolean that indicates whether the app acts as a browser that supports
    /// authentication sessions.
    ///
    /// Set the corresponding value to YES to indicate that your browser app can handle
    /// authentication requests that other apps generate with ASWebAuthenticationSession.
    /// For details, see Supporting Single Sign-On in a Web Browser App.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(rename = "IsSupported", skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    /// A Boolean that indicates whether the app supports ephemeral browsing when
    /// conducting authentication sessions.
    ///
    /// Set the corresponding value to YES to indicate that your browser app, when
    /// handling authentication requests, offers ephemeral browsing.
    ///
    /// If you don’t provide the key, or if you set its value to NO and an app tries to
    /// conduct an ephemeral authentication session, the system warns the user.
    /// If do you declare support by setting the value to YES, be sure to respect the
    /// shouldUseEphemeralSession property on any incoming authentication requests, as
    /// described in Supporting Single Sign-On in a Web Browser App.
    ///
    /// ### Note
    /// It’s strongly recommended that your web browser support ephemeral sessions.
    /// Apps can specifically request this kind of session, and it’s important to honor
    /// the request.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Authentication Services
    #[serde(
        rename = "EphemeralBrowserSessionIsSupported",
        skip_serializing_if = "Option::is_none"
    )]
    pub ephemeral_browser_session_is_supported: Option<bool>,
}

/// External Accessories
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ExternalAccessories {
    /// The protocols that the app uses to communicate with external accessory hardware.
    ///
    /// Add this key to your app’s Info.plist file, and set the value to the names of the
    /// hardware protocols your app supports. You format protocol names as reverse-DNS
    /// strings. For example, the string "com.apple.myProtocol" might represent a
    /// custom protocol that Apple defines. Manufacturers can define custom protocols
    /// for their accessories or work with other manufacturers and organizations to define
    /// standard protocols for different accessory types.
    ///
    /// ## Availability
    /// * iOS 3.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UISupportedExternalAccessoryProtocols",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_external_accessory_protocols: Option<Vec<String>>,
}

/// Service Management
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ServiceManagement {
    /// The Service Management clients authorized to add and remove tools.
    ///
    /// ## Availability
    /// * iOS 12.1+
    /// * macOS 10.6+
    /// * tvOS 12.1+
    /// * watchOS 5.1+
    ///
    /// ## Framework
    /// * Service Management
    #[serde(
        rename = "SMAuthorizedClients",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorized_clients: Option<Vec<String>>,
    /// The Service Management tools owned by the app.
    ///
    /// ## Availability
    /// * iOS 12.1+
    /// * macOS 10.6+
    /// * tvOS 12.1+
    /// * watchOS 5.1+
    ///
    /// ## Framework
    /// * Service Management
    #[serde(
        rename = "SMPrivilegedExecutables",
        skip_serializing_if = "Option::is_none"
    )]
    pub privileged_executables: Option<DefaultDictionary>,
}

/// Interprocess Communication
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct InterprocessCommunication {
    /// ## Availability
    /// * iOS 6.0+
    /// * macOS 10.8+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(rename = "XPCService", skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

/// Service
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Service {
    #[serde(
        rename = "EnvironmentVariables",
        skip_serializing_if = "Option::is_none"
    )]
    pub environment_variables: Option<DefaultDictionary>,
    #[serde(
        rename = "JoinExistingSession",
        skip_serializing_if = "Option::is_none"
    )]
    pub join_existing_session: Option<bool>,
    #[serde(rename = "RunLoopType", skip_serializing_if = "Option::is_none")]
    pub run_loop_type: Option<RunLoopType>,
    #[serde(rename = "ServiceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,
}

/// Run Loop Type
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RunLoopType {
    #[serde(rename = "dispatch_main")]
    DispatchMain,
    #[serde(rename = "NSRunLoop")]
    RunLoop,
}

/// Service Type
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ServiceType {
    #[serde(rename = "Application")]
    Application,
}

/// Store
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Store {
    /// An array of dictionaries containing a list of ad network identifiers.
    ///
    /// Apps that display ads and initiate install validation information shared with ad
    /// networks, must include the ad network identifiers in this key.
    ///
    /// Each dictionary contains one SKAdNetworkIdentifier.
    /// Provide one dictionary for each ad network with which you work.
    ///
    /// ### Important
    /// Ad network identifiers are case-sensitive, and are in lowercase.
    ///
    /// ## Availability
    /// * iOS 11.3+
    ///
    /// ## Framework
    /// * StoreKit
    #[serde(rename = "SKAdNetworkItems", skip_serializing_if = "Option::is_none")]
    pub ad_network_items: Option<Vec<AdNetworkItems>>,
}

/// Ad Network Items
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AdNetworkItems {
    /// A string that contains an ad network identifier.
    ///
    /// Contact the ad network to learn their ad network identifier.
    ///
    /// Include this key, and the value of the ad network identifier as a string, as a
    /// dictionary in the SKAdNetworkItems key.
    ///
    /// ## Availability
    /// * iOS 11.3+
    ///
    /// ## Framework
    /// * StoreKit
    #[serde(
        rename = "SKAdNetworkIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_network_identifier: Option<String>,
}

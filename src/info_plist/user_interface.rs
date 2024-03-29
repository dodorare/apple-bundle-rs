//! # User Interface.
//!
//! Configure an app's scenes, storyboards, icons, fonts, and other user interface
//! elements.
//!
//! You define the user interface that your app presents during normal operation with a
//! combination of code and storyboards. However, the system needs to know a few things
//! about your app’s user interface before execution begins. For example,
//! on some platforms, you have to specify what device orientations your app supports and
//! what the system should display while your app launches. You add keys to your app’s
//! Information Property List file to control certain aspects of its user interface.
//!
//! ## Framework
//! * Bundle Resources

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};

/// Main User Interface
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct MainUserInterface {
    /// The information about the app's scene-based life-cycle support.
    ///
    /// The presence of this key indicates that the app supports scenes and does not
    /// use an app delegate object to manage transitions to and from the foreground or
    /// background.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIApplicationSceneManifest",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_scene_manifest: Option<ApplicationSceneManifest>,
    /// The name of an app's storyboard resource file.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "NSMainStoryboardFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_storyboard_resource_file_base_name: Option<String>,
    /// The name of the app’s main storyboard file.
    ///
    /// ## Availability
    /// * iOS 5.0+
    /// * tvOS 9.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIMainStoryboardFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_storyboard_file_base_name: Option<String>,
    /// The name of an app’s main user interface file.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * macOS 10.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "NSMainNibFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_nib_file_base_name: Option<String>,
    /// A Boolean value indicating whether the app is an agent app that runs in the
    /// background and doesn't appear in the Dock.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename = "LSUIElement",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_is_agent: Option<bool>,
}

/// Launch Interface
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchInterface {
    /// The user interface to show while an app launches.
    ///
    /// You use this key to define the launch screen that the system displays while your
    /// app launches. If you need to provide different launch screens in response to
    /// being launched by different URL schemes, use UILaunchScreens instead.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UILaunchScreen",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_screen: Option<LaunchScreen>,
    /// The user interfaces to show while an app launches in response to different URL
    /// schemes.
    ///
    /// You use this key if your app supports launching in response to one or more URL
    /// schemes, and if you want to provide different launch screens for different
    /// launch triggers. If you need only one launch screen, use UILaunchScreen
    /// instead.
    ///
    /// To define launch screens, create an array of dictionaries, each similar to the one
    /// you might provide for UILaunchScreen, but with an added
    /// UILaunchScreenIdentifier key that uniquely identifies the screen. Store the
    /// array as the value for the UILaunchScreenDefinitions key.
    ///
    /// To map from URL schemes to a launch screens, create a dictionary of schemes and
    /// identifiers, and store it as the value for the UIURLToLaunchScreenAssociations
    /// key. Additionally, indicate a default launch screen by setting a value for the
    /// UIDefaultLaunchScreen key.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UILaunchScreens",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_screens: Option<LaunchScreens>,
    /// The filename of the storyboard from which to generate the app’s launch image.
    ///
    /// Specify the name of the storyboard file without the filename extension. For
    /// example, if the filename of your storyboard is LaunchScreen.storyboard,
    /// specify "LaunchScreen" as the value for this key.
    ///
    /// If you prefer to configure your app’s launch screen without storyboards, use
    /// UILaunchScreen instead.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UILaunchStoryboardName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_name: Option<String>,
    /// The launch storyboards.
    ///
    /// ## Availability
    /// * iOS 9.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UILaunchStoryboards",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboards: Option<LaunchStoryboards>,
    /// The initial user-interface mode for the app.
    ///
    /// Possible Values: 0, 1, 2, 3, 4
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename = "LSUIPresentationMode",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub presentation_mode: Option<u8>,
}

/// Icons
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Icons {
    /// Information about all of the icons used by the app.
    ///
    /// ## Availability
    /// * macOS 10.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename = "CFBundleIcons",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_icons: Option<BundleIcons>,
    /// The names of the bundle’s icon image files.
    ///
    /// ## Availability
    /// * iOS 3.2+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename = "CFBundleIconFiles",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_icon_files: Option<Vec<String>>,
    /// The file containing the bundle's icon.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * macOS 10.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename = "CFBundleIconFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_icon_file: Option<String>,
    /// The name of the asset that represents the app icon.
    ///
    /// ## Availability
    /// * macOS 10.13+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename = "CFBundleIconName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_icon_name: Option<String>,
    /// A Boolean value indicating whether the app’s icon already contains a shine effect.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIPrerenderedIcon",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prerendered_icon: Option<bool>,
}

/// Orientation
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Orientation {
    /// The initial orientation of the app’s user interface.
    ///
    /// ## Availability
    /// * iOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIInterfaceOrientation",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub interface_orientation: Option<InterfaceOrientation>,
    /// The initial orientation of the app’s user interface.
    ///
    /// ## Availability
    /// * iOS 3.2+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UISupportedInterfaceOrientations",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_vec_enum_option"
    )]
    pub supported_interface_orientations: Option<Vec<InterfaceOrientation>>,
}

/// Styling
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Styling {
    /// The user interface style for the app.
    ///
    /// ## Availability
    /// * iOS 13.0+
    /// * tvOS 10.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIUserInterfaceStyle",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub user_interface_style: Option<UserInterfaceStyle>,
    /// A Boolean value indicating whether Core Animation layers use antialiasing when
    /// drawing a layer that's not aligned to pixel boundaries.
    ///
    /// ## Availability
    /// * iOS 3.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIViewEdgeAntialiasing",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_edge_antialiasing: Option<bool>,
    /// The app’s white point adaptivity style, enabled on devices with True Tone
    /// displays.
    ///
    /// ## Availability
    /// * iOS 9.3+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIWhitePointAdaptivityStyle",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub white_point_adaptivity_style: Option<WhitePointAdaptivityStyle>,
    /// A Boolean value indicating whether Core Animation sublayers inherit the opacity of
    /// their superlayer.
    ///
    /// ## Availability
    /// * iOS 3.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIViewGroupOpacity",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_group_opacity: Option<bool>,
    /// A Boolean value indicating whether the app requires fullscreen or not.
    ///
    /// ## Availability
    /// * iOS 9.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIRequiresFullScreen",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_full_screen: Option<bool>,
    /// The name of a color in an asset catalog to use for a target’s global accent color.
    ///
    /// This Info.plist value controls the global tint color (iOS and watchOS) or accent
    /// color (macOS) for the target. When set in a widget extension, the widget
    /// configuration user interface uses this color as the tint color while editing a
    /// widget.
    ///
    /// While you can set this directly in your Info.plist, the recommended approach is to
    /// use the Global Accent Color Name build setting (in the Asset Catalog Compiler
    /// - Options section) of the target. Set the value of the build setting to the
    /// name of the Color Set in the asset catalog. Xcode automatically sets
    /// NSAccentColorName to the appropriate value in the Info.plist file when
    /// building your project.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    /// * tvOS 14.0+
    /// * watchOS 7.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "NSAccentColorName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub accent_color_name: Option<String>,
    /// The name of a color in an asset catalog to use for a widget’s configuration
    /// interface.
    ///
    /// This Info.plist value controls the background color shown in the widget
    /// configuration interface while editing a widget.
    ///
    /// While you can set this directly in your Info.plist, the recommended approach is to
    /// use the Widget Background Color Name build setting (in the Asset Catalog
    /// Compiler - Options section) of the widget extension target. Set the value
    /// of the build setting to the name of the Color Set in the asset catalog. Xcode
    /// automatically sets NSWidgetBackgroundColorName to the appropriate value in the
    /// Info.plist file when building your project.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * WidgetKit
    #[serde(
        rename = "NSWidgetBackgroundColorName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub widget_background_color_name: Option<String>,
}

/// Fonts
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Fonts {
    /// The location of a font file or directory of fonts in the bundle’s Resources
    /// folder.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(
        rename = "ATSApplicationFontsPath",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_fonts_path: Option<String>,
    /// App-specific font files located in the bundle and that the system loads at
    /// runtime.
    ///
    /// ## Availability
    /// * iOS 3.2+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIAppFonts",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_fonts: Option<Vec<String>>,
}

/// Status Bar
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct StatusBar {
    /// A Boolean value indicating whether the status bar is initially hidden when the app
    /// launches.
    ///
    /// ## Availability
    /// * iOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIStatusBarHidden",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_bar_hidden: Option<bool>,
    /// The style of the status bar as the app launches.
    ///
    /// ## Availability
    /// * iOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIStatusBarStyle",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub status_bar_style: Option<StatusBarStyle>,
    /// The status bar tint.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIStatusBarTintParameters",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_bar_tint_parameters: Option<StatusBarTintParameters>,
    /// A Boolean value indicating whether the status bar appearance is based on the style
    /// preferred for the current view controller.
    ///
    /// ## Availability
    /// * iOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIViewControllerBasedStatusBarAppearance",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_controller_based_status_bar_appearance: Option<bool>,
}

/// Preferences
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Preferences {
    /// The name of an image file used to represent a preference pane in the System
    /// Preferences app.
    ///
    /// ## Availability
    /// * macOS 10.1+
    ///
    /// ## Framework
    /// * Preference Panes
    #[serde(
        rename = "NSPrefPaneIconFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pref_pane_icon_file: Option<String>,
    /// The name of a preference pane displayed beneath the preference pane icon in the
    /// System Preferences app.
    ///
    /// ## Availability
    /// * macOS 10.1+
    ///
    /// ## Framework
    /// * Preference Panes
    #[serde(
        rename = "NSPrefPaneIconLabel",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pref_pane_icon_label: Option<String>,
}

/// Graphics
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Graphics {
    /// A Boolean value indicating whether the app supports HDR mode on Apple TV 4K.
    ///
    /// ## Availability
    /// * tvOS 11.2+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIAppSupportsHDR",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_supports_hdr: Option<bool>,
    /// A Boolean value indicating whether the Cocoa app supports high-resolution
    /// displays.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * macOS 10.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "NSHighResolutionCapable",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_resolution_capable: Option<bool>,
    /// A Boolean value indicating whether an OpenGL app may utilize the integrated GPU.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "NSSupportsAutomaticGraphicsSwitching",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_automatic_graphics_switching: Option<bool>,
    /// The preferred system action when an external GPU is connected from the system.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * Metal
    #[serde(
        rename = "GPUEjectPolicy",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub gpu_eject_policy: Option<GpuEjectPolicy>,
    /// The app's preference for whether it wants to use external graphics processors.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * Metal
    #[serde(
        rename = "GPUSelectionPolicy",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub gpu_selection_policy: Option<GpuSelectionPolicy>,
}

/// Quick Look
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct QuickLook {
    /// A Boolean value indicating whether a Quick Look app's generator can be run in
    /// threads other than the main thread.
    ///
    /// ## Availability
    /// * iOS 4.0+
    /// * macOS 10.5+
    ///
    /// ## Framework
    /// * QuickLook
    #[serde(
        rename = "QLNeedsToBeRunInMainThread",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub needs_to_be_run_in_main_thread: Option<bool>,
    /// A hint at the height, in points, of a Quick Look app's previews.
    ///
    /// ## Availability
    /// * iOS 4.0+
    /// * macOS 10.5+
    ///
    /// ## Framework
    /// * QuickLook
    #[serde(
        rename = "QLPreviewHeight",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_height: Option<f32>,
    /// A hint at the width, in points, of a Quick Look app's previews.
    ///
    /// ## Availability
    /// * iOS 4.0+
    /// * macOS 10.5+
    ///
    /// ## Framework
    /// * QuickLook
    #[serde(
        rename = "QLPreviewWidth",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_width: Option<f32>,
    /// A Boolean value indicating whether a Quick Look app's generator can handle
    /// concurrent thumbnail and preview requests.
    ///
    /// ## Availability
    /// * iOS 4.0+
    /// * macOS 10.5+
    ///
    /// ## Framework
    /// * QuickLook
    #[serde(
        rename = "QLSupportsConcurrentRequests",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_concurrent_requests: Option<bool>,
    /// The minimum size, in points, along one dimension of thumbnails for a Quick Look
    /// app's generator.
    ///
    /// ## Availability
    /// * iOS 4.0+
    /// * macOS 10.5+
    ///
    /// ## Framework
    /// * QuickLook
    #[serde(
        rename = "QLThumbnailMinimumSize",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_minimum_size: Option<f32>,
}

/// Deprecated Keys
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct DeprecatedKeys {
    /// A dictionary containing information about launch images.
    ///
    /// ## Availability
    /// * iOS 7.0–13.0
    /// * tvOS 9.0–13.0
    ///
    /// ## Framework
    /// * UIKit
    #[deprecated(
        since = "iOS 7.0-13.0, tvOS 9.0-13.0",
        note = "UILaunchImages has been deprecated; use Xcode launch storyboards instead."
    )]
    #[serde(
        rename = "UILaunchImages",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_images: Option<Vec<LaunchImage>>,
}

/// Default Dictionary
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchImage {
    pub default: String,
}

/// GPU Eject Policy
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum GpuEjectPolicy {
    /// Set this value to allow macOS to quit and relaunch your app with another GPU.
    /// Your app can implement the application(_:willEncodeRestorableState:) method to
    /// save any state before it quits, and it can implement the
    /// application(_:didDecodeRestorableState:) method to restore any saved state
    /// after it relaunches.
    #[serde(rename = "relaunch")]
    Relaunch,
    /// Set this value to manually respond to the safe disconnect request. Your app must
    /// register and respond to the removalRequested notification posted by Metal.
    /// macOS waits for your app to remove all references to the external GPU before
    /// notifying the user that it's safe to disconnect the GPU.
    #[serde(rename = "wait")]
    Wait,
    /// Set this value to allow macOS to force your app to quit.
    #[serde(rename = "kill")]
    Kill,
    /// Tells the system to ignore the disconnect message. Don’t use this key in new macOS
    /// apps.
    #[serde(rename = "ignore")]
    Ignore,
}

/// GPU Selection Policy
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum GpuSelectionPolicy {
    /// Metal tries to avoid creating contexts on external GPUs. For legacy OpenGL apps,
    /// OpenGL also avoids creating contexts using external GPUs. Set this option only
    /// if your app doesn't support external GPU event handling.
    #[serde(rename = "avoidRemovable")]
    AvoidRemovable,
    /// If external GPUs are visible to the system, Metal prefers them over other GPUs.
    /// Similarly, for legacy OpenGL apps, OpenGL also prefers to create contexts on
    /// the external GPU.
    #[serde(rename = "preferRemovable")]
    PreferRemovable,
}

/// NavigationBar
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct StatusBarTintParameters {
    /// The initial navigation bar’s style and translucency.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UINavigationBar",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub navigation_bar: Option<NavigationBar>,
}

/// Navigation Bar
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct NavigationBar {
    #[serde(rename = "BackgroundImage")]
    pub background_image: String,
    #[serde(rename = "Style")]
    pub style: BarStyle,
    #[serde(rename = "Translucent")]
    pub translucent: bool,
    /// The tint color to apply to the background of the navigation bar.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "TintColor",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tint_color: Option<TintColor>,
}

/// Bar Style
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum BarStyle {
    #[serde(rename = "UIBarStyleDefault")]
    Default,
    #[serde(rename = "UIBarStyleBlack")]
    Black,
}

impl Default for BarStyle {
    fn default() -> Self {
        Self::Default
    }
}

/// Tint Color
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TintColor {
    #[serde(rename = "Blue")]
    pub blue: f32,
    #[serde(rename = "Green")]
    pub green: f32,
    #[serde(rename = "Red")]
    pub red: f32,
}

/// Status Bar Style
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum StatusBarStyle {
    #[serde(rename = "UIStatusBarStyleDefault")]
    Default,
    #[serde(rename = "UIStatusBarStyleBlackTranslucent")]
    BlackTranslucent,
    #[serde(rename = "UIStatusBarStyleBlackOpaque")]
    BlackOpaque,
}

/// White Point Adaptivity Style
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum WhitePointAdaptivityStyle {
    #[serde(rename = "UIWhitePointAdaptivityStyleStandard")]
    Standard,
    #[serde(rename = "UIWhitePointAdaptivityStyleReading")]
    Reading,
    #[serde(rename = "UIWhitePointAdaptivityStylePhoto")]
    Photo,
    #[serde(rename = "UIWhitePointAdaptivityStyleVideo")]
    Video,
    #[serde(rename = "UIWhitePointAdaptivityStyleGame")]
    Game,
}

/// User Interface Style
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum UserInterfaceStyle {
    /// Set this value to adopt the systemwide user interface style, and observe any
    /// changes to that style. This is the default value, and provides the same
    /// functionality as if the key weren’t explicitly set.
    Automatic,
    /// Set this value to force the light user interface style, even when the systemwide
    /// style is set to dark. Your app will ignore any changes to the systemwide
    /// style.
    Light,
    /// Set this value to force the dark user interface style, even when the systemwide
    /// style is set to light. Your app will ignore any changes to the systemwide
    /// style.
    Dark,
}

/// Interface Orientation
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum InterfaceOrientation {
    /// The app supports the display in portrait mode, with the device upright and the
    /// front camera at the top.
    #[serde(rename = "UIInterfaceOrientationPortrait")]
    Portrait,
    /// The app supports the display in portrait mode but is upside down, with the device
    /// upright and the front camera at the bottom. UIViewController ignores this
    /// option on devices without a Home button.
    #[serde(rename = "UIInterfaceOrientationPortraitUpsideDown")]
    PortraitUpsideDown,
    /// The app supports the display in landscape mode, with the device upright and the
    /// front camera on the left.
    #[serde(rename = "UIInterfaceOrientationLandscapeLeft")]
    LandscapeLeft,
    /// The app supports the display in landscape mode, with the device upright and the
    /// front camera on the right.
    #[serde(rename = "UIInterfaceOrientationLandscapeRight")]
    LandscapeRight,
}

impl FromStr for InterfaceOrientation {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<InterfaceOrientation, Self::Err> {
        match s {
            "portrait" => Ok(InterfaceOrientation::Portrait),
            "portrait-upside-down" => Ok(InterfaceOrientation::PortraitUpsideDown),
            "landscape-left" => Ok(InterfaceOrientation::LandscapeLeft),
            "landscape-right" => Ok(InterfaceOrientation::LandscapeRight),
            _ => Err(s.into()),
        }
    }
}

/// Bundle Icons
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct BundleIcons {
    ///
    /// ## Availability
    /// * iOS 5.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename = "CFBundleAlternateIcons",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_alternate_icons: Option<BTreeMap<String, AppIconReferenceName>>,
    /// The primary icon for the Home screen and Settings app, among others.
    ///
    /// ## Availability
    /// * iOS 5.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(rename = "CFBundlePrimaryIcon")]
    pub bundle_primary_icon: BundlePrimaryIcon,
}

/// App Icon Reference Name
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct AppIconReferenceName {
    #[serde(
        rename = "CFBundleIconFiles",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_icon_files: Option<Vec<String>>,
    #[serde(
        rename = "UIPrerenderedIcon",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prerendered_icon: Option<bool>,
}

/// Bundle Primary Icon
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct BundlePrimaryIcon {
    /// The names of a bundle’s icon files.
    ///
    /// ## Availability
    /// * iOS 3.2+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(rename = "CFBundleIconFiles")]
    pub bundle_icon_files: Vec<String>,
    /// The name of a symbol from SF Symbols.
    ///
    /// Action extensions use template images for their icons. To use a symbol from SF
    /// Symbols as the icon, set the value of CFBundleSymbolName to the symbol’s name.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename = "CFBundleSymbolName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_symbol_name: Option<String>,
    /// A Boolean value indicating whether the icon files already incorporate a shine
    /// effect.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(rename = "UIPrerenderedIcon")]
    pub prerendered_icon: bool,
}

/// Launch Screen
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchScreen {
    // Main Interface.
    /// The name of a color to use as the background color on the launch screen.
    ///
    /// Provide a value for this key that’s the name of a color in your asset catalog.
    /// You use the same string for the value that you might use when calling the
    /// init(named:) initializer of UIColor.
    ///
    /// If you don’t set a color, the system uses a default of systemBackground, which
    /// varies according to whether the user has selected the light appearance or Dark
    /// Mode for the device.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIColorName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_name: Option<String>,
    /// The name of an image to display during app launch.
    ///
    /// Provide a value for this key that’s the name of an image in your asset catalog.
    /// You use the same string for the value that you might use when calling the
    /// init(named:) initializer of UIImage. Because the image comes from your asset
    /// catalog, you can use slicing to provide a small image that works on many different
    /// platforms.
    ///
    /// If you don’t specify an image, the display shows the background color, as given by
    /// the UIColorName key. The background color may also show through any
    /// transparency in your image.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIImageName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_name: Option<String>,
    /// A Boolean that specifies whether the launch image should respect the safe area
    /// insets.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIImageRespectsSafeAreaInsets",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_respects_safe_area_insets: Option<bool>,
    // Border Elements.
    /// Navigation bar visibility and configuration during launch.
    ///
    /// When you provide a dictionary for this key, the system displays a navigation bar
    /// during launch. You can optionally set the dictionary’s UIImageName key to
    /// define a custom image for the navigation bar.
    ///
    /// Omit this key if you don’t want to display a navigation bar during launch.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UINavigationBar",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub navigation_bar: Option<Bar>,
    /// Tab bar visibility and configuration during launch.
    ///
    /// When you provide a dictionary for this key, the system displays a tab bar during
    /// launch. You can optionally set the dictionary’s UIImageName key to define a
    /// custom image for the tab bar.
    ///
    /// Omit this key if you don’t want to display a tab bar during launch.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UITabBar",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tab_bar: Option<Bar>,
    /// When you provide a dictionary for this key, the system displays a toolbar during
    /// launch. You can optionally set the dictionary’s UIImageName key to define a
    /// custom image for the toolbar.
    ///
    /// Omit this key if you don’t want to display a toolbar during launch.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIToolbar",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub toolbar: Option<Bar>,
}

/// Application Scene Manifest
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Bar {
    /// A custom image that replaces the navigation/tab/tool bar during launch.
    ///
    /// Provide a value for this key that’s the name of an image in your asset catalog.
    /// You use the same string for the value that you might use when calling the
    /// init(named:) initializer of UIImage.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIImageName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_name: Option<String>,
}

/// Launch Screens
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchScreens {
    // Launch Screen Definitions.
    /// A collection of launch screen configuration dictionaries.
    ///
    /// Each dictionary in the array resembles the one you might define for the
    /// UILaunchScreen key, with the addition of a UILaunchScreenIdentifier key that
    /// provides a unique identifier for the dictionary. You use that identifier when
    /// associating to the dictionary with a URL scheme in the
    /// UIURLToLaunchScreenAssociations array, or to indicate it as the default launch
    /// screen with the UIDefaultLaunchScreen key.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UILaunchScreenDefinitions",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_screen_definitions: Option<LaunchScreenDefinitions>,
    // Associations.
    /// The mapping of URL schemes to launch screen configurations.
    ///
    /// Set the keys of this dictionary to the URL schemes that your app supports.
    /// Provide a value for each key that is the identifier, stored in the
    /// UILaunchScreenIdentifier key, of one of the launch screen definitions in your
    /// UILaunchScreenDefinitions array.
    ///
    /// Any Key - A URL scheme. Set one of the configuration identifiers as the value.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIURLToLaunchScreenAssociations",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_to_launch_screen_associations: Option<BTreeMap<String, String>>,
    /// The default launch screen configuration.
    ///
    /// Provide the identifier, stored in the UILaunchScreenIdentifier key, of one of the
    /// launch screen definitions in your UILaunchScreenDefinitions array. The system
    /// displays the named launch screen when launching your app in response to a URL
    /// scheme that you don’t enumerate in the UIURLToLaunchStoryboardAssociations
    /// dictionary, or when the user launches your app directly.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(
        rename = "UIDefaultLaunchScreen",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_launch_screen: Option<String>,
}

/// Launch Screen Definitions
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchScreenDefinitions {
    /// A unique name for the launch screen configuration.
    ///
    /// You can choose any name you want for the identifier, as long as it’s unique among
    /// all your app’s configuration identifiers. Use this value to refer to the
    /// configuration when storing a URL to configuration mapping as the value for the
    /// UIURLToLaunchScreenAssociations key, or when specifying a default configuration
    /// with the UIDefaultLaunchScreen key.
    #[serde(
        rename = "UIColorName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_name: Option<String>,
    /// Launch Storyboards.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SwiftUI
    #[serde(flatten)]
    pub launch_screen: LaunchScreen,
}

/// Launch Storyboards
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchStoryboards {
    #[serde(
        rename = "UIDefaultLaunchStoryboard",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_launch_storyboard: Option<String>,
    #[serde(
        rename = "UILaunchStoryboardDefinitions",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_definitions: Option<Vec<LaunchStoryboardDefinition>>,
    #[serde(
        rename = "UIURLToLaunchStoryboardAssociations",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_to_launch_storyboard_associations: Option<BTreeMap<String, String>>,
}

/// Launch Storyboard Definition
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct LaunchStoryboardDefinition {
    #[serde(
        rename = "UILaunchStoryboardFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_file: Option<String>,
    #[serde(
        rename = "UILaunchStoryboardIdentifier",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_storyboard_identifier: Option<String>,
}

/// Application Scene Manifest
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct ApplicationSceneManifest {
    /// A Boolean value indicating whether the app supports two or more scenes
    /// simultaneously.
    ///
    /// If your app supports multiple scenes, set the value of this key to true.
    /// If you set the value to false, UIKit never creates more than one scene for your
    /// app.
    ///
    /// Setting this key to true has implications for your code. An app that supports
    /// multiple scenes must coordinate operations to prevent scenes from interfering
    /// with each other. For example, if two scenes access the same shared resource,
    /// you must synchronize access to that resource using a serial dispatch queue or
    /// some other mechanism. Failure to do so may lead to corrupted data or
    /// unexpected behavior from your app.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UIApplicationSupportsMultipleScenes",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_multiple_windows: Option<bool>,
    /// The default configuration details for UIKit to use when creating new scenes.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        flatten,
        rename = "UISceneConfigurations",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scene_configurations: Option<SceneConfigurations>,
}

/// Scene Configurations
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct SceneConfigurations {
    /// Scenes that you use to display content on the device's main screen and respond to
    /// user interactions.
    ///
    /// Use this key to specify the scene configurations for your app.
    /// Each scene corresponds to one you use for content you display on the device's main
    /// screen. Make your app's default scene the first entry in the array.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        flatten,
        rename = "UIWindowSceneSessionRoleApplication",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_session_role: Option<WindowSceneSessionRole>,
    /// Scenes that you use to display content on an externally connected display.
    ///
    /// Use this key to specify the scene configurations you use when displaying content
    /// on an external display. Make the default scene the first entry in the array.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        flatten,
        rename = "UIWindowSceneSessionRoleExternalDisplay",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_display_session_role: Option<WindowSceneSessionRole>,
}

/// Window Scene Session Role
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct WindowSceneSessionRole {
    /// The app-specific name you use to identify the scene.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UISceneConfigurationName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_name: Option<String>,
    /// The name of the scene class you want UIKit to instantiate.
    ///
    /// Specify UIWindowScene for scenes meant for your app or an external display. Do not
    /// specify UIScene.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UISceneClassName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub class_name: Option<String>,
    /// The name of the app-specific class that you want UIKit to instantiate and use as
    /// the scene delegate object.
    ///
    /// The class you specify for this key must adopt the UISceneDelegate protocol.
    /// If the class you specify for the UISceneClassName key is UIWindowScene,
    /// your class must adopt the UIWindowSceneDelegate protocol.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UISceneDelegateClassName",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delegate_class_name: Option<String>,
    /// The name of the storyboard file containing the scene's initial user interface.
    ///
    /// Specify the name of the storyboard file without the filename extension. For
    /// example, if the filename of your storyboard is Main.storyboard, specify Main
    /// as the value for this key.
    ///
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename = "UISceneStoryboardFile",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub storyboard_name: Option<String>,
}

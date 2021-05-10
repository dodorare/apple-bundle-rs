use crate::serialize_vec_enum_option;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct AppSandbox {
    /// A Boolean value that indicates whether the app may use access control technology to contain damage to the system and user data if an app is compromised.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.app-sandbox"),
        skip_serializing_if = "Option::is_none"
    )]
    pub app_sandbox: Option<bool>,
    /// A Boolean value indicating whether your app may listen for incoming network connections.
    ///
    /// Use this key to allow other computers to initiate network connections to your sandboxed app.
    ///
    /// ### Note
    /// For TCP sockets, the com.apple.security.network.server and com.apple.security.network.client entitlements restrict only the initiation of a network connection, not the flow of data.
    /// Outgoing and incoming connections can both send and receive data.
    ///
    /// For UDP sockets, the network entitlements restrict both initiation and data flow.
    /// For example, an app with only the server entitlement enabled can receive, but not send, data.
    /// Apps using UDP usually require both entitlements.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode, and under Network, select Incoming Connections (Server).
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.network.server"),
        skip_serializing_if = "Option::is_none"
    )]
    pub security_network_server: Option<bool>,
    /// A Boolean value indicating whether your app may open outgoing network connections.
    ///
    /// Use this key to allow your sandboxed app to connect to a server process running on another machine, or on the same machine.
    ///
    /// ### Note
    /// For TCP sockets, the com.apple.security.network.client and com.apple.security.network.server entitlements restrict only the initiation of a network connection, not the flow of data.
    /// Outgoing and incoming connections can both send and receive data.
    ///
    /// For UDP sockets, the network entitlements restrict both initiation and data flow.
    /// For example, an app with only the client entitlement enabled can send, but not receive, data.
    /// Apps using UDP usually require both entitlements.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode, and under Network, select Outgoing Connections (Client).
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.network.client"),
        skip_serializing_if = "Option::is_none"
    )]
    pub security_network_client: Option<bool>,
    /// A Boolean value that indicates whether the app may capture movies and still images using the built-in camera.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened Runtime capability in Xcode, and then select Camera.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.device.camera"),
        skip_serializing_if = "Option::is_none"
    )]
    pub camera: Option<bool>,
    /// A Boolean value that indicates whether the app may use the microphone.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and under Hardware select Audio Input.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.device.microphone"),
        skip_serializing_if = "Option::is_none"
    )]
    pub device_microphone: Option<bool>,
    /// A Boolean value indicating whether your app may interact with USB devices.
    ///
    /// Use this key to allow your sandboxed app to interact with USB devices through USB device access APIs.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode, and under Hardware, select USB.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.device.usb"),
        skip_serializing_if = "Option::is_none"
    )]
    pub device_usb: Option<bool>,
    /// A Boolean value indicating whether your app may print a document.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode, and under Hardware, select Printing.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.print"),
        skip_serializing_if = "Option::is_none"
    )]
    pub print: Option<bool>,
    /// A Boolean value indicating whether your app may interact with Bluetooth devices.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode, and under Hardware, select Bluetooth.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.device.bluetooth"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bluetooth: Option<bool>,
    /// A Boolean value that indicates whether the app may access location information from Location Services.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened Runtime capability in Xcode, and then select Location.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.personal-information.location"),
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the user's calendar.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened Runtime capability in Xcode, and then select Calendar.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.personal-information.calendars"),
        skip_serializing_if = "Option::is_none"
    )]
    pub calendars: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to files the user has selected using an Open or Save dialog.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set User Selected File to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.files.user-selected.read-only"),
        skip_serializing_if = "Option::is_none"
    )]
    pub files_user_selected_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to files the user has selected using an Open or Save dialog.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set User Selected File to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.files.user-selected.read-write"),
        skip_serializing_if = "Option::is_none"
    )]
    pub files_user_selected_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the Downloads folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Downloads Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.files.downloads.read-only"),
        skip_serializing_if = "Option::is_none"
    )]
    pub files_downloads_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the Downloads folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Downloads Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.files.downloads.read-write"),
        skip_serializing_if = "Option::is_none"
    )]
    pub files_downloads_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the Pictures folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Pictures Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.assets.pictures.read-only"),
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_pictures_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the Pictures folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Pictures Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.assets.pictures.read-write"),
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_pictures_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the Music folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Music Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.assets.music.read-only"),
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_music_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the Movies folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Movies Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.assets.movies.read-only"),
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_movies_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the Movies folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode and set Movies Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "com.apple.security.assets.movies.read-write"),
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_movies_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have access to all files.
    ///
    /// ## Availability
    /// * macOS 10.7–10.11
    ///
    /// ## Framework
    /// * Security
    #[deprecated(since = "macOS 10.7–10.11")]
    #[serde(
        rename(serialize = "com.apple.security.files.all"),
        skip_serializing_if = "Option::is_none"
    )]
    pub all_files: Option<bool>,
}

use serde::{Deserialize, Serialize};

/// Security
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Security {
    /// Restrict access to system resources and user data in macOS apps to contain damage
    /// if an app becomes compromised.
    ///
    /// App Sandbox provides protection to system resources and user data by limiting your
    /// app’s access to resources requested through entitlements.
    ///
    /// ### Important
    /// To distribute a macOS app through the Mac App Store, you must enable the App
    /// Sandbox capability.
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "AppSandbox",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_sandbox: Option<AppSandbox>,
    /// Manage security protections and resource access for your macOS apps.
    ///
    /// The Hardened Runtime, along with System Integrity Protection (SIP), protects the
    /// runtime integrity of your software by preventing certain classes of exploits, like
    /// code injection, dynamically linked library (DLL) hijacking, and process memory
    /// space tampering. To enable the Hardened Runtime for your app, navigate in
    /// Xcode to your target’s Signing & Capabilities information and click the + button.
    /// In the window that appears, choose Hardened Runtime.
    ///
    /// The Hardened Runtime doesn’t affect the operation of most apps, but it does
    /// disallow certain less common capabilities, like just-in-time (JIT) compilation.
    /// If your app relies on a capability that the Hardened Runtime restricts, add an
    /// entitlement to disable an individual protection. You add an entitlement by
    /// enabling one of the runtime exceptions or access permissions listed in Xcode.
    /// Make sure to use only the entitlements that are absolutely necessary for your
    /// app’s functionality.
    ///
    /// You add entitlements only to executables.
    /// Shared libraries, frameworks, and in-process plug-ins inherit the entitlements of
    /// their host executable.
    ///
    /// ### Important
    /// To upload a macOS app to be notarized, you must enable the Hardened Runtime
    /// capability. For more information about notarization, see Notarizing macOS
    /// Software Before Distribution.
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "HardenedRuntime",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hardened_runtime: Option<HardenedRuntime>,
    /// A list of identifiers specifying the groups your app belongs to.
    ///
    /// App groups allow multiple apps produced by a single development team to access
    /// shared containers and communicate using interprocess communication (IPC). Apps
    /// may belong to one or more app groups.
    ///
    /// For iOS, format the identifier as follows:
    /// ```swift
    /// group.<group name>
    /// ```
    ///
    /// For macOS:
    /// ```swift
    /// <team identifier>.<group name>
    /// ```
    ///
    /// Apps within an app group share access to a group container. For more information
    /// about container creation, location, and deletion, see
    /// containerURL(forSecurityApplicationGroupIdentifier:).
    ///
    /// Apps within a group can communicate with other members in the group using IPC
    /// mechanisms including Mach IPC, POSIX semaphores and shared memory, and UNIX domain
    /// sockets. In macOS, use app groups to enable IPC communication between two
    /// sandboxed apps, or between a sandboxed app and a non-sandboxed app.
    ///
    /// App groups also act as keychain access groups.
    /// For more information about the relationship between app groups and keychain access
    /// groups, see Sharing Access to Keychain Items Among a Collection of Apps.
    ///
    /// To add this entitlement to your app, enable the App Groups capability in Xcode,
    /// and add the groups your app belongs to.
    ///
    /// ## Availability
    /// * iOS 3.0+
    /// * macOS 10.7+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "com.apple.security.application-groups",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_groups: Option<Vec<String>>,
    /// The identifiers for the keychain groups that the app may share items with.
    ///
    /// To add this entitlement to your app, enable the Keychain Sharing capability in
    /// Xcode.
    ///
    /// ## Availability
    /// * iOS 3.0+
    /// * macOS 10.7+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "keychain-access-groups",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub keychain_access_groups: Option<Vec<String>>,
    /// The level of data protection for sensitive user data when an app accesses it on a
    /// device.
    ///
    /// To add this entitlement to your app, enable the Data Protection capability in
    /// Xcode.
    ///
    /// ## Availability
    /// * iOS 3.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "com.apple.developer.default-data-protection",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub data_protection: Option<DataProtection>,
    /// The environment for an app that uses the App Attest service to validate itself
    ///
    /// To add this entitlement to your app, add the key to your app’s entitlements file
    /// manually, choose the String type, and set the associated value to either
    /// development or production. If you omit the entitlement during development,
    /// your app uses the App Attest sandbox servers by default. You can test your app
    /// during development against the App Attest production servers by setting the
    /// entitlement to production.
    ///
    /// After distributing your app through TestFlight, the App Store, or the Apple
    /// Developer Enterprise Program, your app ignores the entitlement you set and uses
    /// the production environment.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * DeviceCheck
    #[serde(
        rename = "com.apple.developer.devicecheck.appattest-environment",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_enum_option"
    )]
    pub devicecheck_appattest: Option<DeviceCheckAppAttest>,
    /// A Boolean that indicates whether your app has access to smart card slots and smart
    /// cards.
    ///
    /// Add this entitlement to your app with a value of true if you want to use the
    /// TKSmartCardSlotManager class. For an app without the entitlement, the slot
    /// manager’s default value is nil. The system also requires this entitlement for
    /// sandboxed applications that access smart cards using legacy PCSC framework APIs.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * CryptoTokenKit
    #[serde(
        rename = "com.apple.security.smartcard",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_smartcard: Option<bool>,
}

/// App Sandbox
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AppSandbox {
    /// A Boolean value that indicates whether the app may use access control technology
    /// to contain damage to the system and user data if an app is compromised.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.app-sandbox",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_sandbox: Option<bool>,
    /// A Boolean value indicating whether your app may listen for incoming network
    /// connections.
    ///
    /// Use this key to allow other computers to initiate network connections to your
    /// sandboxed app.
    ///
    /// ### Note
    /// For TCP sockets, the com.apple.security.network.server and
    /// com.apple.security.network.client entitlements restrict only the initiation of a
    /// network connection, not the flow of data. Outgoing and incoming connections
    /// can both send and receive data.
    ///
    /// For UDP sockets, the network entitlements restrict both initiation and data flow.
    /// For example, an app with only the server entitlement enabled can receive, but not
    /// send, data. Apps using UDP usually require both entitlements.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode,
    /// and under Network, select Incoming Connections (Server).
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.network.server",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_network_server: Option<bool>,
    /// A Boolean value indicating whether your app may open outgoing network connections.
    ///
    /// Use this key to allow your sandboxed app to connect to a server process running on
    /// another machine, or on the same machine.
    ///
    /// ### Note
    /// For TCP sockets, the com.apple.security.network.client and
    /// com.apple.security.network.server entitlements restrict only the initiation of a
    /// network connection, not the flow of data. Outgoing and incoming connections
    /// can both send and receive data.
    ///
    /// For UDP sockets, the network entitlements restrict both initiation and data flow.
    /// For example, an app with only the client entitlement enabled can send, but not
    /// receive, data. Apps using UDP usually require both entitlements.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode,
    /// and under Network, select Outgoing Connections (Client).
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.network.client",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_network_client: Option<bool>,
    /// A Boolean value that indicates whether the app may capture movies and still images
    /// using the built-in camera.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened
    /// Runtime capability in Xcode, and then select Camera.
    ///
    /// In macOS 10.14 and later, the user must explicitly grant permission for each app
    /// to access cameras. See Requesting Authorization for Media Capture on macOS.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.device.camera",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub camera: Option<bool>,
    /// A Boolean value that indicates whether the app may use the microphone.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and under Hardware select Audio Input.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.device.microphone",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_microphone: Option<bool>,
    /// A Boolean value indicating whether your app may interact with USB devices.
    ///
    /// Use this key to allow your sandboxed app to interact with USB devices through USB
    /// device access APIs.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode,
    /// and under Hardware, select USB.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.device.usb",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_usb: Option<bool>,
    /// A Boolean value indicating whether your app may print a document.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode,
    /// and under Hardware, select Printing.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.print",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub print: Option<bool>,
    /// A Boolean value indicating whether your app may interact with Bluetooth devices.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode,
    /// and under Hardware, select Bluetooth.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.device.bluetooth",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bluetooth: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to
    /// contacts in the user's address book.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and then select Contacts, or enable the Hardened Runtime capability and then
    /// select Address Book.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.addressbook",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_book: Option<bool>,
    /// A Boolean value that indicates whether the app may access location information
    /// from Location Services.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened
    /// Runtime capability in Xcode, and then select Location.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.location",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the
    /// user's calendar.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened
    /// Runtime capability in Xcode, and then select Calendar.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.calendars",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calendars: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to files
    /// the user has selected using an Open or Save dialog.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set User Selected File to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.files.user-selected.read-only",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub files_user_selected_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to files
    /// the user has selected using an Open or Save dialog.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set User Selected File to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.files.user-selected.read-write",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub files_user_selected_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the
    /// Downloads folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Downloads Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.files.downloads.read-only",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub files_downloads_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the
    /// Downloads folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Downloads Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.files.downloads.read-write",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub files_downloads_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the
    /// Pictures folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Pictures Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.assets.pictures.read-only",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_pictures_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the
    /// Pictures folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Pictures Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.assets.pictures.read-write",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_pictures_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the
    /// Music folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Music Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.assets.music.read-only",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_music_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the
    /// Music folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Music Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.assets.music.read-write",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_music_read_write: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-only access to the
    /// Movies folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Movies Folder to Read Only.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.assets.movies.read-only",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub assets_movies_read_only: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the
    /// Movies folder.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and set Movies Folder to Read/Write.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.assets.movies.read-write",
        serialize_with = "crate::serialize_option",
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
    #[deprecated(since = "macOS 10.7-10.11")]
    #[serde(
        rename = "com.apple.security.files.all",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_files: Option<bool>,
}

/// Hardened Runtime
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct HardenedRuntime {
    /// A Boolean value that indicates whether the app may create writable and executable
    /// memory using the MAP_JIT flag.
    ///
    /// You can create memory that’s both writable and executable by passing the MAP_JIT
    /// flag to the mmap() system function. The Hardened Runtime disallows this by
    /// default, because it creates a security risk. However, some apps and system
    /// frameworks rely on this functionality, typically for performance reasons.
    /// Examples include:
    ///
    /// * The fast-path of the JavaScriptCore framework
    /// * Certain Python frameworks
    /// * Perl-compatible regular expressions (PCRE)
    /// * An app that creates a dynamically-compiled, proprietary macro language
    ///
    /// Without the Allow Execution of JIT-compiled Code Entitlement, frameworks that rely
    /// on just-in-time (JIT) compilation may fall back to an interpreter. Other code
    /// using JIT compilation may crash or behave in unexpected ways.
    ///
    /// Digital rights management (DRM) solutions that currently use unsigned executable
    /// memory should instead change to using the MAP_JIT flag and the entitlement.
    ///
    /// To add the entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Runtime Exceptions, select Allow Execution of
    /// JIT-compiled Code.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.cs.allow-jit",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_execution_of_jit_compiled_code: Option<bool>,
    /// A Boolean value that indicates whether the app may create writable and executable
    /// memory without the restrictions imposed by using the MAP_JIT flag.
    ///
    /// In rare cases, an app might need to override or patch C code, use the
    /// long-deprecated NSCreateObjectFileImageFromMemory (which is fundamentally
    /// insecure), or use the DVDPlayback framework. Add the Allow Unsigned Executable
    /// Memory Entitlement to enable these use cases. Otherwise, the app might crash or
    /// behave in unexpected ways.
    ///
    /// ### Important
    /// Including this entitlement exposes your app to common vulnerabilities in
    /// memory-unsafe code languages. Carefully consider whether your app needs this
    /// exception.
    ///
    /// To add the entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Runtime Exceptions, select Allow Unsigned Executable
    /// Memory.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.cs.allow-unsigned-executable-memory",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_unsigned_executable_memory: Option<bool>,
    /// A Boolean value that indicates whether the app may be affected by dynamic linker
    /// environment variables, which you can use to inject code into your app’s process.
    ///
    /// If your app relies on dynamic linker variables to modify its behavior at runtime,
    /// add the Allow DYLD Environment Variables Entitlement to your app. This causes
    /// the macOS dynamic linker (dyld) to read from environment variables that begin with
    /// DLYD_. See the dyld man page for a list of these variables.
    ///
    /// Injecting libraries or changing search paths with this feature may still require
    /// another entitlement. For example, you also need the Disable Library Validation
    /// Entitlement if an injected library isn’t signed with the expected team ID.
    ///
    /// To add the entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Runtime Exceptions, select Allow DYLD Environment
    /// Variables.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.cs.allow-dyld-environment-variables",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_dyld_environment_variables: Option<bool>,
    /// A Boolean value that indicates whether the app loads arbitrary plug-ins or
    /// frameworks, without requiring code signing.
    ///
    /// The Hardened Runtime enables library validation by default.
    /// This security-hardening feature prevents a program from loading frameworks,
    /// plug-ins, or libraries unless they’re either signed by Apple or signed with the
    /// same Team ID as the main executable. The macOS dynamic linker (dyld) provides
    /// a detailed error message when the system prevents code from loading due to library
    /// validation. Use the Disable Library Validation Entitlement if your program
    /// loads plug-ins that are signed by other third-party developers.
    ///
    /// To add this entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Runtime Exceptions, select Disable Library Validation.
    ///
    /// ### Important
    /// Because library validation is such an important security-hardening feature,
    /// Gatekeeper runs extra security checks on programs that have it disabled.
    /// If your program is blocked by Gatekeeper, check whether you’ve unnecessarily
    /// disabled library validation.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.cs.disable-library-validation",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_library_validation: Option<bool>,
    /// A Boolean value that indicates whether to disable all code signing protections
    /// while launching an app, and during its execution.
    ///
    /// The system causes an app that attempts to directly modify sections of its own
    /// executable files on disk to forcefully exit. Use the Disable Executable Memory
    /// Protection Entitlement to enable this kind of unsafe software update.
    /// Even with this entitlement, however, updates that modify some files but not others
    /// may cause unexpected app state. Ensure that you perform updates atomically,
    /// with the final app bundle swapped out after app exit.
    ///
    /// The entitlement effectively encompasses the behavior provided by the Allow
    /// Unsigned Executable Memory Entitlement, but not the Disable Library Validation
    /// Entitlement.
    ///
    /// ### Warning
    /// The Disable Executable Memory Protection Entitlement is an extreme entitlement
    /// that removes a fundamental security protection from your app, making it possible
    /// for an attacker to rewrite your app’s executable code without detection.
    /// Prefer narrower entitlements if possible.
    ///
    /// To add this entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Runtime Exceptions, select Disable Executable Memory
    /// Protection.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.cs.disable-executable-page-protection",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_executable_memory_protection: Option<bool>,
    /// A Boolean value that indicates whether the app is a debugger and may attach to
    /// other processes or get task ports.
    ///
    /// Apps with the Debugging Tool Entitlement can call task_for_pid() to retrieve a
    /// valid task port for unsigned and third-party apps with the Get Task Allow
    /// entitlement set to true. However, even with the debugging tool entitlement, a
    /// debugger can’t get the task ports of processes that don’t have the Get Task Allow
    /// entitlement, and that are therefore protected by System Integrity Protection.
    /// See the man page for taskgated(8) for more information about getting task ports.
    ///
    /// Xcode automatically adds the Get Task Allow entitlement to apps that you build for
    /// debugging, while removing the entitlement before App Store submission.
    /// This enables Xcode itself to attach to and debug your app during development.
    ///
    /// When a non-root user runs an app with the debugging tool entitlement, the system
    /// presents an authorization dialog asking for a system administrator’s credentials.
    /// If authorization succeeds, the debugger receives a 10-hour session before
    /// authorization expires.
    ///
    /// To add this entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Runtime Exceptions, select Debugging Tool.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.cs.debugger",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub debugging_tool: Option<bool>,
    /// A Boolean value that indicates whether the app may record audio using the built-in
    /// microphone and access audio input using Core Audio.
    ///
    /// To add this entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Resource Access, select Audio Input.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.device.audio-input",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub audioinput: Option<bool>,
    /// A Boolean value that indicates whether the app may capture movies and still images
    /// using the built-in camera.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened
    /// Runtime capability in Xcode, and then select Camera.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.device.camera",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub camera: Option<bool>,
    /// A Boolean value that indicates whether the app may access location information
    /// from Location Services.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened
    /// Runtime capability in Xcode, and then select Location.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.location",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to
    /// contacts in the user's address book.
    ///
    /// To add this entitlement to your app, enable the App Sandbox capability in Xcode
    /// and then select Contacts, or enable the Hardened Runtime capability and then
    /// select Address Book.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.addressbook",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_book: Option<bool>,
    /// A Boolean value that indicates whether the app may have read-write access to the
    /// user's calendar.
    ///
    /// To add this entitlement to your app, first enable the App Sandbox or Hardened
    /// Runtime capability in Xcode, and then select Calendar.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.calendars",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calendars: Option<bool>,
    /// A Boolean value that indicates whether the app has read-write access to the user's
    /// Photos library.
    ///
    /// To add this entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode. Then, under Resource Access, select Photos Library.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.personal-information.photos-library",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub photos_library: Option<bool>,
    /// A Boolean value that indicates whether the app may prompt the user for permission
    /// to send Apple events to other apps.
    ///
    /// Your app doesn’t need the Apple Events Entitlement if it only sends Apple events
    /// to itself or to other processes signed with the same team ID.
    ///
    /// To add this entitlement to your app, first enable the Hardened Runtime capability
    /// in Xcode, and then under Resource Access, select Apple Events.
    ///
    /// ## Availability
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename = "com.apple.security.automation.apple-events",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub apple_events: Option<bool>,
}

/// Data Protection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DataProtection {
    #[serde(rename = "NSFileProtectionCompleteUnlessOpen")]
    FileProtectionCompleteUnlessOpen,
    #[serde(rename = "NSFileProtectionCompleteUntilFirstUserAuthentication")]
    FileProtectionCompleteUntilFirstUserAuthentication,
    #[serde(rename = "NSFileProtectionNone")]
    NSFileProtectionNone,
    #[serde(rename = "NSFileProtectionComplete")]
    NSFileProtectionComplete,
}

/// Device Check App Attest
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeviceCheckAppAttest {
    /// The App Attest sandbox environment that you use to test a device without affecting
    /// its risk metrics. Keys you create in the sandbox environment don’t work in the
    /// production environment.
    #[serde(rename = "development")]
    Development,
    /// The App Attest production environment. Keys you create in the production
    /// environment don’t work in the sandbox environment.
    #[serde(rename = "production")]
    Production,
}

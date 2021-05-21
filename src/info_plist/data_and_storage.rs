use crate::serialize_enum_option;
/// ## Data and Storage
///
/// Regulate documents, URLs, and other kinds of data movement and storage.
///
/// ### Overview
/// The system needs to know what kinds of data your app stores, provides, or consumes.
/// Add keys to your app’s Information Property List that declare your app’s data management capabilities.
use serde::{Deserialize, Serialize};

/// Documents
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Documents {
    /// The document types supported by the bundle.
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
        rename(serialize = "CFBundleDocumentTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_document_types: Option<Vec<BundleDocumentTypes>>,
    /// A Boolean value indicating whether the app is a document-based app.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename(serialize = "UISupportsDocumentBrowser"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_document_browser: Option<bool>,
    /// A Boolean value indicating whether the app may open the original document from a file provider, rather than a copy of the document.
    ///
    /// ## Availability
    /// * iOS 12.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename(serialize = "LSSupportsOpeningDocumentsInPlace"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_opening_documents_in_place: Option<bool>,
    /// The Core Data persistent store type associated with a document type.
    ///
    /// ## Availability
    /// * macOS 10.4+
    ///
    /// ## Framework
    /// * Core Data
    #[serde(
        rename(serialize = "NSPersistentStoreTypeKey"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub persistent_store_type_key: Option<PersistentStoreTypeKey>,
    /// A Boolean value that indicates whether the system should download documents before handing them over to the app.
    ///
    /// By default, the system displays the download progress. Set the value to YES if you want your app to display a custom download progress indicator instead.
    ///
    /// ## Availability
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(
        rename(serialize = "NSDownloadsUbiquitousContents"),
        skip_serializing_if = "Option::is_none"
    )]
    pub downloads_ubiquitous_contents: Option<bool>,
}
/// PersistentStoreTypeKey
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PersistentStoreTypeKey {
    #[serde(rename(serialize = "SQLite"))]
    SQLite,
    #[serde(rename(serialize = "XML"))]
    Xml,
    #[serde(rename(serialize = "Binary"))]
    Binary,
    #[serde(rename(serialize = "InMemory"))]
    InMemory,
}

/// URL Schemes
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct URLSchemes {
    /// A list of URL schemes (http, ftp, and so on) supported by the app.
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
        rename(serialize = "CFBundleURLTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_url_types: Option<Vec<BundleURLTypes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BundleDocumentTypes {
    /// The icon to associate with the document type.
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
        rename(serialize = "CFBundleTypeIconFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_type_icon_file: Option<String>,
    /// The abstract name for the document type.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * macOS 10.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(rename(serialize = "CFBundleTypeName"))]
    pub bundle_type_name: String,
    /// The app's role with respect to the document type.
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
        rename(serialize = "CFBundleTypeRole"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub bundle_type_role: Option<BundleTypeRole>,
    /// The ranking of this app among apps that declare themselves as editors or viewers of the given file type.
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
        rename(serialize = "LSHandlerRank"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub handler_rank: Option<HandlerRank>,
    /// The document file types the app supports.
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
        rename(serialize = "LSItemContentTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub item_content_types: Option<Vec<String>>,
    /// A Boolean value indicating whether the document is distributed as a bundle.
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
        rename(serialize = "LSTypeIsPackage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_is_package: Option<bool>,
    /// The subclass used to create instances of this document.
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
        rename(serialize = "NSDocumentClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub document_class: Option<String>,
    /// The file types that this document can be exported to.
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
        rename(serialize = "NSExportableTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exportable_types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BundleTypeRole {
    #[serde(rename(serialize = "Editor"))]
    Editor,
    #[serde(rename(serialize = "Viewer"))]
    Viewer,
    #[serde(rename(serialize = "Shell"))]
    Shell,
    #[serde(rename(serialize = "QLGenerator"))]
    QLGenerator,
    #[serde(rename(serialize = "None"))]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HandlerRank {
    #[serde(rename(serialize = "Owner"))]
    Owner,
    #[serde(rename(serialize = "Default"))]
    Default,
    #[serde(rename(serialize = "Alternate"))]
    Alternate,
    #[serde(rename(serialize = "None"))]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BundleURLTypes {
    /// The app’s role with respect to the type.
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
        rename(serialize = "CFBundleTypeRole"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_type_role: Option<BundleTypeRole>,
    /// The name of the icon image file, without the extension, to be used for this type.
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
        rename(serialize = "CFBundleURLIconFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_url_icon_file: Option<String>,
    /// The abstract name for this type.
    ///
    /// ## Availability
    /// * iOS 2.0+
    /// * macOS 10.0+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(rename(serialize = "CFBundleURLName"))]
    pub bundle_url_name: String,
    /// The URL schemes supported by this type.
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
        rename(serialize = "CFBundleURLSchemes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_url_schemes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UniversalTypeIdentifiers {
    /// The uniform type identifiers owned and exported by the app.
    ///
    /// ## Availability
    /// * iOS 5.0+
    /// * macOS 10.7+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename(serialize = "UTExportedTypeDeclarations"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exported_type_declarations: Option<Vec<ExportedTypeDeclarations>>,
    /// The uniform type identifiers inherently supported, but not owned, by the app.
    ///
    /// ## Availability
    /// * iOS 3.2+
    /// * macOS 10.5+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename(serialize = "UTImportedTypeDeclarations"),
        skip_serializing_if = "Option::is_none"
    )]
    pub imported_type_declarations: Option<Vec<ImportedTypeDeclarations>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExportedTypeDeclarations {
    /// The Uniform Type Identifier types that this type conforms to.  
    #[serde(rename(serialize = "UTTypeConformsTo"))]
    pub type_conforms_to: Vec<String>,
    /// A description for this type.
    #[serde(
        rename(serialize = "UTTypeConformsTo"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_description: Option<String>,
    /// The bundle icon resource to associate with this type.
    #[serde(
        rename(serialize = "UTTypeIconFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_icon_file: Option<String>,
    /// One or more bundle icon resources to associate with this type.
    #[serde(
        rename(serialize = "UTTypeIconFiles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_icon_files: Option<Vec<String>>,
    /// The Uniform Type Identifier to assign to this type.
    #[serde(rename(serialize = "UTTypeIdentifier"))]
    pub type_identifier: String,
    /// The webpage for a reference document that describes this type.
    #[serde(
        rename(serialize = "UTTypeReferenceURL"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_reference_url: Option<String>,
    /// A dictionary defining one or more equivalent type identifiers.
    #[serde(rename(serialize = "UTTypeTagSpecification"))]
    pub type_tag_specification: DefaultDictionary,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ImportedTypeDeclarations {
    /// The Uniform Type Identifier types that this type conforms to.
    #[serde(rename(serialize = "UTTypeConformsTo"))]
    pub type_conforms_to: Vec<String>,
    /// A description for this type.
    #[serde(
        rename(serialize = "UTTypeDescription"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_description: Option<String>,
    /// The bundle icon resource to associate with this type.
    #[serde(
        rename(serialize = "UTTypeIconFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_icon_file: Option<String>,
    /// One or more bundle icon resources to associate with this type.
    #[serde(
        rename(serialize = "UTTypeIconFiles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_icon_files: Option<Vec<String>>,
    /// The Uniform Type Identifier to assign to this type.
    #[serde(rename(serialize = "UTTypeIdentifier"))]
    pub type_identifier: String,
    /// The webpage for a reference document that describes this type.
    #[serde(
        rename(serialize = "UTTypeReferenceURL"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_reference_url: Option<String>,
    /// A dictionary defining one or more equivalent type identifiers.
    #[serde(rename(serialize = "UTTypeTagSpecification"))]
    pub type_tag_specification: DefaultDictionary,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DefaultDictionary {
    pub default: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Network {
    /// The URL where Private Click Measurement sends event attribution information.
    ///
    /// Include this key in your app to specify where the system sends event attribution data it receives from launched websites that support Private Click Measurement (PCM).
    /// The value provided for this key is a string that contains a valid URL that points to a server endpoint.
    /// PCM won’t work if your app doesn’t include this key.
    ///
    /// For more information on PCM and setting up a server to receive event attribution data, see Introducing Private Click Measurement.
    ///
    /// ### Note
    /// Mac apps built with Mac Catalyst don’t support PCM.
    ///
    /// ## Availability
    /// * iOS 14.5+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename(serialize = "NSAdvertisingAttributionReportEndpoint"),
        skip_serializing_if = "Option::is_none"
    )]
    pub advertising_attribution_report_endpoint: Option<String>,
    /// A description of changes made to the default security for HTTP connections.
    ///
    /// On Apple platforms, a networking feature called App Transport Security (ATS) improves privacy and data integrity for all apps and app extensions.
    /// ATS requires that all HTTP connections made with the URL Loading System—typically using the URLSession class—use HTTPS.
    /// It further imposes extended security checks that supplement the default server trust evaluation prescribed by the Transport Layer Security (TLS) protocol.
    /// ATS blocks connections that fail to meet minimum security specifications.
    /// For additional details, see Preventing Insecure Network Connections.
    ///
    /// You can circumvent or augment these protections by adding the NSAppTransportSecurity key to your app’s Information Property List file and providing an ATS configuration dictionary as the value.
    /// For example, you can:
    /// * Allow insecure loads for web views while maintaining ATS protections elsewhere in your app using the NSAllowsArbitraryLoadsInWebContent key.
    /// * Enable additional security features like Certificate Transparency using the NSRequiresCertificateTransparency key, or Certificate Pinning using the NSPinnedDomains key.
    /// * Reduce or remove security requirements for communication with particular servers using the NSExceptionDomains key.
    ///
    /// ### Important
    /// Always look for ways to improve server security before adding ATS exceptions.
    /// Loosening ATS restrictions reduces the security of your app.
    ///
    /// All keys in the ATS configuration dictionary are optional, with default values that are suitable for most apps.
    /// Keys that define global exceptions apply to all network connections made by your app, except connections to domains specified in the NSExceptionDomains sub-dictionary.
    /// That sub-dictionary allows you to separately manage settings for individual domains.
    ///
    /// ### Versioning
    /// ATS operates by default for apps linked against the iOS 9.0 or macOS 10.11 SDKs or later.
    /// When you link your app against an older SDK, ATS is disabled no matter which version of operating system your app runs on.
    ///
    /// If you specify a value for any of the global exceptions besides NSAllowsArbitraryLoads, then the ATS behavior depends on the version of the OS on which your app runs:
    /// * iOS 9.0 or macOS 10.11
    /// ATS uses the NSAllowsArbitraryLoads value that you set, or NO by default, and ignores the other global exceptions.
    /// * iOS 10.0 or later or macOS 10.12 or later
    /// ATS ignores the NSAllowsArbitraryLoads value that you set and instead obeys the other key or keys.
    ///
    /// This behavior enables you to manage differences between OS versions.
    /// You provide a coarse exception (NSAllowsArbitraryLoads) for older versions, and a more targeted exception, like NSAllowsArbitraryLoadsInWebContent, for when it’s available.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSAppTransportSecurity"),
        skip_serializing_if = "Option::is_none"
    )]
    pub app_transport_security: Option<AppTransportSecurity>,
    /// Bonjour service types browsed by the app.
    ///
    /// The value associated with this key is an array of strings that represent Bonjour service types.
    /// Include all service types that your app expects to use.
    /// Bonjour service type strings look like _ipp._tcp, and _myservice._udp, where the first substring identifies the application protocol and the second identifies the transport protocol.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    /// * tvOS 14.0+
    ///
    /// ## Framework
    /// * Network
    #[serde(
        rename(serialize = "NSBonjourServices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bonjour_services: Option<Vec<String>>,
    /// A Boolean value that indicates your app supports CloudKit Sharing.
    ///
    /// If your app supports CloudKit Sharing, add this key to your app’s Info.plist file with a value of true.
    /// This tells the system to launch your app when the user taps or clicks a share’s URL.
    /// For example, one they receive in an email or an iMessage from the share’s owner.
    ///
    /// Before your app launches, CloudKit verifies that the user has an active iCloud account and, for private shares, that it matches their participant details.
    /// Following successful verification, CloudKit provides the share’s metadata to your app’s scene, or application, delegate.
    /// The method it calls varies by platform and app configuration.
    /// For more information, see CKShare.Metadata.
    ///
    /// To indicate that your app supports CloudKit Sharing:
    /// 1. Select your project’s Info.plist file in the Project navigator in Xcode.
    /// 2. Click the Add button (+) next to any key in the property list editor and press Return.
    /// 3. Type the key name CKSharingSupported.
    /// 4. Choose Boolean from the pop-up menu in the Type column.
    /// 5. Choose YES from the pop-up menu in the Value column.
    /// 6. Save your changes.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.12+
    ///
    /// ## Framework
    /// * CloudKit
    #[serde(
        rename(serialize = "CKSharingSupported"),
        skip_serializing_if = "Option::is_none"
    )]
    pub sharing_supported: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppTransportSecurity {
    /// A Boolean value indicating whether App Transport Security restrictions are disabled for all network connections.
    ///
    /// Set this key’s value to YES to disable App Transport Security (ATS) restrictions for all domains not specified in the NSExceptionDomains dictionary.
    /// Domains you specify in that dictionary aren’t affected by this key’s value.
    ///
    /// ### Important
    /// You must supply a justification during App Store review if you set the key’s value to YES, as described in Provide Justification for Exceptions.
    /// Use this key with caution because it significantly reduces the security of your app.
    /// In most cases, it’s better to upgrade your servers to meet the requirements imposed by ATS, or at least to use a narrower exception.
    ///
    /// Disabling ATS means that unsecured HTTP connections are allowed.
    /// HTTPS connections are also allowed, and are still subject to default server trust evaluation, as described in Ensure the Network Server Meets Minimum Requirements.
    /// However, extended security checks—like requiring a minimum Transport Layer Security (TLS) protocol version—are disabled.
    /// Without ATS, you’re also free to loosen the default server trust requirements, as described in Performing Manual Server Trust Authentication.
    ///
    /// In iOS 10 and later and macOS 10.12 and later, the value of the NSAllowsArbitraryLoads key is ignored—and the default value of NO used instead—if any of the following keys are present in your app’s Information Property List file:
    /// * NSAllowsArbitraryLoadsForMedia
    /// * NSAllowsArbitraryLoadsInWebContent
    /// * NSAllowsLocalNetworking
    ///
    /// For more information about how the OS version affects ATS behavior, see the NSAppTransportSecurity key’s Versioning section.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSAllowsArbitraryLoads"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_arbitrary_loads: Option<bool>,
    /// A Boolean value indicating whether all App Transport Security restrictions are disabled for requests made using the AV Foundation framework.
    ///
    /// Set this key’s value to YES to disable App Transport Security restrictions for media loaded using the AVFoundation framework, without affecting your URLSession connections.
    /// Domains you specify in the NSExceptionDomains dictionary aren’t affected by this key’s value.
    ///
    /// Employ this key only for loading encrypted media—like files protected by FairPlay or by secure HTTP Live Streaming—that don’t contain personalized information.
    ///
    /// In iOS 10 and later and in macOS 10.12 and later, if you include this key with any value, then App Transport Security ignores the value of the NSAllowsArbitraryLoads key, instead using that key’s default value of NO.
    /// For more information about how the OS version affects ATS behavior, see the NSAppTransportSecurity key’s Versioning section.
    ///
    /// ### Important
    /// You must supply a justification during App Store review if you set the key’s value to YES, as described in Provide Justification for Exceptions.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.12+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSAllowsArbitraryLoadsForMedia"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_arbitrary_loads_for_media: Option<bool>,
    /// A Boolean value indicating whether all App Transport Security restrictions are disabled for requests made from web views.
    ///
    /// Set this key’s value to YES to exempt your app’s web views from App Transport Security restrictions without affecting your URLSession connections.
    /// Domains you specify in the NSExceptionDomains dictionary aren’t affected by this key’s value.
    ///
    /// A web view is an instance of any of the following classes:
    /// * WKWebView
    /// * UIWebView (iOS only)
    /// * WebView (macOS only)
    ///
    /// In iOS 10 and later and in macOS 10.12 and later, if you include this key with any value, then App Transport Security ignores the value of the NSAllowsArbitraryLoads key, instead using that key’s default value of NO.
    /// For more information about how the OS version affects ATS behavior, see the NSAppTransportSecurity key’s Versioning section.
    ///
    /// ### Important
    /// You must supply a justification during App Store review if you set the key’s value to YES, as described in Provide Justification for Exceptions.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.12+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSAllowsArbitraryLoadsInWebContent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_arbitrary_loads_in_web_content: Option<bool>,
    /// A Boolean value indicating whether to allow loading of local resources.
    ///
    /// In iOS 9 and macOS 10.11, App Transport Security (ATS) disallows connections to unqualified domains, .local domains, and IP addresses.
    /// You can add exceptions for unqualified domains and .local domains in the NSExceptionDomains dictionary, but you can’t add numerical IP addresses.
    /// Instead you use NSAllowsArbitraryLoads when you want to load directly from an IP address.
    ///
    /// In iOS 10 and macOS 10.12 and later, ATS allows all three of these connections by default, so you no longer need an exception for any of them.
    /// However, if you need to maintain compatibility with older versions of the OS, set both of the NSAllowsArbitraryLoads and NSAllowsLocalNetworking keys to YES.
    ///
    /// The local networking exception tells newer versions of the OS—which already allow unqualified domains, .local domains, and IP addresses—to ignore the arbitrary loads key.
    /// Meanwhile, the arbitrary loads key tells older versions of the OS, which don’t process the local networking exception key, to bypass ATS completely.
    /// This allows your app to work on different OS versions while minimizing the use of the wider exception.
    /// For more information about how global ATS exceptions interact across OS versions, see the NSAppTransportSecurity key’s Versioning section.
    ///
    /// ### Note
    /// While ATS doesn’t block local loads by default in newer versions of the OS, consider setting NSAllowsLocalNetworking to YES as a declaration of intent, if appropriate, even if you don’t support older OS versions.
    ///
    /// ## Availability
    /// * iOS 10.0+
    /// * macOS 10.12+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSAllowsLocalNetworking"),
        skip_serializing_if = "Option::is_none"
    )]
    pub allows_local_networking: Option<bool>,
    /// Custom App Transport Security configurations for named domains.
    ///
    /// The value for this key is a dictionary with keys that name specific domains for which you want to set exceptions.
    /// The value for each domain key is another dictionary that indicates the exceptions for that domain.
    ///
    /// ```swift
    /// NSExceptionDomains : Dictionary {
    ///     <domain-name-string> : Dictionary {
    ///         NSIncludesSubdomains : Boolean
    ///         NSExceptionAllowsInsecureHTTPLoads : Boolean
    ///         NSExceptionMinimumTLSVersion : String
    ///         NSExceptionRequiresForwardSecrecy : Boolean
    ///         NSRequiresCertificateTransparency : Boolean
    ///     }
    /// }
    /// ```
    /// Follow these rules when setting a domain name string:
    /// * Use lowercase. Use example.com, not EXAMPLE.COM.
    /// * Don’t include a port number. Use example.com, not example.com:443.
    /// * Don’t use numerical IP addresses. Don’t use 1.2.3.4. For information about how ATS handles IP addresses, see NSAllowsLocalNetworking.
    /// * Don’t include a trailing dot, unless you only want to match a domain string with a trailing dot. For example, example.com. (with a trailing dot) matches “example.com.” but not “example.com”.
    /// Similarly, example.com matches “example.com” but not “example.com.”.
    /// * Don’t use wildcard domains. Don’t use *.example.com. Instead, use example.com and set NSIncludesSubdomains to YES.
    ///
    /// The values for the keys in each individual domain’s dictionary control how ATS treats connections made to that domain.
    ///
    /// ### Note
    /// If you specify an exception domain dictionary, ATS ignores any global configuration keys, like NSAllowsArbitraryLoads, for that domain.
    /// This is true even if you leave the domain-specific dictionary empty and rely entirely on its keys’ default values.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSExceptionDomains"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_domains: Option<ExceptionDomains>,
    /// A collection of certificates that App Transport Security expects when connecting to named domains.
    ///
    /// The value for this optional key is a dictionary with keys that specify the domain names for which you want to set the expected certificates.
    /// The value for each domain name key is another dictionary that configures the expected certificates for that domain.
    ///
    /// ```swift
    /// NSPinnedDomains : Dictionary {
    ///     <domain-name-string> : Dictionary {
    ///         NSIncludesSubdomains : Boolean
    ///         NSPinnedCAIdentities : Array
    ///         NSPinnedLeafIdentities : Array
    ///     }
    /// }
    /// ```
    ///
    /// For any domain that you specify, you must include one or more expected Certificate Authority (CA) or sub-CA certificates as the value for the NSPinnedCAIdentities key, one or more expected leaf certificates as the value for the NSPinnedLeafIdentities key, or both.
    /// If you specify both, App Transport Security (ATS) requires a match in each category.
    ///
    /// To specify a domain name string, follow the rules for domain names given in NSExceptionDomains.
    /// You can also extend the pinning to cover subdomains by setting the value for the NSIncludesSubdomains key to YES.
    ///
    /// Pinning a certificate for a given domain has no impact on other security requirements or configuration.
    /// For example, pinning a CA certificate doesn’t change the way the system evaluates that certificate’s suitability as an anchor certificate.
    /// For information about securing network connections, see Preventing Insecure Network Connections.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSPinnedDomains"),
        skip_serializing_if = "Option::is_none"
    )]
    pub pinned_domains: Option<PinnedDomains>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExceptionDomains {
    /// A Boolean value that indicates whether to extend the configuration to subdomains of the given domain.
    ///
    /// You can include this key in any of the domain-specific dictionaries that you add to the NSExceptionDomains and NSPinnedDomains dictionaries.
    /// Adding the NSIncludesSubdomains key affects the applicability of the other configuration in the same domain-specific dictionary.
    /// The key is optional, with a default value of NO.
    ///
    /// Set the value for this key to YES to apply the configuration for the given domain to all subdomains of the domain that have one additional path component.
    /// For example, if you set this value to YES and the domain name string is example.com, then the configuration applies to example.com, as well as math.example.com and history.example.com.
    /// However, it doesn’t apply to the subdomains advanced.math.example.com or ancient.history.example.com because those subdomains have two additional path components.
    /// If the value is NO the configuration applies only to example.com.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSIncludesSubdomains"),
        skip_serializing_if = "Option::is_none"
    )]
    pub includes_subdomains: Option<bool>,
    /// A Boolean value indicating whether to allow insecure HTTP loads.
    ///
    /// Set the value for this key to YES to allow insecure HTTP loads for the given domain, or to be able to loosen the server trust evaluation requirements for HTTPS connections to the domain, as described in Performing Manual Server Trust Authentication.
    ///
    /// Using this key doesn’t by itself change default server trust evaluation requirements for HTTPS connections, described in Ensure the Network Server Meets Minimum Requirements.
    /// Using only this key also doesn’t change the TLS or forward secrecy requirements imposed by ATS.
    /// As a result, you might need to combine this key with the NSExceptionMinimumTLSVersion or NSExceptionRequiresForwardSecrecy key in certain cases.
    ///
    /// This key is optional.
    /// The default value is NO.
    ///
    /// ### Important
    /// You must supply a justification during App Store review if you set the key’s value to YES, as described in Provide Justification for Exceptions.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSExceptionAllowsInsecureHTTPLoads"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_allows_insecure_http_loads: Option<bool>,
    /// The minimum Transport Layer Security (TLS) version for network connections.
    ///
    /// This key is optional. The value is a string, with a default value of TLSv1.2.
    ///
    /// ### Important
    /// You must supply a justification during App Store review if you use this key to set a protocol version lower than 1.2, as described in Provide Justification for Exceptions.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSExceptionMinimumTLSVersion"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub exception_minimum_tls_version: Option<ExceptionMinimumTLSVersion>,
    /// A Boolean value indicating whether to override the perfect forward secrecy requirement.
    ///
    /// Set the value for this key to NO to override the requirement that a server support perfect forward secrecy (PFS) for the given domain.
    /// Disabling this requirement also removes the key length check described in Ensure the Network Server Meets Minimum Requirements.
    /// However, it doesn’t impact the TLS version requirement.
    /// To control that, use NSExceptionMinimumTLSVersion.
    ///
    /// This key is optional.
    /// The default value is YES, which limits the accepted ciphers to those that support PFS through Elliptic Curve Diffie-Hellman Ephemeral (ECDHE) key exchange.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSExceptionRequiresForwardSecrecy"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_requires_forward_secrecy: Option<bool>,
    /// A Boolean value indicating whether to require Certificate Transparency.
    ///
    /// Certificate Transparency (CT) is a protocol that ATS can use to identify mistakenly or maliciously issued X.509 certificates.
    /// Set the value for the NSRequiresCertificateTransparency key to YES to require that for a given domain, server certificates are supported by valid, signed CT timestamps from at least two CT logs trusted by Apple.
    /// For more information about Certificate Transparency, see RFC6962.
    ///
    /// Unlike most other ATS exceptions, using a non-default value in this case tightens security requirements.
    ///
    /// This key is optional.
    /// The default value is NO.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSRequiresCertificateTransparency"),
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_certificate_transparency: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ExceptionMinimumTLSVersion {
    /// Require a minimum TLS version of 1.0.
    #[serde(rename(serialize = "TLSv1.0"))]
    TlSv10,
    /// Require a minimum TLS version of 1.1.
    #[serde(rename(serialize = "TLSv1.1"))]
    TlSv11,
    /// Require a minimum TLS version of 1.2.
    #[serde(rename(serialize = "TLSv1.2"))]
    TlSv12,
    /// Require a minimum TLS version of 1.3.
    #[serde(rename(serialize = "TLSv1.3"))]
    TlSv13,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PinnedDomains {
    /// A Boolean value that indicates whether to extend the configuration to subdomains of the given domain.
    ///
    /// You can include this key in any of the domain-specific dictionaries that you add to the NSExceptionDomains and NSPinnedDomains dictionaries.
    /// Adding the NSIncludesSubdomains key affects the applicability of the other configuration in the same domain-specific dictionary.
    /// The key is optional, with a default value of NO.
    ///
    /// Set the value for this key to YES to apply the configuration for the given domain to all subdomains of the domain that have one additional path component.
    /// For example, if you set this value to YES and the domain name string is example.com, then the configuration applies to example.com, as well as math.example.com and history.example.com.
    /// However, it doesn’t apply to the subdomains advanced.math.example.com or ancient.history.example.com because those subdomains have two additional path components.
    ///
    /// If the value is NO the configuration applies only to example.com.
    ///
    /// ## Availability
    /// * iOS 9.0+
    /// * macOS 10.11+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSIncludesSubdomains"),
        skip_serializing_if = "Option::is_none"
    )]
    pub includes_subdomains: Option<bool>,
    /// A list of allowed Certificate Authority certificates for a given domain name.
    ///
    /// Provide an array of dictionaries as the value for this key.
    /// Each dictionary in the array contains the SPKI-SHA256-BASE64 key with a value that represents the Base64-encoded SHA-256 digest of an X.509 certificate’s DER-encoded ASN.1 Subject Public Key Info (SPKI) structure.
    ///
    /// ```swift
    /// NSPinnedCAIdentities : Array {
    ///     Dictionary {
    ///         SPKI-SHA256-BASE64 : String
    ///     }
    /// }
    /// ```
    ///
    /// When making a network connection to a named domain, App Transport Security (ATS) blocks the connection unless it can find the SPKI digest of at least one Certificate Authority (CA) or sub-CA certificate in the chain presented by the server.
    ///
    /// You must include this key or the NSPinnedLeafIdentities key or both in each domain-specific NSPinnedDomains subdictionary.
    /// If you include both, then both must produce a match.
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "NSPinnedCAIdentities"),
        skip_serializing_if = "Option::is_none"
    )]
    pub pinned_ca_identities: Option<Vec<SPKISHA256BASE64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SPKISHA256BASE64 {
    /// The digest of an X.509 certificate’s Subject Public Key Info structure.
    ///
    /// You represent a pinned certificate using the Base64-encoded SHA-256 digest of an X.509 certificate’s DER-encoded ASN.1 Subject Public Key Info (SPKI) structure.
    /// For a PEM-encoded public-key certificate stored in the file ca.pem, you can calculate the SPKI-SHA256-BASE64 value with the following openssl commands:
    ///
    /// ```swift
    /// % cat ca.pem |
    ///   openssl x509 -inform pem -noout -outform pem -pubkey |
    ///   openssl pkey -pubin -inform pem -outform der |
    ///   openssl dgst -sha256 -binary |
    ///   openssl enc -base64
    /// ```
    ///
    /// ## Availability
    /// * iOS 14.0+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Security
    #[serde(
        rename(serialize = "SPKI-SHA256-BASE64"),
        skip_serializing_if = "Option::is_none"
    )]
    pub spki_sha256_base64: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Storage {
    /// Describes the files or directories the app installs on the system.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(rename(serialize = "APFiles"), skip_serializing_if = "Option::is_none")]
    pub files: Option<Files>,
    /// The base path to the files or directories the app installs.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(
        rename(serialize = "APInstallerURL"),
        skip_serializing_if = "Option::is_none"
    )]
    pub installer_url: Option<String>,
    /// A Boolean value indicating whether the app continues working if the system purges the local storage.
    ///
    /// ## Availability
    /// * iOS 9.3+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSSupportsPurgeableLocalStorage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_purgeable_local_storage: Option<bool>,
    /// A Boolean value indicating whether the files this app creates are quarantined by default.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename(serialize = "LSFileQuarantineEnabled"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_quarantine_enabled: Option<bool>,
    /// A Boolean value indicating whether the app shares files through iTunes.
    ///
    /// ## Availability
    /// * iOS 3.2+
    /// * tvOS 9.0+
    /// * watchOS 2.0+
    ///
    /// ## Framework
    /// * UIKit
    #[serde(
        rename(serialize = "UIFileSharingEnabled"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_sharing_enabled: Option<bool>,
    /// A Boolean value indicating whether the app's resources files should be mapped into memory.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Core Foundation
    #[serde(
        rename(serialize = "CSResourcesFileMapped"),
        skip_serializing_if = "Option::is_none"
    )]
    pub resources_file_mapped: Option<bool>,
    /// A Boolean value that indicates whether the system should download documents before handing them over to the app.
    ///
    /// By default, the system displays the download progress.
    /// Set the value to YES if you want your app to display a custom download progress indicator instead.
    ///
    /// ## Availability
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(
        rename(serialize = "NSDownloadsUbiquitousContents"),
        skip_serializing_if = "Option::is_none"
    )]
    pub downloads_ubiquitous_contents: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Files {
    /// A Boolean value indicating whether the file or a folder icon is displayed in the Info window.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(
        rename(serialize = "APDisplayedAsContainer"),
        skip_serializing_if = "Option::is_none"
    )]
    pub displayed_as_container: Option<bool>,
    /// A short description of the file or folder that appears in the Info window.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(rename(serialize = "APFileDescriptionKey"))]
    pub file_description_key: String,
    /// The path to use when installing the file or folder, relative to the app bundle.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(rename(serialize = "APFileDestinationPath"))]
    pub file_destination_path: String,
    /// The name of the file or folder to install.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(rename(serialize = "APFileName"))]
    pub file_name: String,
    /// The path to the file or folder in the app package, relative to the installer path.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(rename(serialize = "APFileSourcePath"))]
    pub file_source_path: String,
    /// The action to take on the file or folder.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * AppKit
    #[serde(
        rename(serialize = "APInstallAction"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub install_action: Option<InstallAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InstallAction {
    #[serde(rename(serialize = "Copy"))]
    Copy,
    #[serde(rename(serialize = "Open"))]
    Open,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CoreMLModels {
    /// A Boolean value indicating whether the app contains a Core ML model to optimize loading the model.
    ///
    /// ## Availability
    /// * iOS 12.0+
    /// * macOS 10.0+
    /// * tvOS 12.0+
    /// * watchOS 5.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename(serialize = "LSBundleContainsCoreMLmlmodelc"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_contains_core_ml_mlmodelc: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Java {
    /// The root directory for the app’s Java class files.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename(serialize = "NSJavaRoot"),
        skip_serializing_if = "Option::is_none"
    )]
    pub java_root: Option<String>,
}

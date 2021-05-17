use super::DefaultDictionary;
use crate::{serialize_enum_option, serialize_vec_enum_option};
/// ## Data nad Storage
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
    /// Framework
    /// Core Foundation
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
    /// Framework
    /// UIKit
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
    /// Framework
    /// Core Services
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
    /// Framework
    /// Core Data
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
    /// Framework
    /// AppKit
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
    /// Framework
    /// Core Foundation
    #[serde(
        rename(serialize = "CFBundleURLTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_url_types: Option<Vec<DefaultDictionary>>,
}

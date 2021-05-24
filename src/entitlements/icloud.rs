use crate::{serialize_enum_option, serialize_vec_enum_option};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Icloud {
    /// The container identifiers for the iCloud development environment.
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
        rename(
            serialize = "com.apple.developer.icloud-container-development-container-identifiers"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub icloud_container_development_container_identifiers: Option<Vec<String>>,
    /// The development or production environment to use for the iCloud containers.
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
        rename(serialize = "com.apple.developer.icloud-container-environment"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub icloud_container_environment: Option<IcloudContainerEnvironment>,
    /// The container identifiers for the iCloud production environment.
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
        rename(serialize = "com.apple.developer.icloud-container-identifiers"),
        skip_serializing_if = "Option::is_none"
    )]
    pub icloud_container_identifiers: Option<Vec<String>>,
    /// The iCloud services used by the app.
    ///
    /// To add this entitlement to your app, enable the iCloud capability and the iCloud Documents or CloudKit service in Xcode.
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
        rename(serialize = "com.apple.developer.icloud-services"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub icloud_services: Option<Vec<IcloudServices>>,
    /// The container identifier to use for iCloud key-value storage.
    ///
    /// To add this entitlement to your app, enable the iCloud capability and “Key-value storage” service in Xcode.
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
        rename(serialize = "com.apple.developer.ubiquity-kvstore-identifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub icloud_key_value_store: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IcloudContainerEnvironment {
    #[serde(rename(serialize = "Development"))]
    Development,
    #[serde(rename(serialize = "Production"))]
    Production,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IcloudServices {
    #[serde(rename(serialize = "CloudDocuments"))]
    CloudDocuments,
    #[serde(rename(serialize = "CloudKit"))]
    CloudKit,
}

//! # Apple Bundle Resources
//!
//! Resources located in an app, framework, or plugin bundle.
//!
//! A bundle is a directory with a standardized hierarchical structure that holds executable code and the resources
//! used by that code. The bundle contains resources that may be accessed at runtime, such as images, audio files,
//! user interface files, and property lists.
//!
//! Official documentation: https://developer.apple.com/documentation/bundleresources

/// Entitlements
///
/// Key-value pairs that grant an executable permission to use a service or technology.
pub mod entitlements;

/// Information Property List
///
/// A resource containing key-value pairs that identify and configure a bundle.
pub mod info_plist;

pub mod prelude {
    pub use super::entitlements::prelude::*;
    pub use super::info_plist::prelude::*;
}

use serde::{ser::SerializeSeq, Serialize, Serializer};

fn serialize_enum_option<S: Serializer, T: Serialize>(
    value: &Option<T>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&serde_plain::to_string(value).unwrap())
}

fn serialize_vec_enum_option<S: Serializer, T: Serialize>(
    value: &Option<Vec<T>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(ref val) => {
            let mut seq = s.serialize_seq(Some(val.len()))?;
            for element in val.iter() {
                seq.serialize_element(&serde_plain::to_string(element).unwrap())?;
            }
            seq.end()
        }
        None => panic!("unsupported"),
    }
}

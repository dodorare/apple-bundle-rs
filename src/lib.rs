//! Apple Bundle Resources
//!
//! Resources located in an app, framework, or plugin bundle.
//!
//! A bundle is a directory with a standardized hierarchical structure that holds executable code and the resources
//! used by that code. The bundle contains resources that may be accessed at runtime, such as images, audio files,
//! user interface files, and property lists.
//!
//! Official documentation: https://developer.apple.com/documentation/bundleresources
//!

/// Entitlements
/// Key-value pairs that grant an executable permission to use a service or technology.
pub mod entitlements;

/// Information Property List
/// A resource containing key-value pairs that identify and configure a bundle.
pub mod info_plist;

pub mod prelude {
    pub use super::entitlements::prelude::*;
    pub use super::info_plist::prelude::*;
}

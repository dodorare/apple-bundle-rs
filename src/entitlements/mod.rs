//! # Entitlements
//!
//! Key-value pairs that grant an executable permission to use a service or technology.
//!
//! An entitlement is a right or privilege that grants an executable particular capabilities. For example, an app needs
//! the HomeKit Entitlement — along with explicit user consent — to access a user’s home automation network. An app stores
//! its entitlements as key-value pairs embedded in the code signature of its binary executable.
//!
//! You configure entitlements for your app by declaring capabilities for a target in Xcode. Xcode records capabilities
//! that you add in a property list file with the .entitlements extension. You can also edit the entitlements file directly.
//! When code signing your app, Xcode combines the entitlements file, information from your developer account, and other
//! project information to apply a final set of entitlements to your app.
//!
//! Official documentation: https://developer.apple.com/documentation/bundleresources/entitlements
//!
//! ## Availability
//! * iOS 2.0+
//! * macOS 10.7+
//! * tvOS 9.0+
//! * watchOS 2.0+
//!
//! ## Framework
//! Bundle Resources

pub mod app_clips;
pub mod authentication;
pub mod car_play;
pub mod contacts;
pub mod deprecated_entitlements;
pub mod education;
pub mod exposure_notification;
pub mod games;
pub mod health;
pub mod home_automation;
pub mod hypervisor;
pub mod icloud;
pub mod networking;
pub mod push_notifications;
pub mod security;
pub mod sensors;
pub mod siri;
pub mod system;
pub mod tv;
pub mod wallet;

pub mod prelude {
    pub use super::app_clips::*;
    pub use super::authentication::*;
    pub use super::car_play::*;
    pub use super::contacts::*;
    pub use super::deprecated_entitlements::*;
    pub use super::education::*;
    pub use super::exposure_notification::*;
    pub use super::games::*;
    pub use super::health::*;
    pub use super::home_automation::*;
    pub use super::hypervisor::*;
    pub use super::icloud::*;
    pub use super::networking::*;
    pub use super::push_notifications::*;
    pub use super::security::*;
    pub use super::sensors::*;
    pub use super::siri::*;
    pub use super::system::*;
    pub use super::tv::*;
    pub use super::wallet::*;
    pub use super::Entitlements;
}

use prelude::*;
use serde::{Deserialize, Serialize};

/// Entitlements.
/// https://developer.apple.com/documentation/bundleresources/entitlements
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Entitlements {
    /// Authentication
    #[serde(flatten)]
    pub authentication: Authentication,
}

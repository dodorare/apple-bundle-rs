//! Entitlements
//!
//! Key-value pairs that grant an executable permission to use a service or technology.
//!
//! Bundle Resources
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

pub mod prelude {}

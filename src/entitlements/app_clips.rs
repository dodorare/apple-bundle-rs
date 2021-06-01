use serde::{Deserialize, Serialize};

/// App Clips
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AppClips {
    /// A list of parent application identifiers for an App Clip with exactly one entry.
    ///
    /// The Parent Application Identifiers entitlement establishes a secure association between an App Clip and its corresponding app
    /// Add it only to an App Clip target.
    ///
    /// ### Note
    /// When you add an App Clip target to your project as described in Creating an App Clip with Xcode,
    /// Xcode creates this entitlement and adds the correct value.
    ///
    /// Because an App Clip is always associated with exactly one app, ensure the parent application entitlement
    /// has exactly one entry, the corresponding app’s application identifier.
    ///
    /// Ensure that the application identifier for the App Clip uses the full app’s application identifier as its
    /// prefix, followed by a string.
    /// For example, if your app’s application identifier is $(AppIdentifierPrefix)com.example.
    /// MyApp, the App Clip’s application identifier may be $(AppIdentifierPrefix)com.example.MyApp.Clip.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * App Clip
    #[serde(
        rename = "com.apple.developer.parent-application-identifiers",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_application_identifiers: Option<Vec<String>>,
    /// A Boolean value that indicates whether a bundle represents an App Clip.
    ///
    /// Adding an App Clip target to your project as described in Creating an App Clip with Xcode enables
    /// a capability called On Demand Install Capable for the App Clip target.
    ///
    /// When you code-sign your full app, Xcode embeds the App Clip in the full app and applies the
    /// com.apple.developer.on-demand-install-capable entitlement.
    /// Because of this behavior, the App Clip’s .entitlements file doesn’t include this entitlement
    /// if you open the file in Xcode’s Project navigator.
    ///
    /// To see the entitlement in the .entitlements file, first archive the full app, then export the
    /// App Clip for distribution as described in Distributing Your App Clip.
    /// Next, open the Terminal app and run codesign -d --entitlements :- /path/to/ExampleApp.app/AppClips/ExampleAppClip.app.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * App Clip
    #[serde(
        rename = "com.apple.developer.on-demand-install-capable",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_demand_install_capable: Option<bool>,
}

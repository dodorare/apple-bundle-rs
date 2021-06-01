use crate::serialize_enum_option;
use serde::{Deserialize, Serialize};

/// Education
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Education {
    /// The ClassKit development or production environment for an education app that works with the Schoolwork app.
    ///
    /// This key specifies the ClassKit environment your app uses to share data with Apple’s Schoolwork app.
    ///
    /// To support testing locally, Xcode sets the value to development by default. When you upload your app to
    /// the App Store, Xcode changes the value to production.
    ///
    /// To add this entitlement to your app, enable the ClassKit capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 11.4+
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * ClassKit
    #[serde(
        rename(serialize = "com.apple.developer.ClassKit-environment"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_enum_option"
    )]
    pub classkit_environment: Option<ClassKitEnvironment>,
    /// A Boolean value that indicates whether an app may create an assessment session.
    ///
    /// Use an AEAssessmentSession instance to put a device into a state that prevents users from accessing
    /// certain system features during high-stakes assessment activities, such as administering an exam.
    /// Your app needs the com.apple.developer.automatic-assessment-configuration entitlement to create an assessment session.
    ///
    /// To add the entitlement to your app, set the entitlement’s type to Boolean in the Xcode property list editor, and the corresponding value to YES.
    ///
    /// Before your app can use this entitlement, you must first get permission to use it.
    /// Request permission by filling in the Automatic Assessment Configuration Entitlement Request form.
    ///
    /// ### Important
    /// If your app has a deployment target earlier than macOS 11, to use the com.apple.developer.automatic-assessment-configuration entitlement, your app also needs the com.apple.security.temporary-exception.mach-lookup.global-name entitlement.
    /// Add this to your app’s entitlements file with a corresponding value that’s an array of strings containing the string com.apple.assessmentagent.
    ///
    /// ## Availability
    /// * iOS 13.4+
    /// * macOS 10.15.4+
    ///
    /// ## Framework
    /// * Automatic Assessment Configuration
    #[serde(
        rename(serialize = "com.apple.developer.automatic-assessment-configuration"),
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_assessment_configuration: Option<bool>,
}

/// ClassKit Environment Entitlement
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ClassKitEnvironment {
    /// The environment used to develop and test your app locally, without requiring a Managed Apple ID issued
    /// by an educational institution.
    #[serde(rename(serialize = "development"))]
    Development,
    /// The environment used by customers of your app who have a Managed Apple ID. This enviroment enables
    /// teachers and students to share data through iCloud.
    #[serde(rename(serialize = "production"))]
    Production,
}

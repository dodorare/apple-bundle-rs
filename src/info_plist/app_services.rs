//! # App Services
//!
//! Configure services provided by the app, like support for giving directions or using game controllers.
//!
//! Add keys to your app’s Information Property List file that tell the system about services that your app provides.
//!
//! ## Framework
//! * Bundle Resources

use crate::serialize_enum_option;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct CarPlay {
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "CPSupportsDashboardNavigationScene"),
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_dashboard_navigation_scene: Option<bool>,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "CPTemplateApplicationDashboardSceneSessionRoleApplication"),
        skip_serializing_if = "Option::is_none"
    )]
    pub template_application_dashboard: Option<Vec<TemplateApplicationDashboard>>,
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(
        rename(serialize = "CPTemplateApplicationSceneSessionRoleApplication"),
        skip_serializing_if = "Option::is_none"
    )]
    pub template_application_scene_session_role: Option<Vec<TemplateApplicationSceneSessionRole>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TemplateApplicationDashboard {
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename(serialize = "UISceneClassName"))]
    pub scene_class_name: ClassName,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename(serialize = "UISceneConfigurationName"))]
    pub scene_configuration_name: String,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename(serialize = "UISceneDelegateClassName"))]
    pub scene_delegate_class_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum ClassName {
    #[serde(rename(serialize = "CPTemplateApplicationDashboardScene"))]
    TemplateApplicationDashboardScene,
}

impl Default for ClassName {
    fn default() -> Self {
        Self::TemplateApplicationDashboardScene
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct TemplateApplicationSceneSessionRole {
    /// ## Availability
    /// * iOS 13.0+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename(serialize = "UISceneClassName"))]
    pub scene_class_name: TemplateApplication,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename(serialize = "UISceneConfigurationName"))]
    pub scene_configuration_name: String,
    /// ## Availability
    /// * iOS 13.1+
    ///
    /// ## Framework
    /// * CarPlay
    #[serde(rename(serialize = "UISceneDelegateClassName"))]
    pub scene_delegate_class_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum TemplateApplication {
    #[serde(rename(serialize = "CPTemplateApplicationScene"))]
    Scene,
}

impl Default for TemplateApplication {
    fn default() -> Self {
        Self::Scene
    }
}

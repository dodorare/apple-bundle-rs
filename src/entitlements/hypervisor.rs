use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Hypervisor {
    /// A Boolean value that indicates whether the app creates and manages virtual machines.
    ///
    /// The entitlement is required to use the Hypervisor APIs in any process.
    ///
    /// ## Important
    ///
    /// If your app has a deployment target of macOS 10.15 or earlier, add the com.apple.vm.hypervisor entitlement to your app in addition to this entitlement.
    /// ## Availability
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Hypervisor
    #[serde(
        rename(serialize = "com.apple.security.hypervisor"),
        skip_serializing_if = "Option::is_none"
    )]
    pub security_hypervisor: Option<bool>,
    /// A Boolean value that indicates whether the app creates and manages virtual machines.
    ///
    /// The entitlement is required to use the Hypervisor APIs in a sandboxed process.
    ///
    /// ## Availability
    /// * macOS 10.10–11.0
    ///
    /// ## Framework
    /// * Hypervisor
    #[deprecated(
        since = "macOS 10.10–11.0",
        note = "For apps with a deployment target of macOS 11 and later, use com.apple.security.hypervisor instead.
        For deployment targets earlier than macOS 11, add both that and the com.apple.vm.hypervisor entitlement to your app."
    )]
    #[serde(
        rename(serialize = "com.apple.vm.hypervisor"),
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_hypervisor: Option<bool>,
}

use serde::{Deserialize, Serialize};

/// Hypervisor
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Hypervisor {
    /// A Boolean value that indicates whether the app creates and manages virtual
    /// machines.
    ///
    /// The entitlement is required to use the Hypervisor APIs in any process.
    ///
    /// ### Important
    ///
    /// If your app has a deployment target of macOS 10.15 or earlier, add the
    /// com.apple.vm.hypervisor entitlement to your app in addition to this entitlement.
    ///
    /// ## Availability
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Hypervisor
    #[serde(
        rename = "com.apple.security.hypervisor",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_hypervisor: Option<bool>,
    /// A Boolean value that indicates whether the app creates and manages virtual
    /// machines.
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
        rename = "com.apple.vm.hypervisor",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_hypervisor: Option<bool>,
    /// A Boolean value that indicates whether the app captures USB devices and uses them
    /// in the guest-operating system.
    ///
    /// The entitlement is required to use the IOUSBHost APIs for USB device capture.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * Hypervisor
    #[serde(
        rename = "com.apple.vm.device-access",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_device_access: Option<bool>,
    /// A Boolean that indicates whether the app manages virtual network interfaces
    /// without escalating privileges to the root user.
    ///
    /// The entitlement is required to use the vmnet APIs.
    ///
    /// ### Note
    /// This entitlement is restricted to developers of virtualization software.
    /// To request this entitlement, contact your Apple representative.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * Hypervisor
    #[serde(
        rename = "com.apple.vm.networking",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_networking: Option<bool>,
    /// A Boolean that indicates whether the app can use the Virtualization framework.
    ///
    /// Read the value of isSupported to check for the presence of both this entitlement
    /// and the hardware support needed for virtualization.
    ///
    /// ## Availability
    /// * macOS 11.0+
    ///
    /// ## Framework
    /// * Hypervisor
    #[serde(
        rename = "com.apple.security.virtualization",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_virtualization: Option<bool>,
}

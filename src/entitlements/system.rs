use serde::{Deserialize, Serialize};

/// Extend the capabilities of macOS from user space.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct System {
    /// A Boolean value that indicates whether your app has permission to activate or deactivate system extensions.
    ///
    /// To add this entitlement to your app, enable the System Extension capability in Xcode.
    /// Add this entitlement for all system extension types, including DriverKit extensions.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * System Extensions
    #[serde(
        rename(serialize = "com.apple.developer.system-extension.install"),
        skip_serializing_if = "Option::is_none"
    )]
    pub system_extension: Option<bool>,
    /// A Boolean value that indicates whether other development teams may distribute a system extension you create.
    ///
    /// Add this entitlement to a system extension that you create and sign using your development team credentials, but which other development teams distribute in their apps.
    /// This extension allows a distributing app to have a different team ID than the one associated with the system extension.
    /// If this entitlement isn't present, the team ID of the app and system extension must match.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * System Extensions
    #[serde(
        rename(serialize = "com.apple.developer.system-extension.redistributable"),
        skip_serializing_if = "Option::is_none"
    )]
    pub system_extension_redistributable: Option<bool>,
    /// The entitlement required to monitor system events for potentially malicious activity.
    ///
    /// You must request this entitlement from Apple.
    /// For information about how to request the entitlement, see System Extensions and DriverKit.
    ///
    /// If your app or extension lacks this requirement, es_new_client fails with the result ES_NEW_CLIENT_RESULT_ERR_NOT_ENTITLED.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * Endpoint Security
    #[serde(
        rename(serialize = "com.apple.developer.endpoint-security.client"),
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_security_client: Option<bool>,
    /// A Boolean value that indicates whether your extension has permission to run as a user-space driver.
    ///
    /// Add this entitlement to every DriverKit driver you create.
    /// You must request this entitlement from Apple.
    /// For information about how to request the entitlement, see System Extensions and DriverKit.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit: Option<bool>,
    /// A Boolean value indicating whether to match the driver against devices that communicate using networking protocols.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.family.networking"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_family_networking: Option<bool>,
    /// A Boolean value that indicates whether to match the driver against devices with SCSI controllers.
    ///
    /// Add this entitlement to the default entitlements file that Xcode created for your driver project.
    ///
    /// ## Availability
    /// * macOS 11.3+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.family.scsicontroller"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_family_scsi_controller: Option<bool>,
    /// A Boolean value that indicates whether to match the driver against devices with serial communication interfaces.
    ///
    /// Add this entitlement to the default entitlements file that Xcode created for your driver project.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.family.serial"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_family_serial: Option<bool>,
    /// An array of PCI device descriptors that your custom driver supports.
    ///
    /// Each element in the array is a dictionary whose keys identify a supported device.
    /// The values of these keys correspond to values stored in the PCI device’s configuration registers.
    ///
    /// You can provide several matching values for a key, separated by spaces.
    /// You can also specify an optional mask for the configuration register value by putting the mask after the value, prepended with an & character.
    ///
    /// ### Note
    /// You also use the keys defined by this entitlement in your app’s Info.plist, to identify which devices your driver loads on.
    ///
    /// ## Availability
    /// * macOS 10.15.4+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.transport.pci"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_transport_pci: Option<Vec<DefaultDictionary>>,
    /// An array of dictionaries that identify the USB devices the driver supports.
    ///
    /// Each element in the array is a dictionary whose keys and values identify a specific type of supported device.
    /// The keys in the dictionary correspond to field names of the device descriptor associated with the USB device.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.transport.usb"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_transport_usb: Option<Vec<DefaultDictionary>>,
    /// An array of strings that represent driver extensions which may communicate with other DriverKit services.
    ///
    /// Add this entitlement to your app that opens the IOUserClient.
    /// Set its value to an array of bundle IDs of driver extensions that you want to use with DriverKit.
    /// If you have only one bundle ID, you can use either a single string or a one-element array.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.userclient-access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_userclient_access: Option<Vec<String>>,
    /// A Boolean value that indicates whether the driver provides a HID-related service to the system.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * HIDDriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.family.hid.device"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_family_hid_device: Option<bool>,
    /// A Boolean value that indicates whether the driver provides a HID-related event service to the system.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * HIDDriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.family.hid.eventservice"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_family_hid_eventservice: Option<bool>,
    /// A Boolean value that indicates whether the driver communicates with human interface devices.
    ///
    /// This entitlement gives your driver permission to interact with the hardware for a human interface device.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * HIDDriverKit
    #[serde(
        rename(serialize = "com.apple.developer.driverkit.transport.hid"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_transport_hid: Option<bool>,
    /// A Boolean value that indicates whether the driver creates a virtual HID device.
    ///
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * HIDDriverKit
    #[serde(
        rename(serialize = "com.apple.developer.hid.virtual.device"),
        skip_serializing_if = "Option::is_none"
    )]
    pub hid_virtual_device: Option<bool>,
}

/// Default Dictionary.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct DefaultDictionary {
    pub default: String,
}

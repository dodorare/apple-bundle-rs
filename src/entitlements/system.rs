use serde::{Deserialize, Serialize};

/// System
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
    /// * Bundle resources
    #[serde(rename = "System Extensions", skip_serializing_if = "Option::is_none")]
    pub system_extension: Option<SystemExtensions>,
    /// A Boolean that indicates whether the app can act as a user’s default mail client.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename = "com.apple.developer.mail-client",
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_client: Option<bool>,
    /// A Boolean that indicates whether the app can act as the user’s default web browser.
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * Core Services
    #[serde(
        rename = "com.apple.developer.web-browser",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_browser: Option<bool>,
}

/// System Extensions
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct SystemExtensions {
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
        rename = "com.apple.developer.system-extension.redistributable",
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
        rename = "com.apple.developer.endpoint-security.client",
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
        rename = "com.apple.developer.driverkit",
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
        rename = "com.apple.developer.driverkit.family.networking",
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
        rename = "com.apple.developer.driverkit.family.scsicontroller",
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
        rename = "com.apple.developer.driverkit.family.serial",
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
    /// Examples:
    /// * Key = IOPCIMatch. Value = 0x00261011. Result = Matches a device whose vendor ID is 0x1011, and device ID is 0x0026, including subsystem IDs.
    /// * Key = IOPCIMatch. Value = 0x00789004&0x00ffffff 0x78009004&0x0xff00ffff. Result = Matches with any device with a vendor ID of 0x9004, and a device ID of 0xzz78 or 0x78zz, where ‘z’ is any hexadecimal digit.
    /// * Key = IOPCIClassMatch. Value = 0x02000000&0xffff0000. Result = Matches a device whose class code is 0x0200zz (where ‘z’ is any hexadecimal digit), an ethernet device.
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
        rename = "com.apple.developer.driverkit.transport.pci",
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_transport_pci: Option<Vec<DriverkitTransportPci>>,
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
        rename = "com.apple.developer.driverkit.transport.usb",
        skip_serializing_if = "Option::is_none"
    )]
    pub driverkit_transport_usb: Option<Vec<DriverkitTransportUsb>>,
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
        rename = "com.apple.developer.driverkit.userclient-access",
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
        rename = "com.apple.developer.driverkit.family.hid.device",
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
        rename = "com.apple.developer.driverkit.family.hid.eventservice",
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
        rename = "com.apple.developer.driverkit.transport.hid",
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
        rename = "com.apple.developer.hid.virtual.device",
        skip_serializing_if = "Option::is_none"
    )]
    pub hid_virtual_device: Option<bool>,
}

/// Driver Kit Transport PCI
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct DriverkitTransportPci {
    /// A key to match PCI devices by vendor and device ID registers or subsystem registers.
    ///
    /// This value of this key matches the vendor and device ID (0x00) register, or the subsystem register (0x2c).
    ///
    /// ## Availability
    /// * macOS 10.15.4+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "IOPCIMatch", skip_serializing_if = "Option::is_none")]
    pub pci_match: Option<String>,
    /// A key to match PCI devices by vendor and device ID registers.
    ///
    /// This value of this key matches the vendor and device ID (0x00) register.
    ///
    /// ## Availability
    /// * macOS 10.15.4+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "IOPCIPrimaryMatch", skip_serializing_if = "Option::is_none")]
    pub primary_match: Option<String>,
    /// A key to match PCI devices by subsystem vendor ID and device ID registers.
    ///
    /// This value of this key matches the subsystem register (0x2c).
    ///
    /// ## Availability
    /// * macOS 10.15.4+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename = "IOPCISecondaryMatch",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_match: Option<String>,
    /// A key to match PCI devices by class code register.
    ///
    /// This value of this key matches the class code register (0x08). The default mask for this register is 0xffffff00.
    ///
    /// ## Availability
    /// * macOS 10.15.4+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "IOPCIClassMatch", skip_serializing_if = "Option::is_none")]
    pub class_match: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct DriverkitTransportUsb {
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename = "bConfigurationValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_value: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bDeviceClass", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bDeviceProtocol", skip_serializing_if = "Option::is_none")]
    pub device_protocol: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bDeviceSubClass", skip_serializing_if = "Option::is_none")]
    pub device_sub_class: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bInterfaceClass", skip_serializing_if = "Option::is_none")]
    pub interface_class: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bInterfaceNumber", skip_serializing_if = "Option::is_none")]
    pub interface_number: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bInterfaceProtocol", skip_serializing_if = "Option::is_none")]
    pub interface_protocol: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bInterfaceSubClass", skip_serializing_if = "Option::is_none")]
    pub interface_sub_class: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "bcdDevice", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "idProduct", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "idProductArray", skip_serializing_if = "Option::is_none")]
    pub product_array: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "idProductMask", skip_serializing_if = "Option::is_none")]
    pub product_mask: Option<String>,
    /// ## Availability
    /// * macOS 10.15+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(rename = "idVendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<i32>,
}

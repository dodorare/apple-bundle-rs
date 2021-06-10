use serde::{Deserialize, Serialize};

/// Wireless Interfaces
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct WirelessInterfaces {
    /// A Boolean value indicating whether your app can access information about the
    /// connected Wi-Fi network.
    ///
    /// This key indicates whether your app may use the CNCopyCurrentNetworkInfo function
    /// to obtain information about the current Wi-Fi network.
    ///
    /// To add this entitlement to your app, enable the Access WiFi Information capability
    /// in Xcode.
    ///
    /// ## Availability
    /// * iOS 12.0+
    ///
    /// ## Framework
    /// * System Configuration
    #[serde(
        rename = "com.apple.developer.networking.wifi-info",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_wifi_information: Option<bool>,
    /// A Boolean value that indicates whether your app may configure MFi Wi-Fi
    /// accessories.
    ///
    /// This key indicates whether your app may configure third-party hardware accessories
    /// that use Apple's MFi licensed technology to connect to Apple devices.
    ///
    /// To add this entitlement to your app, enable the Wireless Accessory Configuration
    /// capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 3.0+
    ///
    /// ## Framework
    /// * External Accessory
    #[serde(
        rename = "com.apple.external-accessory.wireless-configuration",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub wireless_accessory_configuration: Option<bool>,
    /// A Boolean value indicating whether your app may use Multipath protocols to
    /// seamlessly transition between Wi-Fi and cellular networks.
    ///
    /// This key Indicates whether your app may use Multipath protocols, such as Multipath
    /// TCP, to smoothly hand over traffic from one interface to another.
    ///
    /// To add this entitlement to your app, enable the Multipath capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 3.0+
    ///
    /// ## Framework
    /// * Foundation
    #[serde(
        rename = "com.apple.developer.networking.multipath",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multipath: Option<bool>,
    /// A Boolean value indicating whether your app can use the hotspot manager to
    /// configure Wi-Fi networks.
    ///
    /// This key indicates whether your app may use the NEHotspotConfigurationManager and
    /// NEHotspotConfiguration classes to configure Wi-Fi networks.
    ///
    /// To add this entitlement to your app, enable the Hotspot Configuration capability
    /// in Xcode.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Network Extension
    #[serde(
        rename = "com.apple.developer.networking.HotspotConfiguration",
        serialize_with = "crate::serialize_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub hotspot_configuration: Option<bool>,
    /// The Near Field Communication data formats an app can read.
    ///
    /// To add this entitlement to your app, enable the Near Field Communication Tag
    /// Reading capability in Xcode.
    ///
    /// ## Availability
    /// * iOS 11.0+
    ///
    /// ## Framework
    /// * Core NFC
    #[serde(
        rename = "com.apple.developer.nfc.readersession.formats",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_vec_enum_option"
    )]
    pub near_field_communication_tag_reader_session_formats:
        Option<Vec<NearFieldCommunicationTagReaderSessionFormats>>,
}

/// Near Field Communication Tag Reader Session Formats
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NearFieldCommunicationTagReaderSessionFormats {
    /// Allows read and write access to a tag using NFCTagReaderSession.
    #[serde(rename = "TAG")]
    Tag,
}

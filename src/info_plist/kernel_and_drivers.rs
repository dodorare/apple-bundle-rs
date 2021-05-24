use super::DefaultDictionary;
/// # Kernel and Drivers
///
/// Configure device drivers provided by the app.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DriverPersonalities {
    /// One or more groups of attributes that tell the system about the devices your driver supports.
    ///
    /// This key contains a dictionary of driver personalities, each of which specifies how to pair the driver to a device.
    /// Each key in the dictionary is a string you designate as the name of a specific personality, and the system doesn’t use your key names internally.
    /// The value of each key is a dictionary of attributes that describe the specific device to match with the driver.
    /// Thus, each key and dictionary combination represents a single personality of the driver.
    /// The system uses these personalities to match the driver to an attached device.
    ///
    /// During the matching process, the system compares the attributes in each personality dictionary to data it obtained from the attached device.
    /// For example, if the personality dictionary includes the VendorID key, the system compares that key to the vendor information from the device.
    /// The system picks the driver that is compatible with the device and provides the best overall match.
    /// It then uses additional information from the personality dictionary to load and run the driver.
    ///
    /// All personality dictionaries must include the following keys:
    /// * CFBundleIdentifier
    /// * IOProviderClass
    /// * IOClass
    ///
    /// Include any of the following keys in your personality dictionary to customize the match criteria:
    /// * IOPropertyMatch
    /// * IONameMatch
    /// * IOResourceMatch
    /// * IOParentMatch
    /// * IOPathMatch
    /// * IOMatchCategory
    /// * Device-specific keys, such as DeviceUsagePairs, VendorID, or ProductID.
    /// See a specific IOService subclass for information about the keys it supports.
    ///
    /// Include one of more of the following keys to specify how to load your driver’s code:
    /// * IOUserClass
    /// * IOUserServerName
    /// * IOUserClientClass
    ///
    /// Use the following keys to further customize your driver’s behavior:
    /// * IOMatchDefer. Set the value of this key to true to defer the matching process until after kextd starts.
    /// * IOUserServerOneProcess. Set the value of this key to true to run your DriverKit services in one process.
    /// If the key is missing or its value is false, the system creates a separate process for each service.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOKitPersonalities"),
        skip_serializing_if = "Option::is_none"
    )]
    pub kit_personalities: Option<KitPersonalities>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KitPersonalities {
    /// The name of your driver’s main class, which is the entry point for interacting with your driver’s code.
    ///
    /// Include this key only in the personality dictionary of a DriverKit extension, and use it to specify the name of the custom IOService subclass that provides your driver’s behavior.
    /// When it’s time to load your driver, the system instantiates the specified class and begins the initialization and startup processes.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "IOUserClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_class: Option<String>,
    /// The name of the class that your driver expects to provide the implementation for its provider object.
    ///
    /// The value of this key is a string that contains the name of an IOService subclass.
    /// This class corresponds to the provider object that the system passes to your IOService subclass at startup.
    /// (For a kernel extension, the system passes the provider object to the start method of your IOService subclass.
    /// For a DriverKit extension, the system passes it to the Start method of your IOService subclass.)
    /// Use the provider object in your driver you receive to communicate with the underlying device.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOProviderClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_class: Option<String>,
    /// The name of the class to instantiate from your driver.
    ///
    /// The value of this key is a string that contains the name of a custom IOService subclass in your driver.
    /// When the system successfully matches one of your driver’s personalities to a device, it instantiates the class in this key and calls its start method.
    ///
    /// For the personalities in a DriverKit extension, specify the value IOUserService unless otherwise directed by the documentation.
    /// For example, the IOUserHIDEventService class expects you to specify the value AppleUserHIDEventService.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(rename(serialize = "IOClass"), skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// The name of the class to instantiate when the system requires a client connection to the driver.
    ///
    /// The value of this key is a string that contains the name of an IOService subclass in your driver.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOUserClientClass"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_client_class: Option<String>,
    /// The name that the system uses to facilitate communication between your driver and other clients.
    ///
    /// Typically, you set the value of this key to your kext or DriverKit extension’s bundle identifier.
    /// The system registers your driver under the specified server name, and uses that name to facilitate communications between your driver and other clients, including the kernel itself.
    ///
    /// ## Availability
    /// * macOS 10.14+
    ///
    /// ## Framework
    /// * DriverKit
    #[serde(
        rename(serialize = "IOUserServerName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub user_server_name: Option<String>,
    /// The device-specific keys the system must match in order to use your driver.
    ///
    /// The value of this key is a dictionary of device-specific keys and values to use during the matching process.
    /// For the system to match the driver personality to a device, all keys in the dictionary must be present in the device, and all values must exactly match the device-provided values.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOPropertyMatch"),
        skip_serializing_if = "Option::is_none"
    )]
    pub property_match: Option<DefaultDictionary>,
    /// One or more strings that contain the names of possible provider objects in the system registry.
    ///
    /// The value of this key is a string or an array of strings.
    /// The system begins the matching process with a provider object, and looks for additional drivers or nubs that support that provider object.
    /// When this key is present, the system compares its values to the provider object’s name.
    /// (It also compares the strings to the provider’s compatible and device_type properties.)
    /// If it doesn’t find any matches, the system doesn’t match the driver to the provider object.
    ///
    /// The default name of a provider object is its class name, but providers may register a custom name.
    /// For more information about how to set or get information for registered services, see IORegistryEntry.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IONameMatch"),
        skip_serializing_if = "Option::is_none"
    )]
    pub name_match: Option<Vec<String>>,
    /// One or more system-specific or device-specific resources that your driver requires.
    ///
    /// The value of this key is a string or an array of strings.
    /// Each string contains the name of a resource that must be published in the global resource list before the system loads the driver.
    /// For example, specify IOBSD to prevent the system from loading your driver until after the BSD kernel is available.
    ///
    /// To access the list of global resources, call the getResourceService method of IOService.
    /// To publish custom resources from your driver, call the publishResource method.
    ///
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOResourceMatch"),
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_match: Option<String>,
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOParentMatch"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_match: Option<DefaultDictionary>,
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOPathMatch"),
        skip_serializing_if = "Option::is_none"
    )]
    pub path_match: Option<String>,
    /// ## Availability
    /// * macOS 10.0+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "IOMatchCategory"),
        skip_serializing_if = "Option::is_none"
    )]
    pub match_category: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KextDependencies {
    /// Specify a previous version for the current driver, or the driver’s current version.
    /// Format this string the same way you format the value of the CFBundleVersion key.
    /// The combination of this value and the value in the CFBundleVersion key define the range of versions that offers the same level of compatibility.
    /// Dependent drivers use this information to determine if they are compatible with the driver.
    /// For example, if the driver’s current version is 10.0, and you set the value of this key to 5.0, a driver that depends on version 7.0 can successfully use the current driver.
    ///
    /// When you change your driver in a way that breaks compatibility with your old code, update the value of this key.
    /// At that time, set the new value to the current version of your driver.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "OSBundleCompatibleVersion"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_compatible_version: Option<String>,
    /// The drivers that the system must load before your driver.
    ///
    /// Use this key to specify other drivers that your driver depends on.
    /// For example, specify any drivers that contain symbols your driver creates or uses at startup.
    /// The system loads the drivers in this list before it attempts to load your driver.
    /// If the system fails to resolve the dependencies or load the corresponding libraries, the kernel doesn’t load your driver.
    ///
    /// Each key in the dictionary is the bundle identifier of another driver, and the value is a string that contains the minimum version of the driver you require.
    /// Your driver must be compatible with the specified version of the other driver.
    ///
    /// Don’t include this key for codeless kexts.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * Kernel
    #[serde(
        rename(serialize = "OSBundleLibraries"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bundle_libraries: Option<DefaultDictionary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ThunderboltCompatibility {
    /// A Boolean value that indicates whether your driver supports Thunderbolt devices.
    ///
    /// Include this key in the personality dictionary of your driver if that personality supports Thunderbolt devices.
    ///
    /// ## Availability
    /// * macOS 10.10+
    ///
    /// ## Framework
    /// * PCIDriverKit
    #[serde(
        rename(serialize = "IOPCITunnelCompatible"),
        skip_serializing_if = "Option::is_none"
    )]
    pub tunnel_compatible: Option<bool>,
}

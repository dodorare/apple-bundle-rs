//! # Apple Bundle Resources
//!
//! Resources located in an app, framework, or plugin bundle.
//!
//! A bundle is a directory with a standardized hierarchical structure that holds
//! executable code and the resources used by that code. The bundle contains resources
//! that may be accessed at runtime, such as images, audio files, user interface files,
//! and property lists.
//!
//! Official documentation: https://developer.apple.com/documentation/bundleresources

/// Entitlements
pub mod entitlements;
/// Information Property List
pub mod info_plist;
/// Prelude
pub mod prelude {
    pub use super::entitlements::prelude::*;
    pub use super::info_plist::prelude::*;
    #[cfg(feature = "plist")]
    pub use plist;
}
#[cfg(feature = "plist")]
pub use plist::{
    self, from_bytes, from_file, from_reader, from_reader_xml, to_file_binary, to_file_xml,
    to_writer_binary, to_writer_xml,
};

use serde::{ser::SerializeSeq, Serialize, Serializer};

fn serialize_enum_option<S: Serializer, T: Serialize>(
    value: &Option<T>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&serde_plain::to_string(value).unwrap())
}

fn serialize_vec_enum_option<S: Serializer, T: Serialize>(
    value: &Option<Vec<T>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(ref val) => {
            let mut seq = s.serialize_seq(Some(val.len()))?;
            for element in val.iter() {
                seq.serialize_element(&serde_plain::to_string(element).unwrap())?;
            }
            seq.end()
        }
        None => panic!("unsupported"),
    }
}

fn serialize_option<S, T>(value: &Option<T>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    value
        .as_ref()
        .expect(r#"`serialize_option` must be used with `skip_serializing_if = "Option::is_none"`"#)
        .serialize(ser)
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    pub const PLIST_FILE_NAME: &str = "Info.plist";

    pub const PLIST_TEST_EXAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.business</string>
    <key>CFBundleIdentifier</key>
    <string>com.test.test-id</string>
    <key>CFBundleName</key>
    <string>Test</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>1.0</string>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UIRequiresFullScreen</key>
    <false/>
    <key>CFBundleExecutable</key>
    <string>test</string>
</dict>
</plist>"#;

    #[test]
    fn test_plist_equality() {
        let dir = tempfile::tempdir().unwrap();
        let properties = InfoPlist {
            localization: Localization {
                bundle_development_region: Some("en".to_owned()),
                ..Default::default()
            },
            launch: Launch {
                bundle_executable: Some("test".to_owned()),
                ..Default::default()
            },
            identification: Identification {
                bundle_identifier: "com.test.test-id".to_owned(),
                ..Default::default()
            },
            bundle_version: BundleVersion {
                bundle_version: Some("1".to_owned()),
                bundle_info_dictionary_version: Some("1.0".to_owned()),
                bundle_short_version_string: Some("1.0".to_owned()),
                ..Default::default()
            },
            naming: Naming {
                bundle_name: Some("Test".to_owned()),
                ..Default::default()
            },
            categorization: Categorization {
                bundle_package_type: Some("APPL".to_owned()),
                application_category_type: Some(AppCategoryType::Business),
            },
            launch_interface: LaunchInterface {
                launch_storyboard_name: Some("LaunchScreen".to_owned()),
                ..Default::default()
            },
            styling: Styling {
                requires_full_screen: Some(false),
                ..Default::default()
            },
            orientation: Orientation {
                supported_interface_orientations: Some(vec![
                    InterfaceOrientation::Portrait,
                    InterfaceOrientation::PortraitUpsideDown,
                    InterfaceOrientation::LandscapeLeft,
                    InterfaceOrientation::LandscapeRight,
                ]),
                ..Default::default()
            },
            ..Default::default()
        };
        // Create Info.plist file
        let file_path = dir.path().join(PLIST_FILE_NAME);
        let file = std::fs::File::create(file_path).unwrap();
        // Write to Info.plist file
        plist::to_writer_xml(file, &properties).unwrap();
        // Read Info.plist
        let file_path = dir.path().join(PLIST_FILE_NAME);
        let result = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(result, PLIST_TEST_EXAMPLE.replace("    ", "\t"));
        // Parse Info.plist
        let got_props: InfoPlist = plist::from_bytes(result.as_bytes()).unwrap();
        assert_eq!(properties, got_props);
    }
}

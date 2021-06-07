<div align="center">
<h1>Apple Bundle Resources</h1>

<a href="https://github.com/dodorare/apple-bundle-rs/actions"><img alt="CI Info" src="https://github.com/dodorare/apple-bundle-rs/workflows/CI/badge.svg"/></a>
<a href="https://crates.io/crates/apple-bundle"><img alt="Crate Info" src="https://img.shields.io/crates/v/apple-bundle.svg"/></a>
<a href="https://docs.rs/apple-bundle/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-apple-bundle"/></a>
<a href="https://crates.io/crates/apple-bundle"><img alt="Crate" src="https://img.shields.io/crates/d/apple-bundle?label=cargo%20installs"/></a>
</div>

[AppleBundleResources] serializer and deserializer for Rust. This library will also likely continue to stay up to date with the official Apple Bundle Resources specification as changes happen.

[AppleBundleResources]: https://developer.apple.com/documentation/bundleresources

Install [apple-bundle](https://crates.io/crates/apple-bundle):

```toml
# Cargo.toml
[dependencies]
apple-bundle = "*"
```

Create `Info.plist` by yourself:
```rs
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
        ..Default::default()
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
```

Or parse any `Info.plist` file:
```rs
pub const PLIST_FILE_EXAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
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
    <false />
    <key>CFBundleExecutable</key>
    <string>test</string>
</dict>
</plist>"#;
// Read from bytes
let properties: InfoPlist = plist::from_bytes(&PLIST_FILE_EXAMPLE.as_bytes()).unwrap();
// Or from file
let file_path = "/path/to/Info.plist";
let properties: InfoPlist = plist::from_file(&file_path).unwrap();
```

### License

This project is licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in toml-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

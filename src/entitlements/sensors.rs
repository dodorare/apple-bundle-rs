use serde::{Deserialize, Serialize};

/// Sensors
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Sensors {
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SensorKit
    #[serde(
        rename = "com.apple.developer.sensorkit.reader.allow",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_vec_enum_option"
    )]
    pub sensorkit_reader_allow: Option<Vec<SensorkitReaderAllow>>,
}

/// Sensor Kit Reader Allow
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SensorkitReaderAllow {
    #[serde(rename = "on-wrist")]
    OnWrist,
    #[serde(rename = "ambient-light-sensor")]
    AmbientLightSensor,
    #[serde(rename = "motion-accelerometer")]
    MotionAccelerometer,
    #[serde(rename = "motion-rotation-rate")]
    MotionRotationRate,
    #[serde(rename = "visits")]
    Visits,
    #[serde(rename = "pedometer")]
    Pedometer,
    #[serde(rename = "device-usage")]
    DeviceUsage,
    #[serde(rename = "messages-usage")]
    MessagesUsage,
    #[serde(rename = "phone-usage")]
    PhoneUsage,
    #[serde(rename = "keyboard-metrics")]
    KeyboardMetrics,
}

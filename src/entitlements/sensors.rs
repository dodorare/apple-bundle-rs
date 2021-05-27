use crate::serialize_vec_enum_option;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Sensors {
    ///
    /// ## Availability
    /// * iOS 14.0+
    ///
    /// ## Framework
    /// * SensorKit
    #[serde(
        rename(serialize = "com.apple.developer.sensorkit.reader.allow"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_vec_enum_option"
    )]
    pub sensorkit_reader_allow: Option<Vec<SensorkitReaderAllow>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum SensorkitReaderAllow {
    #[serde(rename(serialize = "on-wrist"))]
    OnWrist,
    #[serde(rename(serialize = "ambient-light-sensor"))]
    AmbientLightSensor,
    #[serde(rename(serialize = "motion-accelerometer"))]
    MotionAccelerometer,
    #[serde(rename(serialize = "motion-rotation-rate"))]
    MotionRotationRate,
    #[serde(rename(serialize = "visits"))]
    Visits,
    #[serde(rename(serialize = "pedometer"))]
    Pedometer,
    #[serde(rename(serialize = "device-usage"))]
    DeviceUsage,
    #[serde(rename(serialize = "messages-usage"))]
    MessagesUsage,
    #[serde(rename(serialize = "phone-usage"))]
    PhoneUsage,
    #[serde(rename(serialize = "keyboard-metrics"))]
    KeyboardMetrics,
}

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::capability::{
    ColorModel, ColorSettingParameters, ColorSettingState, EventPropertyParameters,
    EventPropertyState, FloatPropertyParameters, FloatPropertyState, ModeParameters, ModeState,
    OnOffParameters, OnOffState, RangeParameters, RangeState, TemperatureRange, ToggleParameters,
    ToggleState, VideoStreamParameters,
};

/// Device type, corresponding to `devices.types.*` values in the Yandex Smart Home API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// `devices.types.light`
    Light,
    /// `devices.types.socket`
    Socket,
    /// `devices.types.switch`
    Switch,
    /// `devices.types.thermostat.ac`
    ThermostatAc,
    /// `devices.types.thermostat`
    Thermostat,
    /// `devices.types.media_device.tv`
    MediaDeviceTv,
    /// `devices.types.media_device.tv_box`
    MediaDeviceTvBox,
    /// `devices.types.media_device.receiver`
    MediaDeviceReceiver,
    /// `devices.types.media_device`
    MediaDevice,
    /// `devices.types.cooking.coffee_maker`
    CookingCoffeeMaker,
    /// `devices.types.cooking.kettle`
    CookingKettle,
    /// `devices.types.cooking.multicooker`
    CookingMulticooker,
    /// `devices.types.cooking`
    Cooking,
    /// `devices.types.vacuum_cleaner`
    VacuumCleaner,
    /// `devices.types.washing_machine`
    WashingMachine,
    /// `devices.types.dishwasher`
    Dishwasher,
    /// `devices.types.iron`
    Iron,
    /// `devices.types.humidifier`
    Humidifier,
    /// `devices.types.purifier`
    Purifier,
    /// `devices.types.fan`
    Fan,
    /// `devices.types.pet_drinking_fountain`
    PetDrinkingFountain,
    /// `devices.types.pet_feeder`
    PetFeeder,
    /// `devices.types.sensor`
    Sensor,
    /// Any device type not yet covered by a named variant
    Other(String),
}

impl Serialize for DeviceType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Light => "devices.types.light",
            Self::Socket => "devices.types.socket",
            Self::Switch => "devices.types.switch",
            Self::ThermostatAc => "devices.types.thermostat.ac",
            Self::Thermostat => "devices.types.thermostat",
            Self::MediaDeviceTv => "devices.types.media_device.tv",
            Self::MediaDeviceTvBox => "devices.types.media_device.tv_box",
            Self::MediaDeviceReceiver => "devices.types.media_device.receiver",
            Self::MediaDevice => "devices.types.media_device",
            Self::CookingCoffeeMaker => "devices.types.cooking.coffee_maker",
            Self::CookingKettle => "devices.types.cooking.kettle",
            Self::CookingMulticooker => "devices.types.cooking.multicooker",
            Self::Cooking => "devices.types.cooking",
            Self::VacuumCleaner => "devices.types.vacuum_cleaner",
            Self::WashingMachine => "devices.types.washing_machine",
            Self::Dishwasher => "devices.types.dishwasher",
            Self::Iron => "devices.types.iron",
            Self::Humidifier => "devices.types.humidifier",
            Self::Purifier => "devices.types.purifier",
            Self::Fan => "devices.types.fan",
            Self::PetDrinkingFountain => "devices.types.pet_drinking_fountain",
            Self::PetFeeder => "devices.types.pet_feeder",
            Self::Sensor => "devices.types.sensor",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for DeviceType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "devices.types.light" => Self::Light,
            "devices.types.socket" => Self::Socket,
            "devices.types.switch" => Self::Switch,
            // More-specific subtypes must be matched before their prefix
            "devices.types.thermostat.ac" => Self::ThermostatAc,
            "devices.types.thermostat" => Self::Thermostat,
            "devices.types.media_device.tv" => Self::MediaDeviceTv,
            "devices.types.media_device.tv_box" => Self::MediaDeviceTvBox,
            "devices.types.media_device.receiver" => Self::MediaDeviceReceiver,
            "devices.types.media_device" => Self::MediaDevice,
            "devices.types.cooking.coffee_maker" => Self::CookingCoffeeMaker,
            "devices.types.cooking.kettle" => Self::CookingKettle,
            "devices.types.cooking.multicooker" => Self::CookingMulticooker,
            "devices.types.cooking" => Self::Cooking,
            "devices.types.vacuum_cleaner" => Self::VacuumCleaner,
            "devices.types.washing_machine" => Self::WashingMachine,
            "devices.types.dishwasher" => Self::Dishwasher,
            "devices.types.iron" => Self::Iron,
            "devices.types.humidifier" => Self::Humidifier,
            "devices.types.purifier" => Self::Purifier,
            "devices.types.fan" => Self::Fan,
            "devices.types.pet_drinking_fountain" => Self::PetDrinkingFountain,
            "devices.types.pet_feeder" => Self::PetFeeder,
            "devices.types.sensor" => Self::Sensor,
            _ => Self::Other(s),
        })
    }
}

/// Capability type — kept for use in action result responses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityType {
    /// `devices.capabilities.on_off`
    OnOff,
    /// `devices.capabilities.color_setting`
    ColorSetting,
    /// `devices.capabilities.range`
    Range,
    /// `devices.capabilities.mode`
    Mode,
    /// `devices.capabilities.toggle`
    Toggle,
    /// `devices.capabilities.video_stream`
    VideoStream,
    /// Any capability type not yet covered by a named variant
    Other(String),
}

impl Serialize for CapabilityType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::OnOff => "devices.capabilities.on_off",
            Self::ColorSetting => "devices.capabilities.color_setting",
            Self::Range => "devices.capabilities.range",
            Self::Mode => "devices.capabilities.mode",
            Self::Toggle => "devices.capabilities.toggle",
            Self::VideoStream => "devices.capabilities.video_stream",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for CapabilityType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "devices.capabilities.on_off" => Self::OnOff,
            "devices.capabilities.color_setting" => Self::ColorSetting,
            "devices.capabilities.range" => Self::Range,
            "devices.capabilities.mode" => Self::Mode,
            "devices.capabilities.toggle" => Self::Toggle,
            "devices.capabilities.video_stream" => Self::VideoStream,
            _ => Self::Other(s),
        })
    }
}

/// Capability of a device.
///
/// Each variant encodes both the capability type (via the `#[serde(tag = "type")]` field)
/// and its typed `parameters` and `state`.  Unknown types deserialize to `Unknown` and
/// their payload is discarded.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DeviceCapability {
    /// Remote power control (`devices.capabilities.on_off`).
    #[serde(rename = "devices.capabilities.on_off")]
    OnOff {
        /// Platform is notified when the state changes.
        reportable: bool,
        /// Current state can be queried.
        retrievable: bool,
        /// Capability parameters.
        parameters: OnOffParameters,
        /// Current state (`None` if never reported).
        #[serde(default)]
        state: Option<OnOffState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Colour and colour-temperature control (`devices.capabilities.color_setting`).
    #[serde(rename = "devices.capabilities.color_setting")]
    ColorSetting {
        /// Platform is notified when the state changes.
        reportable: bool,
        /// Current state can be queried.
        retrievable: bool,
        /// Supported colour models and temperature range.
        parameters: ColorSettingParameters,
        /// Current colour state (`None` if never reported).
        #[serde(default)]
        state: Option<ColorSettingState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Numeric range control (`devices.capabilities.range`).
    #[serde(rename = "devices.capabilities.range")]
    Range {
        /// Platform is notified when the state changes.
        reportable: bool,
        /// Current state can be queried.
        retrievable: bool,
        /// Instance name, unit, and allowed range.
        parameters: RangeParameters,
        /// Current value (`None` if never reported).
        #[serde(default)]
        state: Option<RangeState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Operational mode selection (`devices.capabilities.mode`).
    #[serde(rename = "devices.capabilities.mode")]
    Mode {
        /// Platform is notified when the state changes.
        reportable: bool,
        /// Current state can be queried.
        retrievable: bool,
        /// Instance name and supported modes.
        parameters: ModeParameters,
        /// Active mode (`None` if never reported).
        #[serde(default)]
        state: Option<ModeState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Binary feature toggle (`devices.capabilities.toggle`).
    #[serde(rename = "devices.capabilities.toggle")]
    Toggle {
        /// Platform is notified when the state changes.
        reportable: bool,
        /// Current state can be queried.
        retrievable: bool,
        /// Instance name.
        parameters: ToggleParameters,
        /// Current toggle state (`None` if never reported).
        #[serde(default)]
        state: Option<ToggleState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Live video stream (`devices.capabilities.video_stream`).
    #[serde(rename = "devices.capabilities.video_stream")]
    VideoStream {
        /// Platform is notified when the state changes.
        reportable: bool,
        /// Current state can be queried (always `false` for video streams).
        retrievable: bool,
        /// Supported streaming protocols.
        parameters: VideoStreamParameters,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Any capability type not recognised by this library version.
    #[serde(other)]
    Unknown,
}

/// Capability of a device group (no `reportable` or `last_updated` per the API spec).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum GroupCapability {
    /// Remote power control (`devices.capabilities.on_off`).
    #[serde(rename = "devices.capabilities.on_off")]
    OnOff {
        /// Current state can be queried.
        retrievable: bool,
        /// Capability parameters.
        parameters: OnOffParameters,
        /// Current state (`None` if inconsistent across group members).
        #[serde(default)]
        state: Option<OnOffState>,
    },
    /// Colour and colour-temperature control (`devices.capabilities.color_setting`).
    #[serde(rename = "devices.capabilities.color_setting")]
    ColorSetting {
        /// Current state can be queried.
        retrievable: bool,
        /// Supported colour models and temperature range.
        parameters: ColorSettingParameters,
        /// Current colour state (`None` if inconsistent across group members).
        #[serde(default)]
        state: Option<ColorSettingState>,
    },
    /// Numeric range control (`devices.capabilities.range`).
    #[serde(rename = "devices.capabilities.range")]
    Range {
        /// Current state can be queried.
        retrievable: bool,
        /// Instance name, unit, and allowed range.
        parameters: RangeParameters,
        /// Current value (`None` if inconsistent across group members).
        #[serde(default)]
        state: Option<RangeState>,
    },
    /// Operational mode selection (`devices.capabilities.mode`).
    #[serde(rename = "devices.capabilities.mode")]
    Mode {
        /// Current state can be queried.
        retrievable: bool,
        /// Instance name and supported modes.
        parameters: ModeParameters,
        /// Active mode (`None` if inconsistent across group members).
        #[serde(default)]
        state: Option<ModeState>,
    },
    /// Binary feature toggle (`devices.capabilities.toggle`).
    #[serde(rename = "devices.capabilities.toggle")]
    Toggle {
        /// Current state can be queried.
        retrievable: bool,
        /// Instance name.
        parameters: ToggleParameters,
        /// Current toggle state (`None` if inconsistent across group members).
        #[serde(default)]
        state: Option<ToggleState>,
    },
    /// Live video stream (`devices.capabilities.video_stream`).
    #[serde(rename = "devices.capabilities.video_stream")]
    VideoStream {
        /// Current state can be queried (always `false` for video streams).
        retrievable: bool,
        /// Supported streaming protocols.
        parameters: VideoStreamParameters,
    },
    /// Any capability type not recognised by this library version.
    #[serde(other)]
    Unknown,
}

/// Property (sensor / read-only state) of a device.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DeviceProperty {
    /// Numeric sensor reading (`devices.properties.float`).
    #[serde(rename = "devices.properties.float")]
    Float {
        /// Platform is notified when the value changes.
        reportable: bool,
        /// Current value can be queried.
        retrievable: bool,
        /// Sensor type and unit.
        parameters: FloatPropertyParameters,
        /// Last reported value (`None` if never reported).
        #[serde(default)]
        state: Option<FloatPropertyState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Discrete event sensor (`devices.properties.event`).
    #[serde(rename = "devices.properties.event")]
    Event {
        /// Platform is notified when the event fires.
        reportable: bool,
        /// Current state can be queried.
        retrievable: bool,
        /// Sensor type and possible event values.
        parameters: EventPropertyParameters,
        /// Last reported event (`None` if never reported).
        #[serde(default)]
        state: Option<EventPropertyState>,
        /// Unix timestamp of the last state update.
        last_updated: f64,
    },
    /// Any property type not recognised by this library version.
    #[serde(other)]
    Unknown,
}

/// Colour control modes a device supports, derived from its `color_setting` parameters.
#[derive(Debug, Clone)]
pub enum ColorMode {
    /// RGB via the `rgb` instance (packed 24-bit integer).
    Rgb,
    /// HSV via the `hsv` instance.
    Hsv,
    /// Colour temperature only via `temperature_k`.
    Temperature(TemperatureRange),
    /// RGB and colour temperature.
    RgbAndTemperature(TemperatureRange),
    /// HSV and colour temperature.
    HsvAndTemperature(TemperatureRange),
}

/// Inspect a device's `color_setting` capability and return the colour mode(s) it supports,
/// or `None` if the device has no usable colour control.
pub fn color_mode(device: &Device) -> Option<ColorMode> {
    let params = device.capabilities.iter().find_map(|c| match c {
        DeviceCapability::ColorSetting { parameters, .. } => Some(parameters),
        _ => None,
    })?;

    Some(match (&params.color_model, &params.temperature_k) {
        (Some(ColorModel::Rgb), Some(r)) => ColorMode::RgbAndTemperature(r.clone()),
        (Some(ColorModel::Rgb), None) => ColorMode::Rgb,
        (Some(ColorModel::Hsv), Some(r)) => ColorMode::HsvAndTemperature(r.clone()),
        (Some(ColorModel::Hsv), None) => ColorMode::Hsv,
        (_, Some(r)) => ColorMode::Temperature(r.clone()),
        _ => return None,
    })
}

/// A device in the user's smart home
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    /// Unique device ID
    pub id: String,
    /// User-defined name
    pub name: String,
    /// Additional user-defined names
    pub aliases: Vec<String>,
    /// Device type
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    /// ID in the manufacturer's cloud
    pub external_id: String,
    /// ID of the manufacturer's skill
    pub skill_id: String,
    /// ID of the household this device belongs to
    pub household_id: String,
    /// Room ID (`None` if not assigned to a room)
    pub room: Option<String>,
    /// IDs of groups this device belongs to
    pub groups: Vec<String>,
    /// Device capabilities
    pub capabilities: Vec<DeviceCapability>,
    /// Device properties (sensors / read-only states)
    pub properties: Vec<DeviceProperty>,
}

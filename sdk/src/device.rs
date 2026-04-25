use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

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

/// Capability type, corresponding to `devices.capabilities.*` values
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

/// Property type, corresponding to `devices.properties.*` values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    /// `devices.properties.float` — numeric sensor reading
    Float,
    /// `devices.properties.event` — discrete event state
    Event,
    /// Any property type not yet covered by a named variant
    Other(String),
}

impl Serialize for PropertyType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Float => "devices.properties.float",
            Self::Event => "devices.properties.event",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for PropertyType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "devices.properties.float" => Self::Float,
            "devices.properties.event" => Self::Event,
            _ => Self::Other(s),
        })
    }
}

/// Capability of a device (includes `reportable` and `last_updated`, unlike group capabilities)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceCapability {
    /// Capability type
    #[serde(rename = "type")]
    pub capability_type: CapabilityType,
    /// Whether the platform is notified of state changes
    pub reportable: bool,
    /// Whether the current state can be queried
    pub retrievable: bool,
    /// Capability-specific parameters
    pub parameters: Value,
    /// Current state (`None` if the state has never been set)
    pub state: Option<Value>,
    /// Unix timestamp of the last state update
    pub last_updated: f64,
}

/// Property (sensor / read-only state) of a device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceProperty {
    /// Property type
    #[serde(rename = "type")]
    pub property_type: PropertyType,
    /// Whether the platform is notified of state changes
    pub reportable: bool,
    /// Whether the current state can be queried
    pub retrievable: bool,
    /// Property-specific parameters
    pub parameters: Value,
    /// Current state (`None` if the state has never been set)
    pub state: Option<Value>,
    /// Unix timestamp of the last state update
    pub last_updated: f64,
}

/// Capability of a device group (no `reportable` or `last_updated` per the API spec)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupCapability {
    /// Capability type
    #[serde(rename = "type")]
    pub capability_type: CapabilityType,
    /// Whether the current state can be queried
    pub retrievable: bool,
    /// Capability-specific parameters
    pub parameters: Value,
    /// Current state; `None` when inconsistent across group members
    pub state: Option<Value>,
}

/// Kelvin range reported by a device's `color_setting` capability.
#[derive(Debug, Clone)]
pub struct TemperatureRange {
    /// Minimum supported colour temperature in Kelvin.
    pub min: u32,
    /// Maximum supported colour temperature in Kelvin.
    pub max: u32,
}

/// What colour control mode(s) a device actually supports.
#[derive(Debug, Clone)]
pub enum ColorMode {
    /// 24-bit packed RGB via the `rgb` instance.
    Rgb,
    /// HSV via the `hsv` instance (`{h, s, v}`).
    Hsv,
    /// Colour temperature only, via the `temperature_k` instance.
    Temperature(TemperatureRange),
    /// Both RGB and colour temperature.
    RgbAndTemperature(TemperatureRange),
    /// Both HSV and colour temperature.
    HsvAndTemperature(TemperatureRange),
}

/// Inspect a device's `color_setting` capability parameters and return
/// which mode(s) it supports, or `None` if the device has no usable colour control.
pub fn color_mode(device: &Device) -> Option<ColorMode> {
    let cap = device
        .capabilities
        .iter()
        .find(|c| c.capability_type == CapabilityType::ColorSetting)?;

    let model = cap
        .parameters
        .get("color_model")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let temp = cap.parameters.get("temperature_k").and_then(|t| {
        Some(TemperatureRange {
            min: t.get("min")?.as_u64()? as u32,
            max: t.get("max")?.as_u64()? as u32,
        })
    });

    Some(match (model, temp) {
        ("rgb", Some(r)) => ColorMode::RgbAndTemperature(r),
        ("rgb", None)    => ColorMode::Rgb,
        ("hsv", Some(r)) => ColorMode::HsvAndTemperature(r),
        ("hsv", None)    => ColorMode::Hsv,
        (_,     Some(r)) => ColorMode::Temperature(r),
        _                => return None,
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

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ─── OnOff ───────────────────────────────────────────────────────────────────

/// Parameters for the `on_off` capability.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OnOffParameters {
    /// When `true` the device handles on and off as independent commands.
    #[serde(default)]
    pub split: bool,
}

/// State for the `on_off` capability.
///
/// JSON: `{"instance": "on", "value": <bool>}`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "instance", content = "value")]
pub enum OnOffState {
    /// Power state — `true` = on, `false` = off.
    #[serde(rename = "on")]
    On(bool),
}

// ─── ColorSetting ────────────────────────────────────────────────────────────

/// Colour model used by a `color_setting` capability.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ColorModel {
    /// 24-bit packed RGB value.
    #[serde(rename = "rgb")]
    Rgb,
    /// Hue / saturation / value.
    #[serde(rename = "hsv")]
    Hsv,
    /// Any colour model not covered by a named variant.
    #[serde(other)]
    Other,
}

/// Kelvin range supported by a `color_setting` capability.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemperatureRange {
    /// Minimum supported colour temperature in Kelvin.
    pub min: u32,
    /// Maximum supported colour temperature in Kelvin.
    pub max: u32,
}

/// Parameters for the `color_setting` capability.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ColorSettingParameters {
    /// Colour model the device uses for the `rgb` or `hsv` instance.
    pub color_model: Option<ColorModel>,
    /// Kelvin range for the `temperature_k` instance, if supported.
    pub temperature_k: Option<TemperatureRange>,
}

/// HSV colour value.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HsvColor {
    /// Hue (0 – 360).
    pub h: u16,
    /// Saturation (0 – 100).
    pub s: u8,
    /// Value / brightness (0 – 100).
    pub v: u8,
}

/// State for the `color_setting` capability.
///
/// JSON: `{"instance": "rgb"|"hsv"|"temperature_k"|"scene", "value": <typed>}`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "instance", content = "value")]
pub enum ColorSettingState {
    /// 24-bit packed RGB colour (e.g. `0xFF8800`).
    #[serde(rename = "rgb")]
    Rgb(u32),
    /// HSV colour.
    #[serde(rename = "hsv")]
    Hsv(HsvColor),
    /// Colour temperature in Kelvin.
    #[serde(rename = "temperature_k")]
    TemperatureK(u32),
    /// Named colour scene.
    #[serde(rename = "scene")]
    Scene(String),
}

// ─── Range ───────────────────────────────────────────────────────────────────

/// Function controlled by a `range` capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeInstance {
    /// Screen or light brightness (0 – 100 %).
    Brightness,
    /// TV/radio channel number.
    Channel,
    /// Humidity level (0 – 100 %).
    Humidity,
    /// Aperture or opening level, e.g. blinds or a window (0 – 100 %).
    Open,
    /// Target temperature (°C or K depending on `unit`).
    Temperature,
    /// Audio volume.
    Volume,
    /// Any instance not covered by a named variant.
    Other(String),
}

impl Serialize for RangeInstance {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Brightness => "brightness",
            Self::Channel => "channel",
            Self::Humidity => "humidity",
            Self::Open => "open",
            Self::Temperature => "temperature",
            Self::Volume => "volume",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for RangeInstance {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "brightness" => Self::Brightness,
            "channel" => Self::Channel,
            "humidity" => Self::Humidity,
            "open" => Self::Open,
            "temperature" => Self::Temperature,
            "volume" => Self::Volume,
            _ => Self::Other(s),
        })
    }
}

/// Allowed value range for a `range` capability instance.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangeSpec {
    /// Minimum allowed value.
    pub min: f64,
    /// Maximum allowed value.
    pub max: f64,
    /// Smallest meaningful step between values.
    #[serde(default = "range_precision_default")]
    pub precision: f64,
}

fn range_precision_default() -> f64 {
    1.0
}

fn range_random_access_default() -> bool {
    true
}

/// Parameters for the `range` capability.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangeParameters {
    /// Function controlled by this instance.
    pub instance: RangeInstance,
    /// Unit of measurement (e.g. `"unit.percent"`, `"unit.temperature.celsius"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Whether the device accepts arbitrary target values (not just increments).
    #[serde(default = "range_random_access_default")]
    pub random_access: bool,
    /// Value bounds and precision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeSpec>,
}

/// Device state for a `range` capability instance, read from the API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangeState {
    /// Function controlled by this instance.
    pub instance: RangeInstance,
    /// Current value.
    pub value: f64,
}

/// Desired state sent when issuing a `range` action.
///
/// Adds an optional `relative` flag for incrementing/decrementing from the current value.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangeActionState {
    /// Function to control.
    pub instance: RangeInstance,
    /// Target value (absolute, or delta when `relative` is `true`).
    pub value: f64,
    /// When `true`, `value` is a delta applied to the current state.
    #[serde(default, skip_serializing_if = "is_false")]
    pub relative: bool,
}

fn is_false(b: &bool) -> bool {
    !b
}

// ─── Mode ────────────────────────────────────────────────────────────────────

/// Function controlled by a `mode` capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeInstance {
    /// Vacuum-cleaner cleaning mode.
    CleanupMode,
    /// Coffee-maker drink type.
    CoffeeMode,
    /// Dishwasher wash programme.
    Dishwashing,
    /// Fan / ventilation speed level.
    FanSpeed,
    /// Heating mode.
    Heat,
    /// Signal input source (TV, receiver, etc.).
    InputSource,
    /// Generic numbered programme.
    Program,
    /// Air-flow direction (climate appliances).
    Swing,
    /// Kettle tea type.
    TeaMode,
    /// Thermostat / AC operating mode.
    Thermostat,
    /// Ventilation mode (supply vs. extraction).
    VentilationMode,
    /// Generic work speed.
    WorkSpeed,
    /// Any instance not covered by a named variant.
    Other(String),
}

impl Serialize for ModeInstance {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::CleanupMode => "cleanup_mode",
            Self::CoffeeMode => "coffee_mode",
            Self::Dishwashing => "dishwashing",
            Self::FanSpeed => "fan_speed",
            Self::Heat => "heat",
            Self::InputSource => "input_source",
            Self::Program => "program",
            Self::Swing => "swing",
            Self::TeaMode => "tea_mode",
            Self::Thermostat => "thermostat",
            Self::VentilationMode => "ventilation_mode",
            Self::WorkSpeed => "work_speed",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for ModeInstance {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "cleanup_mode" => Self::CleanupMode,
            "coffee_mode" => Self::CoffeeMode,
            "dishwashing" => Self::Dishwashing,
            "fan_speed" => Self::FanSpeed,
            "heat" => Self::Heat,
            "input_source" => Self::InputSource,
            "program" => Self::Program,
            "swing" => Self::Swing,
            "tea_mode" => Self::TeaMode,
            "thermostat" => Self::Thermostat,
            "ventilation_mode" => Self::VentilationMode,
            "work_speed" => Self::WorkSpeed,
            _ => Self::Other(s),
        })
    }
}

/// A concrete mode value supported or active on a `mode` capability instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeKind {
    // ── Cleaning ─────────────────────────────────────────────────────────────
    /// `wet_cleaning` — mop / wet pass.
    WetCleaning,
    /// `dry_cleaning` — vacuum / dry pass.
    DryCleaning,
    /// `mixed_cleaning` — combined wet and dry.
    MixedCleaning,

    // ── Speed / fan ──────────────────────────────────────────────────────────
    /// `auto` — device chooses the level automatically.
    Auto,
    /// `eco` — reduced power / eco mode.
    Eco,
    /// `smart` — sensor-driven automatic mode.
    Smart,
    /// `turbo` — maximum burst speed.
    Turbo,
    /// `high` — high speed.
    High,
    /// `low` — low speed.
    Low,
    /// `medium` — medium speed.
    Medium,
    /// `max` — absolute maximum.
    Max,
    /// `min` — absolute minimum.
    Min,
    /// `fast` — fast speed.
    Fast,
    /// `slow` — slow speed.
    Slow,

    // ── Temperature / AC ─────────────────────────────────────────────────────
    /// `cool` — cooling mode.
    Cool,
    /// `dry` — dehumidification mode.
    Dry,
    /// `fan_only` — fan without heating or cooling.
    FanOnly,
    /// `heat` — heating mode.
    Heat,
    /// `preheat` — warm-up before main heating.
    Preheat,

    // ── Numbered programmes ───────────────────────────────────────────────────
    /// `one`
    One,
    /// `two`
    Two,
    /// `three`
    Three,
    /// `four`
    Four,
    /// `five`
    Five,
    /// `six`
    Six,
    /// `seven`
    Seven,
    /// `eight`
    Eight,
    /// `nine`
    Nine,
    /// `ten`
    Ten,

    // ── Coffee drinks ─────────────────────────────────────────────────────────
    /// `americano`
    Americano,
    /// `cappuccino`
    Cappuccino,
    /// `double_espresso`
    DoubleEspresso,
    /// `espresso`
    Espresso,
    /// `latte`
    Latte,

    // ── Tea types ────────────────────────────────────────────────────────────
    /// `black_tea`
    BlackTea,
    /// `flower_tea`
    FlowerTea,
    /// `green_tea`
    GreenTea,
    /// `herbal_tea`
    HerbalTea,
    /// `oolong_tea`
    OolongTea,
    /// `puerh_tea`
    PuerhTea,
    /// `red_tea`
    RedTea,
    /// `white_tea`
    WhiteTea,

    // ── Dishwasher programmes ─────────────────────────────────────────────────
    /// `glass` — delicate glassware cycle.
    Glass,
    /// `intensive` — heavily soiled load.
    Intensive,
    /// `pre_rinse` — rinse-only cycle.
    PreRinse,

    // ── Multicooker / oven programmes ─────────────────────────────────────────
    /// `aspic`
    Aspic,
    /// `baby_food`
    BabyFood,
    /// `baking`
    Baking,
    /// `bread`
    Bread,
    /// `boiling`
    Boiling,
    /// `cereals`
    Cereals,
    /// `cheesecake`
    Cheesecake,
    /// `deep_fryer`
    DeepFryer,
    /// `dessert`
    Dessert,
    /// `fowl`
    Fowl,
    /// `frying`
    Frying,
    /// `macaroni`
    Macaroni,
    /// `milk_porridge`
    MilkPorridge,
    /// `multicooker` — generic multi-cook programme.
    Multicooker,
    /// `pasta`
    Pasta,
    /// `pilaf`
    Pilaf,
    /// `pizza`
    Pizza,
    /// `sauce`
    Sauce,
    /// `slow_cook`
    SlowCook,
    /// `soup`
    Soup,
    /// `steam`
    Steam,
    /// `stewing`
    Stewing,
    /// `vacuum` — vacuum / sous-vide programme.
    Vacuum,
    /// `yogurt`
    Yogurt,

    // ── Air direction ─────────────────────────────────────────────────────────
    /// `horizontal` — horizontal airflow.
    Horizontal,
    /// `stationary` — fixed / no swing.
    Stationary,
    /// `vertical` — vertical airflow.
    Vertical,

    // ── Ventilation ───────────────────────────────────────────────────────────
    /// `supply_air` — fresh-air supply.
    SupplyAir,
    /// `extraction_air` — exhaust / extraction.
    ExtractionAir,

    // ── General speed ─────────────────────────────────────────────────────────
    /// `normal`
    Normal,
    /// `express` — faster-than-normal programme.
    Express,
    /// `quiet` — reduced-noise mode.
    Quiet,

    /// Any mode value not covered by a named variant.
    Other(String),
}

impl Serialize for ModeKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::WetCleaning => "wet_cleaning",
            Self::DryCleaning => "dry_cleaning",
            Self::MixedCleaning => "mixed_cleaning",
            Self::Auto => "auto",
            Self::Eco => "eco",
            Self::Smart => "smart",
            Self::Turbo => "turbo",
            Self::High => "high",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::Max => "max",
            Self::Min => "min",
            Self::Fast => "fast",
            Self::Slow => "slow",
            Self::Cool => "cool",
            Self::Dry => "dry",
            Self::FanOnly => "fan_only",
            Self::Heat => "heat",
            Self::Preheat => "preheat",
            Self::One => "one",
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
            Self::Ten => "ten",
            Self::Americano => "americano",
            Self::Cappuccino => "cappuccino",
            Self::DoubleEspresso => "double_espresso",
            Self::Espresso => "espresso",
            Self::Latte => "latte",
            Self::BlackTea => "black_tea",
            Self::FlowerTea => "flower_tea",
            Self::GreenTea => "green_tea",
            Self::HerbalTea => "herbal_tea",
            Self::OolongTea => "oolong_tea",
            Self::PuerhTea => "puerh_tea",
            Self::RedTea => "red_tea",
            Self::WhiteTea => "white_tea",
            Self::Glass => "glass",
            Self::Intensive => "intensive",
            Self::PreRinse => "pre_rinse",
            Self::Aspic => "aspic",
            Self::BabyFood => "baby_food",
            Self::Baking => "baking",
            Self::Bread => "bread",
            Self::Boiling => "boiling",
            Self::Cereals => "cereals",
            Self::Cheesecake => "cheesecake",
            Self::DeepFryer => "deep_fryer",
            Self::Dessert => "dessert",
            Self::Fowl => "fowl",
            Self::Frying => "frying",
            Self::Macaroni => "macaroni",
            Self::MilkPorridge => "milk_porridge",
            Self::Multicooker => "multicooker",
            Self::Pasta => "pasta",
            Self::Pilaf => "pilaf",
            Self::Pizza => "pizza",
            Self::Sauce => "sauce",
            Self::SlowCook => "slow_cook",
            Self::Soup => "soup",
            Self::Steam => "steam",
            Self::Stewing => "stewing",
            Self::Vacuum => "vacuum",
            Self::Yogurt => "yogurt",
            Self::Horizontal => "horizontal",
            Self::Stationary => "stationary",
            Self::Vertical => "vertical",
            Self::SupplyAir => "supply_air",
            Self::ExtractionAir => "extraction_air",
            Self::Normal => "normal",
            Self::Express => "express",
            Self::Quiet => "quiet",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for ModeKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "wet_cleaning" => Self::WetCleaning,
            "dry_cleaning" => Self::DryCleaning,
            "mixed_cleaning" => Self::MixedCleaning,
            "auto" => Self::Auto,
            "eco" => Self::Eco,
            "smart" => Self::Smart,
            "turbo" => Self::Turbo,
            "high" => Self::High,
            "low" => Self::Low,
            "medium" => Self::Medium,
            "max" => Self::Max,
            "min" => Self::Min,
            "fast" => Self::Fast,
            "slow" => Self::Slow,
            "cool" => Self::Cool,
            "dry" => Self::Dry,
            "fan_only" => Self::FanOnly,
            "heat" => Self::Heat,
            "preheat" => Self::Preheat,
            "one" => Self::One,
            "two" => Self::Two,
            "three" => Self::Three,
            "four" => Self::Four,
            "five" => Self::Five,
            "six" => Self::Six,
            "seven" => Self::Seven,
            "eight" => Self::Eight,
            "nine" => Self::Nine,
            "ten" => Self::Ten,
            "americano" => Self::Americano,
            "cappuccino" => Self::Cappuccino,
            "double_espresso" => Self::DoubleEspresso,
            "espresso" => Self::Espresso,
            "latte" => Self::Latte,
            "black_tea" => Self::BlackTea,
            "flower_tea" => Self::FlowerTea,
            "green_tea" => Self::GreenTea,
            "herbal_tea" => Self::HerbalTea,
            "oolong_tea" => Self::OolongTea,
            "puerh_tea" => Self::PuerhTea,
            "red_tea" => Self::RedTea,
            "white_tea" => Self::WhiteTea,
            "glass" => Self::Glass,
            "intensive" => Self::Intensive,
            "pre_rinse" => Self::PreRinse,
            "aspic" => Self::Aspic,
            "baby_food" => Self::BabyFood,
            "baking" => Self::Baking,
            "bread" => Self::Bread,
            "boiling" => Self::Boiling,
            "cereals" => Self::Cereals,
            "cheesecake" => Self::Cheesecake,
            "deep_fryer" => Self::DeepFryer,
            "dessert" => Self::Dessert,
            "fowl" => Self::Fowl,
            "frying" => Self::Frying,
            "macaroni" => Self::Macaroni,
            "milk_porridge" => Self::MilkPorridge,
            "multicooker" => Self::Multicooker,
            "pasta" => Self::Pasta,
            "pilaf" => Self::Pilaf,
            "pizza" => Self::Pizza,
            "sauce" => Self::Sauce,
            "slow_cook" => Self::SlowCook,
            "soup" => Self::Soup,
            "steam" => Self::Steam,
            "stewing" => Self::Stewing,
            "vacuum" => Self::Vacuum,
            "yogurt" => Self::Yogurt,
            "horizontal" => Self::Horizontal,
            "stationary" => Self::Stationary,
            "vertical" => Self::Vertical,
            "supply_air" => Self::SupplyAir,
            "extraction_air" => Self::ExtractionAir,
            "normal" => Self::Normal,
            "express" => Self::Express,
            "quiet" => Self::Quiet,
            _ => Self::Other(s),
        })
    }
}

/// Parameters for the `mode` capability.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModeParameters {
    /// Function controlled by this instance.
    pub instance: ModeInstance,
    /// Modes the device supports for this instance.
    pub modes: Vec<ModeValue>,
}

/// A single mode value entry inside `ModeParameters.modes`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModeValue {
    /// The mode.
    pub value: ModeKind,
}

/// State for the `mode` capability — used for both device state and actions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModeState {
    /// Function being reported or targeted.
    pub instance: ModeInstance,
    /// Active or desired mode.
    pub value: ModeKind,
}

// ─── Toggle ──────────────────────────────────────────────────────────────────

/// Parameters for the `toggle` capability.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToggleParameters {
    /// Function controlled by this toggle (e.g. `"backlight"`, `"mute"`).
    pub instance: String,
}

/// State for the `toggle` capability — used for both device state and actions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToggleState {
    /// Function name (e.g. `"backlight"`, `"mute"`).
    pub instance: String,
    /// `true` = enabled, `false` = disabled.
    pub value: bool,
}

// ─── VideoStream ─────────────────────────────────────────────────────────────

/// Streaming protocol supported by a `video_stream` capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamProtocol {
    /// HTTP Live Streaming.
    Hls,
    /// Any protocol not covered by a named variant.
    Other(String),
}

impl Serialize for StreamProtocol {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Hls => "hls",
            Self::Other(s) => s,
        })
    }
}

impl<'de> Deserialize<'de> for StreamProtocol {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(if s == "hls" {
            Self::Hls
        } else {
            Self::Other(s)
        })
    }
}

/// Parameters for the `video_stream` capability.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoStreamParameters {
    /// Streaming protocols the device supports.
    pub protocols: Vec<StreamProtocol>,
}

/// Value sent inside a `get_stream` action request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoStreamRequest {
    /// Protocols the client is willing to receive.
    pub protocols: Vec<StreamProtocol>,
}

/// State sent when issuing a `video_stream` action.
///
/// JSON: `{"instance": "get_stream", "value": {"protocols": [...]}}`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "instance", content = "value")]
pub enum VideoStreamActionState {
    /// Request a stream URL from the device.
    #[serde(rename = "get_stream")]
    GetStream(VideoStreamRequest),
}

// ─── Properties ──────────────────────────────────────────────────────────────

/// Parameters for a `float` property (numeric sensor reading).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FloatPropertyParameters {
    /// Sensor type (e.g. `"temperature"`, `"humidity"`).
    pub instance: String,
    /// Unit of measurement (e.g. `"unit.temperature.celsius"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// State for a `float` property.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FloatPropertyState {
    /// Sensor type (e.g. `"temperature"`).
    pub instance: String,
    /// Current sensor reading.
    pub value: f64,
}

/// A discrete event value reported by an `event` property.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventValue {
    /// Event string (e.g. `"detected"`, `"not_detected"`).
    pub value: String,
}

/// Parameters for an `event` property (discrete sensor state).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventPropertyParameters {
    /// Sensor type (e.g. `"motion"`, `"vibration"`).
    pub instance: String,
    /// All possible event values this sensor can report.
    pub events: Vec<EventValue>,
}

/// State for an `event` property.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventPropertyState {
    /// Sensor type (e.g. `"motion"`).
    pub instance: String,
    /// Last reported event (e.g. `"detected"`).
    pub value: String,
}

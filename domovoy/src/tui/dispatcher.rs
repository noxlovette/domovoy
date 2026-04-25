use yandex_home_sdk::{Device, Room};

/// Does not contain business logic, it routes actions to stores
pub struct Dispatcher {
    store: Store,
}

struct Store {
    devices: Vec<Device>,
    rooms: Vec<Room>,
    current_device: Option<Device>,
}

enum Action {
    DevicesLoaded { devices: Vec<Device> },
}

#[derive(Clone)]
pub enum ColorAction {
    Rgb(u32),
    Hsv { h: u16, s: u8, v: u8 },
    Temperature(u32),
}

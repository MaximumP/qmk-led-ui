use hidapi::{HidApi, HidDevice};

// const VENDOR_ID: u16 = 0xfeed;
// const PRODUCT_ID: u16 = 0x0042;

pub enum CurrentScreen {
    Main,
    SelectDevice,
    DeviceInfo,
    ColorPicker,
    LedModesList,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub device: Option<HidDevice>,
    api: HidApi,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
            device: None,
            api: HidApi::new().expect("Could not initialize HidApi"),
        }
    }

    pub fn devices(&self) -> impl Iterator<Item = &hidapi::DeviceInfo> {
        self.api.device_list()
    }
}

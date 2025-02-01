use hidapi::{HidApi, HidDevice};

// const VENDOR_ID: u16 = 0xfeed;
// const PRODUCT_ID: u16 = 0x0042;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    SelectDevice,
    DeviceInfo,
    ColorPicker,
    LedModesList,
}

pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub device: Option<HidDevice>,
    pub rgb_color: RGBColor,
    api: HidApi,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
            device: None,
            api: HidApi::new().expect("Could not initialize HidApi"),
            rgb_color: RGBColor {
                red: 0u8,
                green: 0u8,
                blue: 0u8,
            },
        }
    }

    pub fn devices(&self) -> impl Iterator<Item = &hidapi::DeviceInfo> {
        self.api.device_list()
    }
}

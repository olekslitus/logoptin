use std::default::Default;


/// ....
type DPI = Vec<u64>;

/// Process ID
type PID = u32;


/// Main config for the logi service
struct Config {
    /// Id of devices to be ignored
    ignored: Vec<PID>,
    /// Config for devices
    devices: Vec<Device>,
    /// Number of threads for service to use, default = 4
    workers: Option<u8>,
    /// Determines how many milliseconds device I/O will wait for before timing out, default = 2000
    io_timeout: u64
}


/// Represents config for one type of a device, ex 'MX Master 2'
/// 
/// `dpi` - [1200] for 1 sensor, [1200, 1000] for 2 sensors
/// 
struct Device {
    name: String,
    dpi: Option<DPI>,
    buttons: Vec<Button>,
    scroll_wheel: ScrollWheel,
    thumb_wheel: Option<ThumbWheel>
}


struct ScrollWheel {
    smart_shift: Option<SmartShift>,
    hi_res_scroll: Option<HiResScroll>,
    down: Option<Action>,
}


/// Settings for the thumb wheel
struct ThumbWheel {
    divert: bool,
    invert: bool,
    left: Option<Gesture>,
    right: Option<Gesture>,
    proxy: Option<Action>,
    touch: Option<Action>,
    tap: Option<Action>
}


/// ....
struct Button {
    cid: CID,
    actions: Vec<Action>
}


/// ...
struct Gesture {
    direction: GestureDirection,
    mode: GestureMode
}


/// ...
enum GestureDirection {
    None,
    Up,
    Donw,
    Left,
    Right
}

/// ...
enum GestureMode {
    OnRelease
}


/// ...
enum CID {
    ForwardButton
}


trait Action {

}




/// sensitivity is 0 to 100
struct SmartShift {
    on: bool,
    sensitivity: u16
}

impl Default for SmartShift {
    fn default() -> Self {
        SmartShift { on: true, sensitivity: 30 }
    }
}

/// ...
struct HiResScroll {
    on: bool,
    invert: bool,
    target: bool
}

impl Default for HiResScroll {
    fn default() -> Self {
        HiResScroll { on: true, invert: false, target: false }
    }
}
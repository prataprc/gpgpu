// mod log; TODO enable this only if we find it useful for debugging purpose.
mod pretty;
mod wloop;

pub use wloop::{
    DeviceButton, DeviceMotion, DeviceMouseMotion, DeviceMouseWheel, HandlerRes,
    WinAxisMotion, WinCursorEntered, WinCursorLeft, WinCursorMoved, WinKeyboardInput,
    WinLoop, WinMouseInput, WinMouseWheel, WinTouchpadPressure,
};

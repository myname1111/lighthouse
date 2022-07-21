use device_query::{DeviceState, Keycode};

use super::mouse::Mouse;

/// Creates a new game object
pub trait Object {
    /// upate the object
    fn update(&mut self);
}

/// An object that can be controlled by your keyboard
pub trait ControllableKey {
    /// Do things with device on update
    fn on_key(&mut self, keys: Vec<Keycode>);
}

/// An object that can be controlled by your Mouse
pub trait ControllableMouse {
    /// Do things with device on update
    fn on_mouse(&mut self, mouse: &mut Mouse, device: &mut DeviceState);
}

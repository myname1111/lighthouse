use device_query::DeviceState;

/// Creates a new game object
pub trait Object {
    /// upate the object
    fn update(&mut self);
}

/// An object that can be controlled by your keyboard
pub trait Controllable {
    /// Do things with device on update
    fn update_input(&mut self, keys: &mut DeviceState);
}

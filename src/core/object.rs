use device_query::{Keycode, MouseState};

/// Creates a new game object
pub trait Object {
    /// upate the object
    fn update(&mut self);
}

/// An object that can be controlled by your keyboard
pub trait ControllableByKey {
    /// Do things when a key is pressed
    fn on_key_press(&mut self, keys: Vec<Keycode>);
}

/// An object that can be controlled by your mouse
pub trait ControllableByMouse {
    /// Do things when a mouse is pressed
    fn on_mouse_press(&mut self, mouse: MouseState);
}

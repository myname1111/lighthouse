/// Creates a new game object
pub trait Object {
    /// upate the object
    fn update(&mut self);
}

/// An interactable object
pub trait Interactable {
    /// Do things when a key is pressed, TODO!
    fn on_key_press(&mut self, key: u8);
    /// Do things when a mouse is pressed, TODO!
    fn on_mouse_press(&mut self, key: u8);
}

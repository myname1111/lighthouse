use super::mouse::Mouse;
use device_query::{DeviceState, Keycode};
use nalgebra_glm::*;

/// Creates a new game object
pub trait Object {
    /// update the object
    fn update(&mut self);

    /// Get the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// let object_pos = object.get_pos();
    ///
    /// // Print position
    /// println!("{:?}", object_pos);
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn print_pos(&self) {
    ///         println!("{}", self.get_pos())
    ///     }
    /// }
    /// ```
    fn get_pos(&self) -> Vec3;
    /// Get the rotation of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// let object_rot = object.get_rot();
    ///
    /// // Print position
    /// println!("{:?}", object_rot);
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn print_pos(&self) {
    ///         println!("{}", self.get_rot())
    ///     }
    /// }
    /// ```
    fn get_rot(&self) -> Vec3;

    /// Set the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// object.set_pos(vec3(0.0, 0.0, 0.0));
    ///
    /// // Print position
    /// println!("{:?}", object.get_pos());
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn update_pos(&mut self) {
    ///         self.set_pos(self.get_pos() + 0.1)
    ///     }
    /// }
    /// ```
    fn set_pos(&mut self, pos: Vec3);
    /// Set the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// object.set_rot(vec3(0.0, 0.0, 0.0));
    ///
    /// // Print position
    /// println!("{:?}", object.get_rot());
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn update_rot(&mut self) {
    ///         self.set_rot(self.get_rot() + 0.1)
    ///     }
    /// }
    /// ```
    fn set_rot(&mut self, rot: Vec3);
}

/// An object trait that if implemented,
/// your object can be controlled by your keyboard
///
/// # Examples
/// this example explains how to implement your trait for your object
/// ```
/// impl ControllableKey for MyObject {
///     fn on_key(&mut, keys: Vec<Keycode>) {
///         // get all keys that are pressed
///         for key in keys {
///             // match keys
///             match key {
///                 Keycode::A => println!("Key a is pressed"),
///             }
///         }
///     }
/// }
/// ```
pub trait ControllableKey {
    /// Do things with device on update
    fn on_key(&mut self, keys: Vec<Keycode>);
}

/// An object trait that if implemented,
/// your object can be controlled by your mouse
///
/// # Examples
/// this example explains how to implement your trait for your object
/// ```
/// // in on_mouse
/// for pressed in mouse.get_pressed_cooldown(Duration::from_millis(100)) {
///     match pressed {
///         MousePressed::LeftMouse => println!("Left mouse pressed"),
///         MousePressed::RightMouse => println!("Right mouse pressed"),
///         _ => (),
///     }
/// }
/// ```
pub trait ControllableMouse {
    /// Do things with device on update
    fn on_mouse(&mut self, mouse: &mut Mouse, device: &mut DeviceState);
}

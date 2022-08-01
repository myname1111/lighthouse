use super::world::World;
use nalgebra_glm::*;
use std::any::Any;

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

/// Sets and gets the position and rotaion of the object
pub trait PosRot {
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
    fn get_pos(&self) -> &Vec3;
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
    fn get_rot(&self) -> &Vec3;

    /// Set the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// object.set_rot() = vec3(0.0, 0.0, 0.0);
    ///
    /// // Print position
    /// println!("{:?}", object.get_pos());
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn update_pos(&mut self) {
    ///         self.set_rot() += 0.1
    ///     }
    /// }
    /// ```
    fn set_pos(&mut self) -> &mut Vec3;
    /// Set the position of the object
    /// It is usually used in default trait impl
    ///
    /// # Direct Example
    /// ```
    /// let object = SomeObject::new();
    /// object.set_rot() = vec3(0.0, 0.0, 0.0);
    ///
    /// // Print position
    /// println!("{:?}", object.get_rot());
    /// ```
    /// # Example using traits
    /// ```
    /// trait SomeObjectTrait {
    ///     fn update_rot(&mut self) {
    ///         self.set_rot() += 0.1
    ///     }
    /// }
    /// ```
    fn set_rot(&mut self) -> &mut Vec3;
}

/// Creates a new game object
pub trait Object: PosRot {
    /// update the object
    fn update(&self) -> fn(world: &mut World, index: usize);
}

/// An object trait that if implemented,
/// your object can be controlled by your keyboard
///
/// # Examples
/// this example explains how to implement your trait for your object
/// ```
/// impl ControllableKey for MyObject {
///     fn on_key() -> fn(world: &mut World, index: usize);
///         |world, index| {
///             // get all keys that are pressed
///             for key in keys {
///                 // match keys
///                 match key {
///                     Keycode::A => println!("Key a is pressed"),
///                 }
///             }
///         }
///     }
/// }
/// ```
pub trait ControllableKey {
    /// Do things with device on update
    fn on_key(world: &mut World);
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
    fn on_mouse(&self) -> fn(world: &mut World, index: usize);
}

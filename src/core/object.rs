use super::world::{GameObjectTrait, World};
use nalgebra_glm::*;

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
    fn get_rot(&self) -> &Vec4;

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
    fn set_rot(&mut self) -> &mut Vec4;
}

#[macro_export]
/// Automaticly implement [PosRot] for you
///
/// # Example
///
/// basic usage
/// ```
/// struct MyObject {
///     pos: Vec3,
///     rot: Vec4 // these two are needed to make impl_posrot work
/// }
///
/// impl_posrot(MyObject) // this will implement posrot for you
/// ```
///
/// todo: A derive would be better
macro_rules! impl_posrot {
    ($object: ident) => {
        impl PosRot for $object {
            fn get_pos(&self) -> &Vec3 {
                &self.pos
            }

            fn get_rot(&self) -> &Vec4 {
                &self.rot
            }

            fn set_pos(&mut self) -> &mut Vec3 {
                &mut self.pos
            }

            fn set_rot(&mut self) -> &mut Vec4 {
                &mut self.rot
            }
        }
    };
}

/// Creates a new game object
pub trait Object<GameObject: GameObjectTrait + Sized>: PosRot {
    /// update the object
    fn update(world: &mut World<GameObject>, index: usize)
    where
        Self: Sized;
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
pub trait ControllableKey<GameObject: GameObjectTrait + Sized> {
    /// Do things with device on update
    fn on_key(world: &mut World<GameObject>);
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
pub trait ControllableMouse<GameObject: GameObjectTrait + Sized> {
    /// Do things with device on update
    fn on_mouse(world: &mut World<GameObject>);
}

/// Implement this trait if your object has a mesh
pub trait Mesh {
    /// get the vertex after position and rotation
    fn get_vert(&self);

    /// updates the mesh
    fn update_mesh(&self);
}

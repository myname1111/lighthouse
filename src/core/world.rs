use beryllium::GlWindow;
use device_query::DeviceState;
use nalgebra_glm::Vec2;

use crate::graphics::shader::ShaderProgram;

use super::{camera::CameraTrait, mouse::Mouse};

/// The world envieorment containing things like the keyboard and window
pub struct Enviroment {
    /// this is the window size
    pub win_size: Vec2,
    /// Window
    pub win: GlWindow,
    /// The shader program
    pub shader_program: ShaderProgram,
    /// device is the [DeviceState] for getting keyboard and mouse
    pub device: DeviceState,
    /// mouse is the [Mouse] wrapper for all things mouse
    pub mouse: Mouse,
}

impl Enviroment {
    /// Creates a new enviroment
    pub fn new(
        win_size: Vec2,
        win: GlWindow,
        shader_program: ShaderProgram,
        device: DeviceState,
        mouse: Mouse,
    ) -> Self {
        Enviroment {
            win_size,
            win,
            shader_program,
            device,
            mouse,
        }
    }
}

/// This trait defines the game objects in your world
/// # Example
/// basic usage
/// ```
/// struct MyObject { ... }
/// impl PosRot for MyObject
/// ...
///
/// struct GameObject {
///     my_objects: Vec<MyObject>
/// }
///
/// impl GameObjectTrait for GameObject {
///     fn update(&mut self) {
///         for i in 0..self.objects.my_objects.len() {
///             self.objects.my_objects[i].update()(self, i)
///         }
///     }
/// }
///
/// ```
pub trait GameObjectTrait {
    /// Updates the objects n game object
    /// See trait level doc for more info
    fn update(&self) -> fn(world: &mut World);

    /// Gets the camera, must implemt CameraTrait
    fn get_camera(&self) -> &dyn CameraTrait;

    /// Sets the camera, must implemt CameraTrait
    fn set_camera(&mut self) -> &mut dyn CameraTrait;
}

/// World struct taht stores everything thats relevant to the world
pub struct World {
    /// The computer enviroment
    pub env: Enviroment,
    /// All the objects in the world
    pub objects: Box<dyn GameObjectTrait>,
}

impl World {
    /// Creates a new world struct
    pub fn new(env: Enviroment, objects: Box<dyn GameObjectTrait>) -> Self {
        World { env, objects }
    }

    /// Update the world
    pub fn update(&mut self) {
        self.objects.update()(self);
    }
}

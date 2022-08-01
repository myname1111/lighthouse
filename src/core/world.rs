use beryllium::GlWindow;
use device_query::DeviceState;
use nalgebra_glm::Vec2;

use crate::graphics::shader::ShaderProgram;

use super::{camera::CameraTrait, mouse::Mouse, object::Object};

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

/// World struct taht stores everything thats relevant to the world
pub struct World<'a> {
    /// The computer enviroment
    pub env: Enviroment,
    /// The camera used to render the world
    pub camera: &'a mut dyn CameraTrait,
    /// All the objects in the world
    pub objects: Vec<Box<dyn Object>>,
}

impl<'a> World<'a> {
    /// Creates a new world struct
    pub fn new(
        env: Enviroment,
        camera: &'a mut (dyn CameraTrait + 'a),
        objects: Vec<Box<dyn Object>>,
    ) -> Self {
        World {
            env,
            camera,
            objects,
        }
    }

    /// Update the world
    pub fn update(&mut self) {
        for index in 0..self.objects.len() {
            self.objects[index].update()(self, index)
        }
        self.camera.matrix()
    }
}

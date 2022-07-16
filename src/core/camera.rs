use super::object::{ControllableByKey, Object};
use crate::graphics::{shader::ShaderProgram, uniform::Uniform};
use device_query::Keycode;

/// Camera trait
pub trait Camera: Object {
    /// Creates a new matrix from the camera position and parameters
    fn matrix(
        &self,
        near_plane: f32,
        far_plane: f32,
        shader_program: &ShaderProgram,
        uniform: &'static str,
    );
}

/// Defalut Camera struct with default implementation
pub struct DefaultCamera {
    /// This field is supposed to store positional information
    pub pos: glm::Vector3<f32>,
    /// This field is supposed to store rotational information
    pub rot: glm::Vector3<f32>,
    /// This field is supposed to store the width of the screen
    pub width: i32,
    /// This field is supposed to store the height of the screen
    pub height: i32,
    /// This field is supposed to store the sensitivity of the camera
    pub sensitivity: f32,
    /// FOV of the camera(in degrees)
    pub fov: f32,
}

impl DefaultCamera {
    /// Creates a new camera
    ///
    /// # Arguments
    ///
    /// pos: glm::Vector3<f32> is supposed to store positional information
    /// rot: glm::Vector3<f32> is supposed to store rotational information
    /// width: i32 is supposed to store the width of the camera
    /// height: i32 is supposed to store the height of the camera
    /// speed_pos: glm::Vector3<f32> is supposed to store the rotational speed of the camera
    /// speed_rot: glm::Vector3<f32> is supposed to store the rotational speed of the camera
    /// sensitivity: f32 is supposed to store the height of the camera
    pub fn new(
        pos: glm::Vector3<f32>,
        rot: glm::Vector3<f32>,
        width: i32,
        height: i32,
        sensitivity: f32,
        fov: f32,
    ) -> Self {
        DefaultCamera {
            pos,
            rot,
            width,
            height,
            sensitivity,
            fov,
        }
    }
}

impl Object for DefaultCamera {
    fn update(&mut self) {}
}

impl Camera for DefaultCamera {
    fn matrix(
        &self,
        near_plane: f32,
        far_plane: f32,
        shader_program: &ShaderProgram,
        uniform: &'static str,
    ) {
        let identity = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let model = identity.clone();

        let view = glm::ext::look_at(self.pos, self.pos + self.rot, glm::vec3(0.0, 1.0, 0.0));
        let proj = glm::ext::perspective::<f32>(
            glm::radians(self.fov),
            (self.width as f32) / (self.height as f32),
            near_plane,
            far_plane,
        );
        Uniform::new(shader_program, uniform).set_uniform_matrix(
            false,
            model
                .mul_m(&view)
                .mul_m(&proj)
                .as_array()
                .map(|x| *x.as_array()),
        )
    }
}

impl ControllableByKey for DefaultCamera {
    fn on_key_press(&mut self, keys: Vec<Keycode>) {
        for key in keys {
            match key {
                Keycode::W => self.pos.z = self.pos.z - 0.01,
                Keycode::A => self.pos.x = self.pos.x - 0.01,
                Keycode::S => self.pos.z = self.pos.z + 0.01,
                Keycode::D => self.pos.x = self.pos.x + 0.01,
                Keycode::LShift | Keycode::RShift => self.pos.y = self.pos.y - 0.01,
                Keycode::Space => self.pos.y = self.pos.y + 0.01,
                _ => (),
            }
        }
    }
}

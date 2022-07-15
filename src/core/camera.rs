use crate::graphics::{shader::ShaderProgram, uniform::Uniform};

use super::object::Object;
/// Camera trait
pub trait Camera: Object {
    /// Creates a new matrix from the camera position and parameters
    fn matrix(
        &self,
        fov_deg: f32,
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
    /// This field is supposed to store the positional speed of the camera
    pub speed_pos: glm::Vector3<f32>,
    /// This field is supposed to store the rotational speed of the camera
    pub speed_rot: glm::Vector3<f32>,
    /// This field is supposed to store the sensitivity of the camera
    pub sensitivity: f32,
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
        speed_pos: glm::Vector3<f32>,
        speed_rot: glm::Vector3<f32>,
        sensitivity: f32,
    ) -> Self {
        DefaultCamera {
            pos,
            rot,
            width,
            height,
            speed_pos,
            speed_rot,
            sensitivity,
        }
    }
}

impl Object for DefaultCamera {
    fn update(&mut self) {
        self.pos = self.pos + self.speed_pos;
        self.rot = self.rot + self.speed_rot;
    }
}

impl Camera for DefaultCamera {
    fn matrix(
        &self,
        fov_deg: f32,
        near_plane: f32,
        far_plane: f32,
        shader_program: &ShaderProgram,
        uniform: &'static str,
    ) {
        let identity = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let model = identity.clone();
        let view = identity.clone();

        let view = glm::ext::translate(&view, glm::vec3(0.0, 0.0, -2.0));
        let proj = glm::ext::perspective::<f32>(
            fov_deg,
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

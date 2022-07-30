use super::object::Object;
use super::world::Enviroment;
use crate::graphics::shader::ShaderProgram;
use crate::graphics::uniform::Uniform;
use nalgebra_glm::*;

/// Builder for [CameraSettings]
///
/// # Example
/// ```
/// // here are the required dependencies
/// let settings = CameraSettingsBuilder::new()
///     .screen_size(size)
///     .win(&win)
///     .shader_program(&shader_program)
///     // Here are the optional ones, they are filled with these default values
///     .fov(45.0)
///     .sensitivity(1.0)
///     .near_plane(0.1)
///     .far_plane(100.0)
///     .build() // And finally build
/// ```
#[derive(Copy, Clone)]
pub struct CameraSettingsBuilder<'a> {
    /// This field is supposed to store the width of the screen
    screen_size: Option<Vec2>,
    /// FOV of the camera(in degrees)
    fov: f32,
    /// Sensitivity of the mouse
    sensitivity: f32,
    /// Anything below this value will be clipped
    near_plane: f32,
    /// Anything above this value will be clipped
    far_plane: f32,
    /// The shader program
    shader_program: Option<&'a ShaderProgram>,
}

impl<'a> CameraSettingsBuilder<'a> {
    /// Creates a new camera settings
    pub fn new(env: Enviroment<'a>) -> Self {
        CameraSettingsBuilder::<'a> {
            screen_size: Some(env.win_size),
            fov: 45.0,
            sensitivity: 1.0,
            near_plane: 0.1,
            far_plane: 100.0,
            shader_program: Some(env.shader_program),
        }
    }

    /// This function is supposed to set the screen_size. It must be called
    pub fn screen_size(&mut self, screen_size: Vec2) -> &mut Self {
        self.screen_size = Some(screen_size);
        self
    }

    /// This function is supposed to set the fov. It is optional
    pub fn fov(&mut self, fov: f32) -> &mut Self {
        self.fov = fov;
        self
    }

    /// This function is supposed to set the sensitivity of the mouse. It is optional
    pub fn sensitivity(&mut self, sensitivity: f32) -> &mut Self {
        self.sensitivity = sensitivity;
        self
    }

    /// This function is supposed to set the near_plane. It is optional
    pub fn near_plane(&mut self, near_plane: f32) -> &mut Self {
        self.near_plane = near_plane;
        self
    }

    /// This function is supposed to set the far_plane. It is optional
    pub fn far_plane(&mut self, far_plane: f32) -> &mut Self {
        self.far_plane = far_plane;
        self
    }

    /// This function is supposed to set the shader_program. It must be called
    pub fn shader_program(&mut self, shader_program: &'a ShaderProgram) -> &mut Self {
        self.shader_program = Some(shader_program);
        self
    }

    /// Build the settings for the camera
    ///
    /// NOTE: will panic if an argument isn't default or specified
    pub fn build(&self) -> CameraSettings<'a> {
        CameraSettings::<'a> {
            screen_size: self.screen_size.expect("Error: argument screen width is not satisfied\nhelp: you can call .screen_width"),
            fov: 45.0,
            sensitivity: self.sensitivity,
            near_plane: 0.1,
            far_plane: 100.0,
            shader_program: self.shader_program.expect("Error: argument shadeer program is not satisfied\nhelp: you can call .shader_program"),
        }
    }
}

impl<'a> Default for CameraSettingsBuilder<'a> {
    /// Creates a new camera settings
    fn default() -> Self {
        CameraSettingsBuilder::<'a> {
            screen_size: None,
            fov: 45.0,
            sensitivity: 1.0,
            near_plane: 0.1,
            far_plane: 100.0,
            shader_program: None,
        }
    }
}

/// Setting for the [Camera] struct
///
/// # Examples
/// Make a new setting using [CameraSettingsBuilder]
/// ```
/// let camera_settings = CameraSettingsBuilder::new().
///     win(&win)
///     ... // see CameraSettingsBuilder
/// ```
/// load it into [Camera]
/// ```
/// let camera = Camera::new(pos, rot, settings);
/// ```
#[derive(Copy, Clone)]
pub struct CameraSettings<'a> {
    /// This field is supposed to store the width of the screen
    pub screen_size: Vec2,
    /// FOV of the camera(in degrees)
    pub fov: f32,
    /// Sensitivity of the mouse
    pub sensitivity: f32,
    /// anything below this value will be clipped
    pub near_plane: f32,
    /// anything above this value will be clipped
    pub far_plane: f32,
    /// the shader program
    pub shader_program: &'a ShaderProgram,
}

/// Camera trait responsible for the Camera struct. TODO: move Camera into Camera, ContorllabeMouse ... and users can implement
///
/// You dont have to implement matrix. You do however need to implement get_camera_settings for the
/// default implementation to work
/// # Examples
/// Make a new Camera
/// ```
/// impl CameraTrait for MyCamera {
///     fn get_camera_settings() {
///         self.settings
///     }
/// }
/// ```
pub trait CameraTrait: Object {
    /// Creates a new matrix from the camera position and parameters
    fn matrix(&self, uniform: &'static str) {
        let settings = self.get_camera_settings();

        let identity = mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let model = identity;

        let view = look_at(
            &self.get_pos(),
            &(self.get_pos() + self.get_rot()),
            &vec3(0.0, 1.0, 0.0),
        );
        let proj = perspective::<f32>(
            settings.screen_size.x / settings.screen_size.y,
            settings.fov.to_radians(),
            settings.near_plane,
            settings.far_plane,
        );

        Uniform::new(self.get_camera_settings().shader_program, uniform)
            .set_uniform_matrix(false, (proj * view * model).into())
    }

    /// Get the camera settings
    fn get_camera_settings(&self) -> CameraSettings;
}

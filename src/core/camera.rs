use super::object::*;
use crate::graphics::{shader::ShaderProgram, uniform::Uniform};
use beryllium::GlWindow;
use device_query::{Keycode, DeviceState, DeviceQuery};

/// Builder for [CameraSettings]
pub struct CameraSettingsBuilder<'a> {
    /// This field is supposed to store the width of the screen
    screen_width: Option<i32>,
    /// This field is supposed to store the height of the screen
    screen_height: Option<i32>,
    /// FOV of the camera(in degrees)
    fov: f32,
    /// Sensitivity of the mouse
    sensitivity: f32,
    /// Window
    win: Option<&'a GlWindow>,
    /// Anything below this value will be clipped
    near_plane: f32,
    /// Anything above this value will be clipped
    far_plane: f32,
    /// The shader program
    shader_program: Option<&'a ShaderProgram>,
}

impl<'a> CameraSettingsBuilder<'a> {
    /// Creates a new camera settings
    pub fn new() -> Self {
        CameraSettingsBuilder::<'a> {
            screen_width: None,
            screen_height: None,
            fov: 45.0,
            win: None,
            sensitivity: 1.0,
            near_plane: 0.1,
            far_plane: 100.0,
            shader_program: None,
        }
    }

    /// This function is supposed to set the screen_width. It must be called
    pub fn screen_width(&mut self, screen_width: i32) -> &mut Self {
        self.screen_width = Some(screen_width);
        self
    }

    /// This function is supposed to set the screen_height. It must be called
    pub fn screen_height(&mut self, screen_height: i32) -> &mut Self {
        self.screen_height = Some(screen_height);
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

    /// This function is supposed to set the win. It must be called
    pub fn win(&mut self, win: &'a GlWindow) -> &mut Self {
        self.win = Some(win);
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
            screen_width: self.screen_width.expect("Error: argument screen width is not satisfied\nhelp: you can call .screen_width"),
            screen_height: self.screen_height.expect("Error: argument screen height is not satisfied\nhelp: you can call .screen_height"),
            fov: 45.0,
            sensitivity: self.sensitivity,
            win: self.win.expect("Error: argument window is not satisfied\nhelp: you can call .win"),
            near_plane: 0.1,
            far_plane: 100.0,
            shader_program: self.shader_program.expect("Error: argument shadeer program is not satisfied\nhelp: you can call .shader_program"),
        }
    }
}

/// Setting for the [Camera] struct
pub struct CameraSettings<'a> {
    /// This field is supposed to store the width of the screen
    pub screen_width: i32,
    /// This field is supposed to store the height of the screen
    pub screen_height: i32,
    /// FOV of the camera(in degrees)
    pub fov: f32,
    /// Sensitivity of the mouse
    pub sensitivity: f32,
    /// Window
    pub win: &'a GlWindow,
    /// anything below this value will be clipped
    pub near_plane: f32,
    /// anything above this value will be clipped
    pub far_plane: f32,
    /// the shader program
    pub shader_program: &'a ShaderProgram,
}

/// Camera trait
pub trait Camera: Object {
    /// Creates a new matrix from the camera position and parameters
    fn matrix(&self, uniform: &'static str);
}

/// Defalut Camera struct with default implementation
pub struct DefaultCamera<'a> {
    /// This field is supposed to store positional information
    pub pos: glm::Vector3<f32>,
    /// This field is supposed to store rotational information
    pub rot: glm::Vector3<f32>,
    /// settings for the camera
    pub settings: CameraSettings<'a>,
}

impl<'a> DefaultCamera<'a> {
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
        settings: CameraSettings<'a>,
    ) -> Self {
        DefaultCamera::<'a> { pos, rot, settings }
    }
}

impl<'a> Object for DefaultCamera<'a> {
    fn update(&mut self) {}
}

impl<'a> Camera for DefaultCamera<'a> {
    fn matrix(&self, uniform: &'static str) {
        let identity = glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let model = identity.clone();

        let view = glm::ext::look_at(self.pos, self.pos + self.rot, glm::vec3(0.0, 1.0, 0.0));
        let proj = glm::ext::perspective::<f32>(
            glm::radians(self.settings.fov),
            (self.settings.screen_width as f32) / (self.settings.screen_height as f32),
            self.settings.near_plane,
            self.settings.far_plane,
        );

        Uniform::new(self.settings.shader_program, uniform).set_uniform_matrix(
            false,
            proj.mul_m(&view)
                .mul_m(&model)
                .as_array()
                .map(|x| *x.as_array()),
        )
    }
}

impl<'a> Controllable for DefaultCamera<'a> {
    fn update_input(&mut self, device: &mut DeviceState) {
        let keys = device.get_keys();

        if !keys.is_empty() {
            for key in keys {
                match key {
                    Keycode::W => self.pos.z = self.pos.z + 0.01,
                    Keycode::A => self.pos.x = self.pos.x + 0.01,
                    Keycode::S => self.pos.z = self.pos.z - 0.01,
                    Keycode::D => self.pos.x = self.pos.x - 0.01,
                    Keycode::LShift | Keycode::RShift => self.pos.y = self.pos.y - 0.01,
                    Keycode::Space => self.pos.y = self.pos.y + 0.01,
                    _ => (),
                }
            }
        }
    }
}
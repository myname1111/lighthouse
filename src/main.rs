#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Opengl tutorial";

use common_macros::*;
use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use image::DynamicImage;
use lighthouse::{
    core::{
        camera::{CameraSettings, CameraSettingsBuilder, CameraTrait},
        mouse::{MousePressed::*, StateOfMouse::*, *},
        object::{
            ControllableKey, ControllableMouse, Mesh, MeshObject, Object, PosRot, VertexTrait,
        },
        world::{self, Enviroment, GameObjectTrait, World},
    },
    graphics::{buffer::*, shader::*, texture::*, uniform::*, vertex::*, *},
    impl_posrot,
};
use nalgebra_glm::*;
use std::thread::sleep;
use std::time::*;
use std::{borrow::BorrowMut, fs};

type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 5] = [
    [0.5, -0.5, 0.5, 1.0, 0.0],   // front right
    [-0.5, -0.5, 0.5, 0.0, 0.0],  // front left
    [-0.5, -0.5, -0.5, 1.0, 0.0], // back left
    [0.5, -0.5, -0.5, 0.0, 0.0],  // back right
    [0.0, 0.5, 0.0, 0.5, 1.5],    // top
];

const INDICES: [TriIndexes; 4] = [[0, 1, 4], [1, 2, 4], [2, 3, 4], [0, 3, 4]];

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;

struct Vertex {
    vert: Vec3,
    tex_coord: Vec2,
}

impl VertexTrait for Vertex {
    const SIZE: usize = 5;

    fn as_list(&self) -> Vec<f32> {
        let out = Vec::<f32>::new();
        out.append(self.vert.into());
        out.append(self.tex_coord.into());
        out
    }
}

struct Camera {
    pos: Vec3,
    rot: Vec4,
    settings: CameraSettings,
    uniform: String,
}

impl Camera {
    pub fn new(pos: Vec3, rot: Vec4, settings: CameraSettings, uniform: String) -> Self {
        Camera {
            pos,
            rot,
            settings,
            uniform,
        }
    }
}

impl_posrot!(Camera);

impl Object<GameObject> for Camera {
    fn update(world: &mut World<GameObject>, _: usize) {
        Camera::matrix(&world.objects.camera);
        Camera::on_key(world);
    }
}

impl<'a> CameraTrait<GameObject> for Camera {
    fn get_camera_settings(&self) -> CameraSettings {
        self.settings
    }

    fn get_camera_uniform(&self) -> String {
        self.uniform.clone()
    }
}

impl ControllableKey<GameObject> for Camera {
    fn on_key(world: &mut World<GameObject>) {
        for key in world.env.device.get_keys() {
            match key {
                Keycode::W => world.objects.set_camera().set_pos().z += 0.01,
                Keycode::A => world.objects.set_camera().set_pos().x += 0.01,
                Keycode::S => world.objects.set_camera().set_pos().z -= 0.01,
                Keycode::D => world.objects.set_camera().set_pos().x -= 0.01,
                Keycode::LShift | Keycode::RShift => world.objects.set_camera().set_pos().y -= 0.01,
                Keycode::Space => world.objects.set_camera().set_pos().y += 0.01,
                _ => (),
            }
        }
    }
}

impl ControllableMouse<GameObject> for Camera {
    fn on_mouse(world: &mut World<GameObject>) {
        if let Some(keys) = world
            .env
            .mouse
            .get_pressed_cooldown(Duration::from_millis(100))
        {
            keys.iter().for_each(|key| match key {
                LeftMouse => world.env.mouse.state = Locked(world.env.win_size / 2.0),
                RightMouse => world.env.mouse.state = Free,
                _ => (),
            });
        }

        match world.env.mouse.state {
            Free => (),
            Locked(vec) => {
                let arr: [f32; 2] = vec.into();
                let (x, y) = (arr[0], arr[1]);

                world.env.win.warp_mouse_in_window(x as i32, y as i32);
                world.env.device = DeviceState::new();
                world.env.mouse.mouse = world.env.device.get_mouse();
            }
        }
    }
}

struct Pyramid {
    pos: Vec3,
    rot: Vec4,
    mesh: Mesh<Vertex>,
}

impl Mesh<GameObject, Vertex> for Pyramid {
    fn get_vert(&self) -> Vec<Vertex> {}

    fn get_mesh(&self) -> (Vec<Vertex>, Vec<usize>, Vec<[u32; 3]>) {
        todo!()
    }

    fn update_mesh(&self) {
        todo!()
    }
}

impl Pyramid {
    fn get_vert(&self) -> [[f32; 5]; 5] {
        let mut out: [[f32; 5]; 5] = self.mesh.0;

        for (index, vertex) in self.mesh.0.iter().enumerate() {
            let vec: [f32; 3] = rotate_vec3(
                &vec3(vertex[0], vertex[1], vertex[2]),
                self.rot.w,
                &self.rot.xyz(),
            )
            .into();

            let (one, _) = out[index].split_at_mut(3);

            one.copy_from_slice(&vec);
        }

        out
    }

    fn new(pos: Vec3, rot: Vec4, mesh: ([Vertex; 5], [TriIndexes; 4])) -> Self {
        let out = Self {
            pos,
            rot,
            mesh,
            vao: VertexArray::new().expect("Couldn't make a VAO"),
            vbo: Buffer::new().expect("Couldn't make a VBO"),
            ebo: Buffer::new().expect("Couldn't make EBO"),
        };

        out.vao.bind();
        out.vbo.bind(BufferType::Array);
        out.ebo.bind(BufferType::ElementArray);

        unsafe {
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            glVertexAttribPointer(
                1,
                2,
                GL_FLOAT,
                GL_FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                size_of::<[f32; 3]>() as *const _,
            );

            glEnableVertexAttribArray(0);
            glEnableVertexAttribArray(1);
        }

        out
    }
}

impl_posrot!(Pyramid);

impl Object<GameObject> for Pyramid {
    fn update(world: &mut World<GameObject>, _: usize)
    where
        Self: Sized,
    {
        world.objects.pyramid.rot.w += 0.01;

        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&world.objects.pyramid.get_vert()),
            GL_STATIC_DRAW,
        );
        buffer_data(
            BufferType::ElementArray,
            bytemuck::cast_slice(&world.objects.pyramid.mesh.1),
            GL_STATIC_DRAW,
        );
    }
}

struct GameObject {
    camera: Camera,
    pyramid: Pyramid,
}

impl GameObjectTrait for GameObject {
    fn update(&self) -> fn(world: &mut World<GameObject>) {
        |world: &mut World<GameObject>| {
            Camera::update(world, 0);
            Pyramid::update(world, 0)
        }
    }

    fn get_camera(&self) -> &dyn CameraTrait<Self> {
        &self.camera
    }

    fn set_camera(&mut self) -> &mut dyn CameraTrait<Self> {
        &mut self.camera
    }
}

fn main() {
    let vert = fs::read_to_string("shaders/vert.glsl").expect("Failed to read vertex shader");
    let frag = fs::read_to_string("shaders/frag.glsl").expect("Failed to read fragment shader");

    let vert = vert.as_str();
    let frag = frag.as_str();

    // Create a new device state
    let device_state = DeviceState::new();
    let mouse: Mouse = device_state.clone().into();

    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl.gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
            .unwrap();
    }
    let win = sdl
        .create_gl_window(
            WINDOW_TITLE,
            WindowPosition::Centered,
            WIDTH.into(),
            HEIGHT.into(),
            WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");
    win.set_swap_interval(SwapInterval::Vsync);

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
    }

    clear_color(0.2, 0.3, 0.3, 1.0); // sets background color

    // Pyramid
    let pyramid = Pyramid::new(
        vec3(0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        (VERTICES, INDICES),
    );

    let shader_program = ShaderProgram::from_vert_frag(vert, frag).unwrap();
    shader_program.use_program();

    // World
    let camera = Camera::new(
        vec3(0.0, 0.0, -2.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        CameraSettingsBuilder::default()
            .screen_size(vec2(WIDTH.into(), HEIGHT.into()))
            .shader_program(shader_program)
            .build(),
        "camera_matrix".to_string(),
    );

    let game_objects = GameObject { camera, pyramid };

    let mut world = World::<GameObject>::new(
        Enviroment::new(
            vec2(WIDTH.into(), HEIGHT.into()),
            win,
            shader_program.clone(),
            device_state,
            mouse,
        ),
        game_objects,
    );

    // textures
    let img = image::io::Reader::open("data/image.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let mut texture = Texture::from_image(
    GL_TEXTURE0,
    GL_TEXTURE_2D,
    hash_map!{
      "GL_TEXTURE_MIN_FILTER" => number::MultiSingularNumber::Number(number::Number::Integer(GL_NEAREST as i32)),
      "GL_TEXTURE_MAG_FILTER" => number::MultiSingularNumber::Number(number::Number::Integer(GL_LINEAR as i32)),
      "GL_TEXTURE_WRAP_S" => number::MultiSingularNumber::Number(number::Number::Integer(GL_REPEAT as i32)),
      "GL_TEXTURE_WRAP_T" => number::MultiSingularNumber::Number(number::Number::Integer(GL_REPEAT as i32))
    },
    0,
    img
  ).unwrap();

    // uniforms
    Uniform::new(&shader_program, "tex_color");

    // enable depth buffer
    enable(GL_DEPTH_TEST);
    world.update();
    // Location of the world
    'main_loop: loop {
        world.env.mouse.mouse = world.env.device.get_mouse();

        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
        }

        texture.bind(GL_TEXTURE_2D);
        world.update();

        // and then draw!
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glClear(GL_DEPTH_BUFFER_BIT);
            glDrawElements(
                GL_TRIANGLES,
                (size_of::<TriIndexes>() * INDICES.len())
                    .try_into()
                    .unwrap(),
                GL_UNSIGNED_INT,
                0 as *const _,
            );
        }
        world.env.win.swap_window();
    }
}

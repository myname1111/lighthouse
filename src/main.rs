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
        camera::{Camera, CameraSettingsBuilder, DefaultCamera},
        mouse::Mouse,
        object::{ControllableKey, ControllableMouse},
    },
    graphics::{buffer::*, shader::*, texture::*, uniform::*, vertex::*, *},
};
use nalgebra_glm::*;
use std::thread::sleep;
use std::time::*;
use std::{borrow::BorrowMut, fs};

type Vertex = [f32; 5];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 5] = [
    [0.5, -0.5, 0.0, 1.0, 0.0],   // front right
    [-0.5, -0.5, 0.0, 0.0, 0.0],  // front left
    [-0.5, -0.5, -1.0, 1.0, 0.0], // back left
    [0.5, -0.5, -1.0, 0.0, 0.0],  // back right
    [0.0, 0.5, -0.5, 0.5, 1.5],   // top
];

const INDICES: [TriIndexes; 4] = [[0, 1, 4], [1, 2, 4], [2, 3, 4], [0, 3, 4]];

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let vert = fs::read_to_string("shaders/vert.glsl").expect("Failed to read vertex shader");
    let frag = fs::read_to_string("shaders/frag.glsl").expect("Failed to read fragment shader");

    let vert = vert.as_str();
    let frag = frag.as_str();

    // Create a new device state
    let mut device_state = DeviceState::new();
    let mut mouse: Mouse = device_state.clone().into();

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
            WIDTH,
            HEIGHT,
            WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");
    win.set_swap_interval(SwapInterval::Vsync);

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name));
    }

    clear_color(0.2, 0.3, 0.3, 1.0); // sets background color

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    let ebo = Buffer::new().expect("Couldn't make EBO");
    ebo.bind(BufferType::ElementArray);
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        GL_STATIC_DRAW,
    );

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
    let shader_program = ShaderProgram::from_vert_frag(vert, frag).unwrap();
    shader_program.use_program();

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

    // Camera
    let mut camera = DefaultCamera::new(
        vec3(0.0, 0.0, -2.0),
        vec3(0.0, 0.0, 1.0),
        CameraSettingsBuilder::new()
            .screen_width(WIDTH.try_into().unwrap())
            .screen_height(HEIGHT.try_into().unwrap())
            .win(&win)
            .shader_program(&shader_program)
            .build(),
    );

    // uniforms
    Uniform::new(&shader_program, "tex_color");

    // enable depth buffer
    enable(GL_DEPTH_TEST);
    camera.matrix("camera_matrix");
    // Location of the world
    'main_loop: loop {
        mouse.mouse = device_state.get_mouse();

        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
        }

        camera.on_key(device_state.get_keys());
        camera.on_mouse(&mut mouse, &mut device_state);
        camera.matrix("camera_matrix");

        texture.bind(GL_TEXTURE_2D);

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
        win.swap_window();
    }
}

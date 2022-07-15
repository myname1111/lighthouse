//! An opengl library built around ogl33
//!
//! It is fully documented an is currently is a WIP
//! Although i hope to change that in the near future
//! It can be used to make 2d graphics
//!
//! It has a complete Shader system so you can make any type of shader
//!
//! # Limitations
//!
//! At the moment it cannot create 3d graphics
//! It only supports the creation of 2D textures
//! And does not support non primitive uniforms

#![deny(missing_docs)]

/// Module containing all things related to [buffer::Buffer]
pub mod buffer;
/// Module containing all things related to [number::MultiSingularNumber]
pub mod number;
/// Module containing all things related to [shader::Shader]
pub mod shader;
/// Module containing all things related to [texture::Texture]
pub mod texture;
/// Module containing all things related to [uniform::Uniform]
pub mod uniform;
/// Module containing all things related to [vertex::VertexArray]
pub mod vertex;

// imports
pub use beryllium::*;
pub use image::DynamicImage::{self, *};
pub use ogl33::*;
use std::ffi::CString;
use texture::TextureError;

/// This is an error enum, It contains several more specific enums in it as well as a misc error
pub enum Error {
    /// This is a texture error, it is used by [Texture]
    TextureError(TextureError),
    /// For all other error that do not fit
    Misc(String),
}

/// Takes a string of type &str and turs it into something that is used by opengl
/// so that it can be passed it opengl functions
///
/// # NOTE
/// This function is different from [to_cstr] due to opengl accepting array of strings
/// and just a string. However arrays of string are never used in my lib
///
/// this function is used by functions that accepts arrays of string
pub fn to_glstr(src: &str) -> *const u8 {
    src.as_bytes().as_ptr()
}

/// Takes a string of type &str and turs it into something that is equivalent to a c string
/// so that it can be passed it c functions
///
/// # NOTE
/// This function is different from [to_glstr] due to opengl accepting array of strings
/// and just a string. However arrays of string are never used in my lib
///
/// this function is used by functions that accepts a string
pub fn to_cstr(src: &'static str) -> CString {
    CString::new(src).unwrap()
}

/// Takes a vector of type &\[T\] and turs it into something that is equivalent to a c array
/// so that it can be passed it c functions
pub fn to_carray<T>(src: &[T]) -> *const T {
    src.as_ptr()
}

/// A safe version of glClearcolor
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { glClearColor(r, g, b, a) }
}

/// glEnable enable various capabilities
pub fn enable(cap: u32) {
    unsafe { glEnable(cap) }
}

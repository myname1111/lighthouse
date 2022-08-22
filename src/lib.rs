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
/// Module containing ECS stuff
pub mod ECS;
/// Module containing all things related to [crate::graphics]
pub mod graphics;

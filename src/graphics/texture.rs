use std::collections::HashMap;

use super::{number::*, *};

/// This is a texture error, it is used by [Texture]
#[derive(Debug)]
pub enum TextureError {
    /// This error happens when the name of the texture parameter dosen't exist
    UnknownTextureParameter(String),
}

/// A type used by [Texture] to store the texture params and it's values
pub type TextureParam = HashMap<&'static str, MultiSingularNumber>;

/// [Texture](https://www.khronos.org/opengl/wiki/Texture) is a wrapper for opengl textures
pub struct Texture {
    /// The texture id
    pub id: u32,
    /// The texture parameters
    pub params: TextureParam,
    /// The texture type, it can exist and not exist
    pub texture_type: Option<u32>,
}
impl Texture {
    /// Creates a new blank texture
    ///
    /// Prefer [Texture::from_image] over this
    pub fn new() -> Self {
        let mut texture: u32 = 0;
        unsafe {
            glGenTextures(1, &mut texture);
            Self {
                id: texture,
                params: {
                    let mut params = TextureParam::new();
                    params.insert("GL_DEPTH_COMPONENT", MultiSingularNumber::None);
                    params.insert("GL_STENCIL_INDEX", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_BASE_LEVEL", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_COMPARE_FUNC", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_COMPARE_MODE", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_LOD_BIAS", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_MIN_FILTER", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_MAG_FILTER", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_MIN_LOD", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_MAX_LOD", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_MAX_LOD", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_SWIZZLE_R", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_SWIZZLE_G", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_SWIZZLE_B", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_SWIZZLE_A", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_WRAP_S", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_WRAP_T", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_WRAP_R", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_BORDER_COLOR", MultiSingularNumber::None);
                    params.insert("GL_TEXTURE_SWIZZLE_RGBA", MultiSingularNumber::None);
                    params
                },
                texture_type: None,
            }
        }
    }

    /// Sets the texture unit for the texture, for more info see [glActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml)
    ///
    /// # Arguments
    ///
    /// * 'texture unit' - Can be anything of GL_TEXTUREi + the texture's location
    ///
    pub fn set_tex_unit(texture_unit: u32) {
        unsafe { glActiveTexture(texture_unit) }
    }

    /// Binds the texture to a certain type
    ///
    /// This function takes 1 argument which is texture_type
    pub fn bind(&mut self, texture_type: u32) {
        self.texture_type = Some(texture_type);
        unsafe { glBindTexture(texture_type, self.id) }
    }

    /// Unbinds the texture
    pub fn unbind(texture_type: u32) {
        unsafe { glBindTexture(texture_type, 0) }
    }

    /// Turns a string into the actual opengl parameter
    ///
    /// If the parameter is not found it will give you a [TextureError]
    pub fn texture_param_to_u32(in_str: &str) -> Result<u32, TextureError> {
        if in_str == "GL_DEPTH_COMPONENT" {
            Ok(GL_DEPTH_COMPONENT)
        } else if in_str == "GL_STENCIL_INDEX" {
            Ok(GL_STENCIL_INDEX)
        } else if in_str == "GL_TEXTURE_BASE_LEVEL" {
            Ok(GL_TEXTURE_BASE_LEVEL)
        } else if in_str == "GL_TEXTURE_COMPARE_FUNC" {
            Ok(GL_TEXTURE_COMPARE_FUNC)
        } else if in_str == "GL_TEXTURE_COMPARE_MODE" {
            Ok(GL_TEXTURE_COMPARE_MODE)
        } else if in_str == "GL_TEXTURE_LOD_BIAS" {
            Ok(GL_TEXTURE_LOD_BIAS)
        } else if in_str == "GL_TEXTURE_MIN_FILTER" {
            Ok(GL_TEXTURE_MIN_FILTER)
        } else if in_str == "GL_TEXTURE_MAG_FILTER" {
            Ok(GL_TEXTURE_MAG_FILTER)
        } else if in_str == "GL_TEXTURE_MIN_LOD" {
            Ok(GL_TEXTURE_MIN_LOD)
        } else if in_str == "GL_TEXTURE_MAX_LOD" {
            Ok(GL_TEXTURE_MAX_LOD)
        } else if in_str == "GL_TEXTURE_SWIZZLE_R" {
            Ok(GL_TEXTURE_SWIZZLE_R)
        } else if in_str == "GL_TEXTURE_SWIZZLE_G" {
            Ok(GL_TEXTURE_SWIZZLE_G)
        } else if in_str == "GL_TEXTURE_SWIZZLE_B" {
            Ok(GL_TEXTURE_SWIZZLE_B)
        } else if in_str == "GL_TEXTURE_SWIZZLE_A" {
            Ok(GL_TEXTURE_SWIZZLE_A)
        } else if in_str == "GL_TEXTURE_WRAP_S" {
            Ok(GL_TEXTURE_WRAP_S)
        } else if in_str == "GL_TEXTURE_WRAP_T" {
            Ok(GL_TEXTURE_WRAP_T)
        } else if in_str == "GL_TEXTURE_WRAP_R" {
            Ok(GL_TEXTURE_WRAP_R)
        } else if in_str == "GL_TEXTURE_BORDER_COLOR" {
            Ok(GL_TEXTURE_BORDER_COLOR)
        } else if in_str == "GL_TEXTURE_SWIZZLE_RGBA" {
            Ok(GL_TEXTURE_SWIZZLE_RGBA)
        } else {
            Err(TextureError::UnknownTextureParameter(format!(
                "{} not found",
                in_str
            )))
        }
    }

    /// Sets the parameters to the texture object
    pub fn set_params(&self) {
        let texture_params = &self.params;
        for (name, value) in texture_params {
            unsafe {
                match value {
                MultiSingularNumber::Number(number) => match number {
                    Number::Integer(int) => glTexParameteri(self.texture_type.unwrap(), Texture::texture_param_to_u32(name).unwrap(), *int),
                    Number::Float(float) => glTexParameterf(self.texture_type.unwrap(), Texture::texture_param_to_u32(name).unwrap(), *float),
                    Number::UsInteger(_) => panic!("For reasons beyond my understanding texture parameters for u8 only exist in vector commands"),
                },
                MultiSingularNumber::Array(array) => match array {
                    Array::Integer(int) => glTexParameterIiv(self.texture_type.unwrap(), Texture::texture_param_to_u32(name).unwrap(), to_carray(int)),
                    Array::Float(float) => glTexParameterfv(self.texture_type.unwrap(), Texture::texture_param_to_u32(name).unwrap(), to_carray(float)),
                    Array::UsInteger(uint) => glTexParameterIuiv(self.texture_type.unwrap(), Texture::texture_param_to_u32(name).unwrap(), to_carray(uint)),
                }
                MultiSingularNumber::None => (),
            }
            }
        }
    }

    /// Sets the image to the texture
    pub fn tex_2d(&self, lod: i32, img: DynamicImage) {
        let img = match img.flipv() {
            ImageRgba8(img) => img,
            img => img.to_rgba8(),
        };
        unsafe {
            glTexImage2D(
                self.texture_type.unwrap(),
                lod,
                GL_RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                to_carray(&img as &[u8]).cast(),
            )
        }
    }

    /// Generate the mipmaps required by the texture
    pub fn generate_mipmaps(&self) {
        unsafe {
            glGenerateMipmap(self.texture_type.unwrap());
        }
    }

    /// Creates a [Texture] object from an image
    pub fn from_image(
        texture_unit: u32,
        texture_type: u32,
        params: TextureParam,
        lod: i32,
        img: DynamicImage,
    ) -> Result<Texture, TextureError> {
        Texture::set_tex_unit(texture_unit);
        let mut texture = Texture::new();
        texture.bind(texture_type);

        for (param, value) in &params {
            if params.contains_key(*param) {
                *(texture.params).get_mut(param).unwrap() = *value;
            } else {
                return Err(TextureError::UnknownTextureParameter(format!(
                    "Error: Unknown parameter {}",
                    param
                )));
            }
        }

        texture.set_params();

        texture.tex_2d(lod, img);
        texture.generate_mipmaps();

        Ok(texture)
    }

    /// Deletes the texture
    pub fn delete(&self) {
        unsafe { glDeleteTextures(1, &self.id) }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::new()
    }
}

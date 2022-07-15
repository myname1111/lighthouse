use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Specifies what the type of the [Buffer] is
pub enum BufferType {
    /// Array Buffers holds arrays of vertex data for drawing.
    Array = GL_ARRAY_BUFFER as isize,
    /// Element Array Buffers hold indexes of what vertexes to use for drawing.
    ElementArray = GL_ELEMENT_ARRAY_BUFFER as isize,
}

/// Implementation of [VBO](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object)
pub struct Buffer(pub u32);
impl Buffer {
    /// Makes a new vertex buffer
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    /// Bind this vertex buffer for the given type
    pub fn bind(&self, ty: BufferType) {
        unsafe { glBindBuffer(ty as u32, self.0) }
    }

    /// Clear the current vertex buffer binding for the given type.
    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer(ty as u32, 0) }
    }
}

/// Store the data in the buffer
pub fn buffer_data(ty: BufferType, data: &[u8], usage: u32) {
    unsafe {
        glBufferData(
            ty as u32,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}

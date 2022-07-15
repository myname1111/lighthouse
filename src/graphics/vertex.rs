use super::*;

/// Creates a [VAO](https://www.khronos.org/opengl/wiki/Client-Side_Vertex_Arrays) and is used to make [VBO](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object)
/// using the [Buffer] struct
pub struct VertexArray(pub u32);
impl VertexArray {
    /// Creates a new VAO
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao) };
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    /// Binds the VAO
    pub fn bind(&self) {
        unsafe { glBindVertexArray(self.0) }
    }

    /// Clears the binding to the VAO
    pub fn clear_binding() {
        unsafe { glBindVertexArray(0) }
    }
}

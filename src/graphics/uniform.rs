use super::{shader::*, *};

/// A [Uniform object](https://www.khronos.org/opengl/wiki/Uniform_(GLSL))
pub struct Uniform(pub i32);
impl Uniform {
    /// Creates a new uniform
    pub fn new(program: &ShaderProgram, name: &str) -> Self {
        unsafe {
            Self(glGetUniformLocation(
                program.0,
                to_cstr(name).as_ptr().cast(),
            ))
        }
    }

    /// Sets the uniform as float
    pub fn set_uniform_f(&self, values: &[f32]) {
        unsafe {
            if values.len() == 1 {
                glUniform1f(self.0, values[0]);
            }
            if values.len() == 2 {
                glUniform2f(self.0, values[0], values[1]);
            }
            if values.len() == 3 {
                glUniform3f(self.0, values[0], values[1], values[2]);
            }
            if values.len() == 4 {
                glUniform4f(self.0, values[0], values[1], values[2], values[3]);
            }
        }
    }

    /// Sets the uniform as integer
    pub fn set_uniform_i(&self, values: &[i32]) {
        unsafe {
            if values.len() == 1 {
                glUniform1i(self.0, values[0]);
            }
            if values.len() == 2 {
                glUniform2i(self.0, values[0], values[1]);
            }
            if values.len() == 3 {
                glUniform3i(self.0, values[0], values[1], values[2]);
            }
            if values.len() == 4 {
                glUniform4i(self.0, values[0], values[1], values[2], values[3]);
            }
        }
    }

    /// Sets the uniform as unsigned integer
    pub fn set_uniform_ui(&self, values: &[u32]) {
        unsafe {
            if values.len() == 1 {
                glUniform1ui(self.0, values[0]);
            }
            if values.len() == 2 {
                glUniform2ui(self.0, values[0], values[1]);
            }
            if values.len() == 3 {
                glUniform3ui(self.0, values[0], values[1], values[2]);
            }
            if values.len() == 4 {
                glUniform4ui(self.0, values[0], values[1], values[2], values[3]);
            }
        }
    }

    /// Sets the uniform as ix2 matrix
    fn set_uniform_matrixix2<const ROW: usize, const COL: usize>(
        &self,
        transpose: bool,
        values: [[f32; COL]; ROW],
    ) {
        let value_vec: [f32; 4] = values
            .iter()
            .map(|inner| (*inner)[0])
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        unsafe {
            if values.len() == 1 {
                self.set_uniform_f(&value_vec);
            }
            if values.len() == 2 {
                glUniformMatrix2fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
            if values.len() == 3 {
                glUniformMatrix3x2fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
            if values.len() == 4 {
                glUniformMatrix4x2fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
        }
    }

    /// Sets the uniform as ix3 matrix
    fn set_uniform_matrixix3<const ROW: usize, const COL: usize>(
        &self,
        transpose: bool,
        values: [[f32; COL]; ROW],
    ) {
        let value_vec: [f32; 4] = values
            .iter()
            .map(|inner| (*inner)[0])
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        unsafe {
            if values.len() == 1 {
                self.set_uniform_f(&value_vec);
            }
            if values.len() == 2 {
                glUniformMatrix2x3fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
            if values.len() == 3 {
                glUniformMatrix3fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
            if values.len() == 4 {
                glUniformMatrix4x3fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
        }
    }

    /// Sets the uniform as ix3 matrix
    fn set_uniform_matrixix4<const ROW: usize, const COL: usize>(
        &self,
        transpose: bool,
        values: [[f32; COL]; ROW],
    ) {
        let value_vec: [f32; 4] = values
            .iter()
            .map(|inner| (*inner)[0])
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        unsafe {
            if values.len() == 1 {
                self.set_uniform_f(&value_vec);
            }
            if values.len() == 2 {
                glUniformMatrix2x4fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
            if values.len() == 3 {
                glUniformMatrix3x4fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
            if values.len() == 4 {
                glUniformMatrix4fv(self.0, 1, transpose as u8, values[0].as_ptr());
            }
        }
    }

    /// Sets the uniform as a matrix
    pub fn set_uniform_matrix<const ROW: usize, const COL: usize>(
        &self,
        transpose: bool,
        values: [[f32; COL]; ROW],
    ) {
        if values.len() == 1 {
            self.set_uniform_f(&values[0]);
        }
        if ROW == 2 {
            self.set_uniform_matrixix2(transpose, values);
        }
        if values.len() == 3 {
            self.set_uniform_matrixix3(transpose, values);
        }
        if values.len() == 4 {
            self.set_uniform_matrixix4(transpose, values);
        }
    }
}

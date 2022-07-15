use super::*;
/// Specifies the type of [Shader]
pub enum ShaderType {
    /// Vertex shaders determine the position of geometry within the screen.
    Vertex = GL_VERTEX_SHADER as isize,
    /// Fragment shaders determine the color output of geometry.
    ///
    /// Also other values, but mostly color.
    Fragment = GL_FRAGMENT_SHADER as isize,
}

/// An opengl [shader](https://www.khronos.org/opengl/wiki/Shader) That is of type [ShaderType]
pub struct Shader(pub u32);
impl Shader {
    /// Creates a new shader
    ///
    /// Prefer from_source as it simplify's the process and is less error prone
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { glCreateShader(ty as u32) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    /// Sets the source code for the shader
    pub fn set_source(&self, src: &str) {
        unsafe {
            glShaderSource(
                self.0,
                1,
                &to_glstr(src).cast(),
                &(src.len().try_into().unwrap()),
            )
        }
    }

    /// Compiles the shader
    pub fn compile(&self) {
        unsafe { glCompileShader(self.0) }
    }

    /// Checks if the shader can be compiled
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(GL_TRUE)
    }

    /// Gets the log data for the program
    pub fn info_log(&self) -> String {
        let mut needed_ln = 0;
        unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_ln) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_ln.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Marks the program for deletion
    pub fn delete(&self) {
        unsafe { glDeleteShader(self.0) }
    }

    /// Creates a new shader program from a string
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty).ok_or_else(|| "Unable to allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }
}

/// A handle to a [Program
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects)
pub struct ShaderProgram(pub u32);
impl ShaderProgram {
    /// Allocates a new program object.
    ///
    /// Prefer ['ShaderProgram::from_vert_frag'](ShaderProgram::from_vert_frag),
    /// it makes a complete program from the vertex and fragment sources all at
    /// once.
    pub fn new() -> Option<Self> {
        let prog = unsafe { glCreateProgram() };
        if prog != 0 {
            Some(Self(prog))
        } else {
            None
        }
    }

    /// Attaches a shader object to this program object.
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { glAttachShader(self.0, shader.0) };
    }

    /// Links the various attached, compiled shader objects into a usable program.
    pub fn link_program(&self) {
        unsafe { glLinkProgram(self.0) };
    }

    /// Checks if the last linking operation was successful.
    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
        success == i32::from(GL_TRUE)
    }

    /// Gets the log data for this program.
    ///
    /// This is usually used to check the message when a program failed to link.
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Sets the program as the program to use when drawing.
    pub fn use_program(&self) {
        unsafe { glUseProgram(self.0) };
    }

    /// Marks the program for deletion.
    ///
    /// Note: This _does not_ immediately delete the program. If the program is
    /// currently in use it won't be deleted until it's not the active program.
    /// When a program is finally deleted and attached shaders are unattached.
    pub fn delete(self) {
        unsafe { glDeleteProgram(self.0) };
    }

    /// Takes a vertex shader source string and a fragment shader source string
    /// and either gets you a working program object or gets you an error message.
    ///
    /// This is the preferred way to create a simple shader program in the common
    /// case. It's just less error prone than doing all the steps yourself.
    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }
}

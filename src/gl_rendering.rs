use gl;
use gl::types::{GLuint, GLint, GLchar};
use std;
use std::ffi::{CStr, CString};

pub struct Shader {
    id: GLuint
}
impl Shader {
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
    pub fn from_str(src: &str, shader_type: GLuint) -> Result<Shader, String> {
        let c_src = CString::new(src).unwrap();
        let id = compile_shader(&c_src, shader_type)?;
        Ok(id)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub fn create_c_string_buffer(len: usize) -> CString {
    let mut c_string_buffer = Vec::with_capacity(len + 1);
    c_string_buffer.extend(std::iter::repeat(b' ').take(len));
    unsafe { CString::from_vec_unchecked(c_string_buffer) }
}

pub fn compile_shader(shader_source: &CStr, shader_type: GLuint) -> Result<Shader, String> {
    let id = unsafe { gl::CreateShader(shader_type) };

    unsafe {
        gl::ShaderSource(id, 1, &shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    let mut success: GLint = 0; // Dummy value replaced by the GetShaderiv function.
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success != 0 {    // 0 is false in C.
        Ok(Shader{ id })
    } else {
        let mut len: GLint = 0; // Dummy value
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let mut error_buffer: CString = create_c_string_buffer(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error_buffer.as_ptr() as *mut GLchar
            );
        }
        Err(error_buffer.to_string_lossy().into_owned())
    }
}

pub struct Program {
    id: GLuint
}

impl Program {
    pub fn id(&self) -> GLuint {
        self.id
    }
}

pub fn link_program(shaders: &[Shader]) -> Result<Program, String> {
    unsafe {
        let program = Program { id: gl::CreateProgram() };
        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }
        gl::LinkProgram(program.id);
        for shader in shaders {
            gl::DetachShader(program.id, shader.id);
        }
        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        if success != 0 { // 0 is failure
            Ok(program)
        }
        else {
            let mut len: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut len);
            let mut error_buffer = create_c_string_buffer(len as usize);
            gl::GetProgramInfoLog(
                program.id,
                len,
                std::ptr::null_mut(),
                error_buffer.as_ptr() as *mut GLchar
            );
            Err(error_buffer.to_string_lossy().into_owned())
        }
    }
}

pub fn use_program(program: &Program) {
    unsafe {
        gl::UseProgram(program.id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

use graphics::Matrix4;
pub fn ortho2d(matrix: &mut Matrix4, left: f32, right: f32, bottom: f32, top: f32) {
    let zNear = -1.0f32;
    let zFar = 1.0f32;
    let inv_z = 1.0f32 / (zFar - zNear);
    let inv_y = 1.0f32 / (top - bottom);
    let inv_x = 1.0f32 / (right - left);
    
    //first column
    matrix[0] = 2.0*inv_x;
    matrix[1] = 0.0;
    matrix[2] = 0.0;
    matrix[3] = 0.0;

    //second
    matrix[4] = 0.0;
    matrix[5] = 2.0*inv_y;
    matrix[6] = 0.0;
    matrix[7] = 0.0;

    //third
    matrix[8] = 0.0;
    matrix[9] = 0.0;
    matrix[10] = -2.0*inv_z;
    matrix[11] = 0.0;

    //fourth
    matrix[12] = -(right + left)*inv_x;
    matrix[13] = -(top + bottom)*inv_y;
    matrix[14] = -(zFar + zNear)*inv_z;
    matrix[15] = 1.0;

}
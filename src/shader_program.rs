use std::ptr;
use std::ffi::CString;
extern crate gl;
use gl::types::*;

pub struct ShaderProgramBuilder {
  handle: GLuint
}

impl ShaderProgramBuilder {
  pub fn new() -> ShaderProgramBuilder {
    ShaderProgramBuilder {
      handle: unsafe { gl::CreateProgram() }
    }
  }

  pub fn with_vertex_shader(self, glsl: &str) -> ShaderProgramBuilder {
    self.with_shader(gl::VERTEX_SHADER, glsl)
  }

  pub fn with_fragment_shader(self, glsl: &str) -> ShaderProgramBuilder {
    self.with_shader(gl::FRAGMENT_SHADER, glsl)
  }

  // geometry shader
  // tesselation shader

  pub fn with_shader(self, shader_type: GLenum, glsl: &str) -> ShaderProgramBuilder {
    let shader = load_shader(shader_type, glsl);
    unsafe { gl::AttachShader(self.handle, shader); }
    self
  }

  pub fn build(self) -> GLuint {
    unsafe {
      link_program(self.handle);
      gl::UseProgram(self.handle);
    }
    self.handle
  }
}

fn load_shader(shader_type: GLenum, glsl: &str) -> GLuint {
  let glsl = &CString::new(glsl).unwrap();
  unsafe {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(shader, 1, &glsl.as_ptr(), ptr::null());
    gl::CompileShader(shader);
    check_gl_status(shader, gl::COMPILE_STATUS);
    shader
  }
}

unsafe fn link_program(program_id: GLuint) {
  gl::LinkProgram(program_id);
  check_gl_status(program_id, gl::LINK_STATUS);
  check_gl_status(program_id, gl::VALIDATE_STATUS);
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  // allocate buffer of correct size
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  // fill it with len spaces
  buffer.extend([b' '].iter().cycle().take(len));
  // convert buffer to CString
  unsafe { CString::from_vec_unchecked(buffer) }
}

unsafe fn check_gl_status(handle: GLuint, status: GLenum){
  let get_paramater: unsafe fn(GLuint, GLenum, *mut GLint);
  let get_info_log: unsafe fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar);
  match status {
    gl::COMPILE_STATUS => {
      get_paramater = gl::GetShaderiv;
      get_info_log = gl::GetShaderInfoLog;
    },
    _ => {
      get_paramater = gl::GetProgramiv;
      get_info_log = gl::GetProgramInfoLog;
    }
  }
  let mut success: GLint = 1;
  get_paramater(handle, status, &mut success);
  if success == 0 { 
    let mut len: GLint = 0;
    get_paramater(handle, gl::INFO_LOG_LENGTH, &mut len);
    let error = create_whitespace_cstring_with_len(len as usize);
    get_info_log(
      handle,
      len,
      ptr::null_mut(),
      error.as_ptr() as *mut GLchar
    );
    println!("Error: {}", error.to_string_lossy().into_owned())
  }
}
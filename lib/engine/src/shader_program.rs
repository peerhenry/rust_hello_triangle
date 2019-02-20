use std::cell::RefCell;
use std::ffi::CString;
use std::collections::HashMap;
use gl::types::*;
use cgmath::{ Matrix, Matrix4 };
use std::ptr;
use std::collections::HashSet;
use std::any::TypeId;
use std::marker::PhantomData;

// todo: cleanup shaders

// Helper structs & enums

/*
use std::cmp::Eq;

#[derive(PartialEq)]
enum Shader {
  Vertex,
  Fragment,
  Geometry
}

struct ShaderHandle {
  shader_type: Shader,
  handle: GLuint
}

impl PartialEq for ShaderHandle {
  fn eq(&self, other: &ShaderHandle) -> bool {
   return self.shader_type == other.shader_type && self.handle == other.handle
  }
}

impl Eq for ShaderHandle {}
*/

#[derive(Clone, Copy)]
pub struct Uniform {
  location: GLint,
  gl_type: GLenum
}

// ShaderProgram

#[derive(Default)]
pub struct ShaderProgram {
  pub handle: GLuint,
  pub uniform_location_map: RefCell<HashMap<String, Uniform>>
}

macro_rules! gl_stringify {
  ($a:expr) => { CString::new($a).unwrap().as_ptr() }
}

impl ShaderProgram {
  pub unsafe fn get_uniform(&self, name: &str) -> Uniform {
    let mut mut_map = self.uniform_location_map.borrow_mut();
    let uniform_option = mut_map.get(name);
    let uniform: Uniform;
    if let Some(wrapped_uniform) = uniform_option {
      uniform = *wrapped_uniform;
    } else {
      let uniform_location = gl::GetUniformLocation(self.handle, gl_stringify!(name));
      if uniform_location < 0 { panic!("uniform {} does not exist", name); }
      let mut length: GLsizei = 0;  // howmany characters are written by OpenGL
      let mut size: GLint = 0;      // size of uniform variable
      let mut gl_type: GLenum = 0;  // type of the uniform variable
      let mut gl_char: GLchar = 0;  // buffer to write the name of the uniform variable to
      gl::GetActiveUniform(self.handle, uniform_location as GLuint, 1, &mut length as _, &mut size as _, &mut gl_type as _, &mut gl_char as _);
      uniform = Uniform { location: uniform_location, gl_type };
      mut_map.insert(String::from(name), uniform);
    }
    uniform
  }

  pub unsafe fn get_uniform_location(&self, name: &str) -> GLint {
    self.get_uniform(name).location
  }

  pub unsafe fn set_uniform_matrix(&self, name: &str, matrix: Matrix4<GLfloat>) {
    let uniform_location: GLint = self.get_uniform_location(name);
    gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.as_ptr());
  }

  pub unsafe fn create_uniform_setter(&self, name: &str) -> UniformSetter {
    let uniform = self.get_uniform(name);
    UniformSetter::new(uniform.location)
  }
}

// checkout a complete list of gl types here: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniform.xhtml
fn get_required_type_id(gl_type: GLenum) -> TypeId {
  match gl_type {
    GL_FLOAT => return TypeId::of::<GLfloat>(),
    GL_FLOAT_MAT4 => return TypeId::of::<Matrix4<GLfloat>>(),
    _ => panic!("gl type not recognized: {}", gl_type)
  }
}

// BUILDER

#[derive(Default)]
pub struct ShaderProgramBuilder {
  handle: GLuint,
  shader_handles: HashSet<GLuint>,
}

impl ShaderProgramBuilder {
  pub fn new() -> Self {
    ShaderProgramBuilder {
      handle: unsafe { gl::CreateProgram() },
      ..Default::default()
    }
  }

  #[allow(dead_code)]
  pub fn with_vertex_shader(self, glsl: &str) -> Self {
    self.with_shader(gl::VERTEX_SHADER, glsl)
  }

  #[allow(dead_code)]
  pub fn with_fragment_shader(self, glsl: &str) -> Self {
    self.with_shader(gl::FRAGMENT_SHADER, glsl)
  }

  #[allow(dead_code)]
  pub fn with_geometry_shader(self, glsl: &str) -> Self {
    self.with_shader(gl::GEOMETRY_SHADER, glsl)
  }

  pub fn with_shader(mut self, shader_type: GLenum, glsl: &str) -> Self {
    let shader_handle = load_shader(shader_type, glsl);
    unsafe { gl::AttachShader(self.handle, shader_handle); }
    self.shader_handles.insert(shader_handle);
    self
  }

  pub fn build(self) -> ShaderProgram {
    unsafe {
      link_program(self.handle);
      gl::UseProgram(self.handle);
    }
    ShaderProgram {
      handle: self.handle,
      ..Default::default()
    }
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

// UniformSetter

pub trait SetUniform<T> {
  unsafe fn set(&self, value: T) {
    panic!("cannot handle type for OpenGL");
  }
}

pub struct UniformSetter {
  location: GLint,
  // phantom: PhantomData<&'a T>
}

impl UniformSetter {
  pub fn new(location: GLint) -> Self {
    return UniformSetter { location };
  }
}

impl SetUniform<Matrix4<GLfloat>> for UniformSetter {
  unsafe fn set(&self, value: Matrix4<GLfloat>) {
    gl::UniformMatrix4fv(self.location, 1, gl::FALSE, value.as_ptr())
  }
}

impl SetUniform<GLfloat> for UniformSetter {
  unsafe fn set(&self, value: GLfloat) {
    gl::Uniform1f(self.location, value)
  }
}

/*
impl UniformSetter<GLfloat> {
  pub unsafe fn set(&self, value: GLfloat) {
    gl::Uniform1f(self.location, value)
  }
}
*/
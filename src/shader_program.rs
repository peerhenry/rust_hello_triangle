use std::cell::RefCell;
use std::ffi::CString;
use std::collections::HashMap;
use gl::types::*;
use cgmath::{ Matrix, Matrix4 };

#[derive(Default)]
pub struct ShaderProgram {
  pub handle: GLuint,
  pub uniform_location_map: RefCell<HashMap<String, GLint>>
}

macro_rules! gl_stringify {
  ($a:expr) => { CString::new($a).unwrap().as_ptr() }
}

impl ShaderProgram {
  pub unsafe fn get_uniform_location(&self, name: &str) -> GLint {
    let mut mut_map = self.uniform_location_map.borrow_mut();
    let uniform_location_option = mut_map.get(name);
    let uniform_location: GLint;
    if let Some(location) = uniform_location_option {
      uniform_location = *location;
    } else {
      uniform_location = gl::GetUniformLocation(self.handle, gl_stringify!(name));
      mut_map.insert(String::from(name), uniform_location);
    }
    uniform_location
  }

  pub unsafe fn set_uniform_matrix(&self, name: &str, matrix: Matrix4<GLfloat>) {
    let uniform_location = self.get_uniform_location(name);
    if uniform_location < 0 { panic!("uniform {} does not exist", name); }
    else { gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.as_ptr()); }
  }
}
use gl::types::GLint;
use gl::types::GLuint;

pub struct AttribData {
  pub location: GLuint,
  pub floats_per_attribute: GLint,
  pub floats_per_vertex: usize,
  pub offset: usize
}
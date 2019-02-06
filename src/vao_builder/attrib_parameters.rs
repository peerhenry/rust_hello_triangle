use gl::types::GLint;

pub struct AttribParameters {
  pub floats_per_attribute: GLint,
  pub floats_per_vertex: usize,
  pub offset: usize
}

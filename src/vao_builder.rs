use std::mem::size_of;
use gl::types::*;

// builder pattern; VaoBuilder returns BufferComponent

pub struct BufferComponent {
  pub vao: GLuint,
  pub vbo: GLuint,
  pub ibo: GLuint
}

pub struct AttribParameters {
  pub floats_per_attribute: GLint,
  pub floats_per_vertex: usize,
  pub offset: usize
}

struct AttribData {
  location: GLuint,
  floats_per_attribute: GLint,
  floats_per_vertex: usize,
  offset: usize
}

pub struct VaoBuilder {
  use_indices: bool,
  attribs: Vec<AttribData>,
  next_attrib_location: GLuint
}

impl VaoBuilder {
  pub fn new() -> VaoBuilder {
    VaoBuilder {
      use_indices: false,
      attribs: Vec::new(),
      next_attrib_location: 0
    }
  }

  #[allow(dead_code)]
  pub fn with_ibo(mut self) -> VaoBuilder {
    self.use_indices = true;
    self
  }

  #[allow(dead_code)]
  pub fn with_attribute(mut self, params: AttribParameters) -> VaoBuilder {
    self.attribs.push(AttribData {
      location: self.next_attrib_location,
      floats_per_attribute: params.floats_per_attribute,
      floats_per_vertex: params.floats_per_vertex,
      offset: params.offset
    });
    self.next_attrib_location = self.next_attrib_location + 1;
    self
  }

  pub fn build(self) -> BufferComponent {
    let mut vao: GLuint = 0; // vertex array object
    let mut vbo: GLuint = 0; // vertex buffer object
    let mut ibo: GLuint = 0; // index buffer object
    unsafe {
      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);
      gl::GenBuffers(1, &mut vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      if self.use_indices {
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
      }
      for attrib_data in self.attribs {
        setup_attribute(attrib_data);
      }

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
      gl::BindVertexArray(0);
    }
    BufferComponent {
      vao: vao,
      vbo: vbo,
      ibo: ibo
    }
  }
}

unsafe fn setup_attribute(attrib: AttribData){
  gl::EnableVertexAttribArray(attrib.location); // this is "layout (location = 0)" in vertex shader
  gl::VertexAttribPointer(
    attrib.location,   // location
    attrib.floats_per_attribute,          // number per attribute
    gl::FLOAT,  // data type
    gl::FALSE,  // normalized
    (attrib.floats_per_vertex * size_of::<GLfloat>()) as GLint,  // stride
    (attrib.offset * size_of::<GLfloat>()) as *const gl::types::GLvoid  // offset
  );
}
use gl::types::*;
use cgmath::{ Rad, Deg, Matrix4, PerspectiveFov, Point3, Vector3 };

pub struct Camera {
  pub view_matrix: Matrix4<GLfloat>,
  pub eye: Point3<GLfloat>,
  pub target: Point3<GLfloat>,
  pub up: Vector3<GLfloat>,

  pub projection_matrix: Matrix4<GLfloat>,
  pub fovy: Rad<GLfloat>,
  pub aspect: GLfloat,
  pub near: GLfloat,
  pub far: GLfloat
}

pub struct CameraBuilder {
  pub eye: Point3<GLfloat>,
  pub target: Point3<GLfloat>,
  pub up: Vector3<GLfloat>,
  pub fovy: Rad<GLfloat>,
  pub aspect: GLfloat,
  pub near: GLfloat,
  pub far: GLfloat
}

impl CameraBuilder {
  pub fn new() -> CameraBuilder {
    CameraBuilder {
      eye: Point3::new(0.0, 0.0, -2.0),
      target: Point3::new(0.0, 0.0, 0.0),
      up: Vector3::new(0.0, 1.0, 0.0),
      fovy: Rad::from( Deg(45.0) ),
      aspect: 16.0/9.0,
      near: 0.1,
      far: 100.0
    }
  }

  #[allow(dead_code)]
  pub fn with_eye(mut self, eye: Point3<GLfloat>) -> CameraBuilder {
    self.eye = eye;
    self
  }

  #[allow(dead_code)]
  pub fn with_target(mut self, target: Point3<GLfloat>) -> CameraBuilder {
    self.target = target;
    self
  }

  #[allow(dead_code)]
  pub fn with_up(mut self, up: Vector3<GLfloat>) -> CameraBuilder {
    self.up = up;
    self
  }

  #[allow(dead_code)]
  pub fn with_fovy(mut self, fovy: Rad<GLfloat>) -> CameraBuilder {
    self.fovy = fovy;
    self
  }

  #[allow(dead_code)]
  pub fn with_aspect(mut self, aspect: GLfloat) -> CameraBuilder {
    self.aspect = aspect;
    self
  }

  #[allow(dead_code)]
  pub fn with_near(mut self, near: GLfloat) -> CameraBuilder {
    self.near = near;
    self
  }

  #[allow(dead_code)]
  pub fn with_far(mut self, far: GLfloat) -> CameraBuilder {
    self.far = far;
    self
  }

  pub fn build(self) -> Camera {
    Camera {
      eye: self.eye,
      target: self.target,
      up: self.up,
      fovy: self.fovy,
      aspect: self.aspect,
      near: self.near,
      far: self.far,
      view_matrix: Matrix4::look_at(self.eye, self.target, self.up),
      projection_matrix: Matrix4::from(PerspectiveFov {
        fovy: self.fovy,
        aspect: self.aspect,
        near: self.near,
        far: self.far
      }),
    }
  }
}
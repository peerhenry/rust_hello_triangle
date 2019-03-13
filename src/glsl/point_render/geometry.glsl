#version 450

layout ( points ) in; // define input type
layout ( triangle_strip, max_vertices = 4 ) out; // define output type

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

in vec4 Color[];
out vec4 GeometryColor;

void createVertex(vec3 offset) {
  float size = 0.1;
  vec4 actualOffset = vec4(offset * size, 0.0);
  vec4 worldPosition = gl_in[0].gl_Position + actualOffset;
  gl_Position = Projection * View * Model * worldPosition;
  GeometryColor = Color[0];
  EmitVertex();
}

void main(void){
  createVertex(vec3(-1.0, 1.0, 0.0));
  createVertex(vec3(-1.0, -1.0, 0.0));
  createVertex(vec3(1.0, 1.0, 0.0));
  createVertex(vec3(1.0, -1.0, 0.0));
  EndPrimitive();
}
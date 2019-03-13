#version 450

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec4 VertexColor;

out vec4 Color;

void main()
{
    Color = VertexColor;
    // gl_Position = Projection * View * Model * vec4(VertexPosition, 1.0);
    gl_Position = vec4(VertexPosition, 1.0);
}
#version 450 core

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec4 VertexColor;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

out vec4 Color;

void main()
{
    Color = VertexColor;
    gl_Position = Projection * View * Model * vec4(VertexPosition, 1.0);
}
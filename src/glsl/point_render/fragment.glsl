#version 450

in vec4 GeometryColor;
out vec4 FragmentColor;

void main()
{
    FragmentColor = GeometryColor;
}
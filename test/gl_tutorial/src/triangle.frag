#version 330 core

out vec3 Color;
in vec3 fragmentColor;

void main()
{
    Color = fragmentColor;
}

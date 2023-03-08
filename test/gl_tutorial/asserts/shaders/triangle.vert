#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 vertexColor;
layout (location = 2) in vec2 vertexUV;
out vec3 fragmentColor;
out vec2 UV;

uniform mat4 Center;
uniform mat4 Scale;
uniform mat4 Translation;
uniform mat4 Rotation;
uniform mat4 Projection;


void main()
{
    vec4 v = vec4(Position, 1.0);
    gl_Position = Projection * Translation * Rotation * Scale * Center * v;
    fragmentColor = vertexColor;
    UV = vertexUV;
}

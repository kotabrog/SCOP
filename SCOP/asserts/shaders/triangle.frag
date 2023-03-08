#version 330 core

out vec3 Color;

in vec3 fragmentColor;
in vec2 UV;
uniform sampler2D myTextureSampler;
uniform int IsTexture;


void main()
{
    if (IsTexture == 0) {
        Color = fragmentColor;
    } else {
        Color = texture( myTextureSampler, UV ).rgb;
    }
}

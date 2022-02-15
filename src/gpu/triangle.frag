#version 330 core

//out vec4 Color;

uniform sampler3D ourTexture;

void main()
{
    vec2 uv = gl_FragCoord.xy / vec2(800, 600);
    gl_FragColor = texture(ourTexture, vec3(uv, .32817)) + vec4(0.0, 0.0, 0.0, 1.0);
}
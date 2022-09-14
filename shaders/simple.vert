#version 430 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;

out vec4 v_color;

uniform mat4 scaling= mat4(
    -1, 0, 0, 0,
    0, -1, 0, 0,
    0, 0, 1, 0,
    0, 0, 0, 1);

void main()
{
    v_color = color;
    gl_Position = scaling*vec4(position.x, position.y, position.z, 1.0f);
}
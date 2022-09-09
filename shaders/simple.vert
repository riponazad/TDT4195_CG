#version 430 core

in vec3 position;

uniform mat4 scaling= mat4(
    -1, 0, 0, 0,
    0, -1, 0, 0,
    0, 0, 1, 0,
    0, 0, 0, 1);

void main()
{
    gl_Position = scaling*vec4(position.x, position.y, position.z, 1.0f);
}
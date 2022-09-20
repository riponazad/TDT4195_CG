#version 430 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;
//uniform layout(location=2) float elapsed;
uniform layout(location=20) mat4 rotation;
uniform layout(location=21) mat4 translation;
uniform layout(location=22) mat4 transform;
out vec4 v_color;

//mat4x4 trans= mat4(
//    1, 0, 0, 0,
//    0, 1, 0, elapsed,
//    0, 0, 1, 0,
//    0, 0, 0, 1);

void main()
{
    v_color = color;
    //gl_Position = trans*vec4(position.x, position.y, position.z, 1.0f);//vec4(position.x, position.y, position.z, 1.0f);
    gl_Position = transform*vec4(position.x, position.y, position.z, 1.0f);
}
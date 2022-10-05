#version 430 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;
layout(location=2) in vec3 normal;
//uniform layout(location=2) float elapsed;
uniform layout(location=3) mat4 transform;
uniform layout(location=8) mat4 trans; //TASK 5


out layout(location=9) vec4 v_color;
out layout(location=10) vec3 v_normal;

/* mat4x4 trans= mat4(
    1, 0, 0, 0,
    0, 1, 0, elapsed,
    0, 0, 1, 0,
    0, 0, 0, 1); */

void main()
{
    v_color = color;
    //v_color = vec4(normal[0], normal[1], normal[2], color[3]);
    v_normal = normalize(mat3(trans)*normal); //TASK 5
    gl_Position = transform*vec4(position.x, position.y, position.z, 1.0f);//vec4(position.x, position.y, position.z, 1.0f);
}
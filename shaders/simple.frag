#version 430 core

in vec4 v_color;

out vec4 color;

void main()
{
    color = v_color;
    //color = vec4(0.1f, 0.8f, 0.5f, 1.0f);
}
#version 430 core

in layout(location=9) vec4 in_color;
in layout(location=10) vec3 in_normal;

out vec4 color;


void main()
{
    vec3 lightDirection = normalize(vec3(0.8,-0.5, 0.6));
    float scalar = max(0.0, dot(-lightDirection, in_normal));
    color = vec4(scalar*in_color.xyz, in_color[3]);
    //color = vec4(0.1f, 0.8f, 0.5f, 1.0f);
}
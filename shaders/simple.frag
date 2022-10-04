#version 430 core

in vec4 v_color;
in vec3 v_normal;

out vec4 color;


void main()
{
    vec3 lightDirection = normalize(vec3(0.8,-0.5, 0.6));
    color = vec4(v_color.rgb * max(0.0, dot(v_normal, -lightDirection)), v_color.a);//v_color;
    //color = vec4(0.1f, 0.8f, 0.5f, 1.0f);
}
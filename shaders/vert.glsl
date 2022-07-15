#version 330 core
layout (location = 0) in vec3 Ipos;
layout (location = 1) in vec2 _tex_coord;

uniform mat4 model;
uniform mat4 proj;
uniform mat4 view;

out vec2 tex_coord;
out vec4 pos;

void main() {
    gl_Position = proj * view * model * vec4(Ipos.x, Ipos.y, Ipos.z, 1.0);
    tex_coord = _tex_coord;
}
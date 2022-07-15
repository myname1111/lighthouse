#version 330 core

in vec2 tex_coord;

uniform sampler2D tex_color;
uniform mat4 proj;

void main() {
    gl_FragColor = texture(tex_color, tex_coord);
}
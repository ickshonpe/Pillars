#version 140

in vec2 position;
in vec4 color;
out vec4 v_color;

uniform mat4 camera_matrix;

void main() {
    gl_Position = camera_matrix * vec4(position, 0.0, 1.0);
    v_color = color;
}
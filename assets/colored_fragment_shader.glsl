#version 140

//uniform vec4 v_color;
in vec4 v_color;
out vec4 color;

void main() {
    color = v_color;
}
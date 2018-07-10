#version 140

in vec2 position;
in vec2 tex_coords;
in vec4 color;
out vec2 v_tex_coords;
out vec4 v_color;
uniform mat4 camera_matrix;

void main() {
   v_tex_coords = tex_coords;
   v_color = color;
   gl_Position = camera_matrix * vec4(position, 0.0, 1.0);
}
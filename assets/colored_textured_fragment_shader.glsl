 #version 140

in vec2 v_tex_coords;
in vec4 v_color;
uniform sampler2D tex;
out vec4 color;

void main() {
    vec4 temp = texture2D(tex, v_tex_coords);
    vec4 temp2 = temp * v_color;
    color = temp2;
}
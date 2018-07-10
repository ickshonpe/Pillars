pub const TRIANGLE_VERTEX_SHADER_SRC: &'static str = r#"
#version 330 core

//layout (location = 0) in vec3 Position;
in vec3 Position;

void main()
{
    gl_Position = vec4(Position, 1.0);
}
"#;

pub const TRIANGLE_FRAGMENT_SHADER_SRC: &'static str = r#"
#version 330 core

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;


pub const VERTEX_SHADER_SRC: &'static str = r#"
    #version 330 core

    in vec2 position;

    uniform mat4 camera_matrix;

    void main() {
        gl_Position = camera_matrix * vec4(position, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 330 core

    uniform vec4 v_color;
    out vec4 color;

    void main() {
        //color = v_color;
          color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    }
"#;

pub const TEXTURED_VERTEX_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;

    uniform mat4 camera_matrix;

    void main() {
        v_tex_coords = tex_coords;

        gl_Position = camera_matrix * vec4(position, 0.0, 1.0);
    }
"#;

pub const TEXTURED_FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 v_tex_coords;
    uniform vec4 v_color;
    uniform sampler2D tex;
    out vec4 color;

    void main() {
        vec4 temp = texture2D(tex, v_tex_coords);
        vec4 temp2 = temp * v_color;
        color = temp2; // texture2D(tex, v_tex_coords);
    }
"#;


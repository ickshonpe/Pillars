pub const VERTEX_SHADER_SRC: &'static str = r#"
    #version 330 core

    in vec2 position;
    in vec2 tex_pos;
    in vec4 v_color;

    uniform mat4 camera_matrix;

    out VS_OUTPUT {
        vec2 tex_pos;
        vec4 v_color;
    } OUT;

    void main() {
        gl_Position = camera_matrix * vec4(position.xy, 0.0, 1.0);
        OUT.tex_pos = tex_pos;
        OUT.v_color = v_color;
    }
"#;

pub const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 330 core

    in VS_OUTPUT {
        vec2 tex_pos;
        vec4 v_color;
    } IN;

    uniform sampler2D tex;    
    out vec4 color;

    void main() {    
        color = texture2D(tex, IN.tex_pos) * IN.v_color;
    }
"#;

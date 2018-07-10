pub const VERTEX_SHADER: &'static str = r#"
#version 330 core

layout (location = 0) in vec3 Position;

void main()
{
    gl_Position = vec4(Position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
#version 330 core

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;



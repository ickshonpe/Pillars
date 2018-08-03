use rectangle::Rectangle;
use gl::types::GLuint;
use texture;

pub type Color = [f32; 4];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.4, 0.0, 1.0];

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct V2T2C4(pub Vector2, pub Vector2, pub Color);

pub type Matrix4 = [f32; 16];

pub type Vector2 = [f32; 2];

pub fn calculate_orthogonal_projection_matrix(size: [f32; 2], position: [f32; 2]) -> Matrix4 {
    let w = 2.0 / size[0];
    let h = 2.0 / size[1];
    let (tx, ty) = (position[0] * w, position[1] * h);
    [
        w,
        0.0,
        0.0,
        0.0,
        0.0,
        h,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        -1.0 - tx,
        -1.0 - ty,
        0.0,
        1.0,
    ]
}



pub struct GraphicsContext {
    pub window_rect: Rectangle,
    pub char_size: Vector2,
    pub charset: ::charset::Charset,
    pub shader_program: ::gl_util::ShaderProgram,
    pub charset_texture: texture::Texture,
    pub vertex_buffer: GLuint,
    pub vertex_attributes_array: GLuint,
    pub window: ::sdl2::video::Window,
    pub border_vertices: Vec<V2T2C4>,
    pub target: Vector2,
    pub cell_size: Vector2,
    pub cell_padding: Vector2,
    pub pillar_texture: texture::Texture,
    pub block_texture: texture::Texture
}





pub type Color = [f32; 4];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.4, 0.0, 1.0];



#[repr(C, packed)]
pub struct CVertex2(pub Vertex2, pub Color);

#[repr(C, packed)]
pub struct TCVertex2(pub Vertex2, pub Vertex2, pub Color);

#[repr(C, packed)]
pub struct TVertex2(pub Vertex2, pub Vertex2);

pub type Matrix4 = [f32; 16];
pub type Vertex3 = [f32; 3];
pub type Vertex2 = [f32; 2];

pub fn calculate_orthogonal_projection_matrix(size: [f32; 2], position: [f32; 2]) -> [f32; 16] {
    let w = 2.0 / size[0];
    let h = 2.0 / size[1];
    let (tx, ty) = (position[0] * w, position[1] * h);
    [
        w, 0.0, 0.0, 0.0,
        0.0, h, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0 - tx, -1.0 - ty, 0.0, 1.0
    ]
}

pub const IDENTITY_MATRIX4: [f32; 16] = [
    1.,0.,0.,0.,
    0.,1.,0.,0.,
    0.,0.,1.,0.,
    0.,0.,0.,1.
];


pub fn rotation_matrix(rads: f32) -> [f32; 16] {
    let c = rads.cos();
    let s = rads.sin();
    [
        c, -s,0.,0.,
        s,c,0.,0.,
        0.,0.,1.,0.,
        0.,0.,0.,1.
    ]
}

pub fn multiply_matrix(a: Matrix4, b: Matrix4) -> Matrix4 {
    let mut ret = [0.;16];
    for j in 0..4 {
    let  j4 = j * 4;
    ret[j4 + 0] = a[j4]*b[0] + a[j4 + 1]*b[0 + 4] + a[j4 + 2]*b[0 + 8] + a[j4 + 3]*b[0 + 12];
    ret[j4 + 1] = a[j4]*b[1] + a[j4 + 1]*b[1 + 4] + a[j4 + 2]*b[1 + 8] + a[j4 + 3]*b[1 + 12];
    ret[j4 + 2] = a[j4]*b[2] + a[j4 + 1]*b[2 + 4] + a[j4 + 2]*b[2 + 8] + a[j4 + 3]*b[2 + 12];
    ret[j4 + 3] = a[j4]*b[3] + a[j4 + 1]*b[3 + 4] + a[j4 + 2]*b[3 + 8] + a[j4 + 3]*b[3 + 12];
    }
    ret
}
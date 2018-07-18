


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
pub struct CVertex2(pub Vertex2, pub Color);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TCVertex2(pub Vertex2, pub Vertex2, pub Color);

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
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

pub const IDENTITY_MATRIX2: [f32; 4] = [
    1., 0.,
    0., 1.
];

pub const IDENTITY_MATRIX4: [f32; 16] = [
    1.,0.,0.,0.,    // first column
    0.,1.,0.,0.,    // second "
    0.,0.,1.,0.,    // third "
    0.,0.,0.,1.     // fourth "
];

// rotates about origin
pub fn rotation_matrix(rads: f32) -> [f32; 16] {
    let c = rads.cos();
    let s = rads.sin();
    [
        c, s, 0., 0.,
        -s, c, 0., 0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.
    ]
}

pub fn translation_matrix(translation: Vertex2) -> Matrix4 {
    let tx = translation[0];
    let ty = translation[1];
    [
        1., 0., 0., 0.,    // first column
        0., 1., 0., 0.,    // second "
        0., 0., 1., 0.,    // third "
        tx, ty, 0., 1.     // fourth "
    ]
}
pub fn multiply_matrices(a: Matrix4, b: Matrix4) -> Matrix4 {
    let mut out = [0.;16];
    for column in 0..4 {
        let row = column * 4;
        out[row + 0] = a[row]*b[0] + a[row + 1]*b[4] + a[row + 2]*b[8] + a[row + 3]*b[12];
        out[row + 1] = a[row]*b[1] + a[row + 1]*b[5] + a[row + 2]*b[9] + a[row + 3]*b[13];
        out[row + 2] = a[row]*b[2] + a[row + 1]*b[6] + a[row + 2]*b[10] + a[row + 3]*b[14];
        out[row + 3] = a[row]*b[3] + a[row + 1]*b[7] + a[row + 2]*b[11] + a[row + 3]*b[15];
    }
    out
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub position: Vertex2,
    pub size: Vertex2
}

impl Rectangle {
    pub fn left(&self) -> f32 {
        self.position[0]
    }
    pub fn right(&self) -> f32 {
        self.position[0] + self.size[0]
    }
    pub fn bottom(&self) -> f32 {
        self.position[0]
    }
    pub fn top(&self) -> f32 {
        self.position[1] + self.size[1]
    }
}
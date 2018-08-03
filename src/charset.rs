use graphics::{Color, V2T2C4, Vector2};

pub struct Charset {
    texture_vertices: Vec<[Vector2; 4]>,
}

impl Charset {
    pub fn new() -> Self {
        let mut vertices = Vec::with_capacity(16 * 16);
        let tile_width = 1.0f32 / 16.;
        let tile_height = 1.0f32 / 16.;
        for y in 0..16 {
            for x in 0..16 {
                let min_x = x as f32 * tile_width;
                let min_y = y as f32 * tile_height;
                let max_x = min_x + tile_width;
                let max_y = min_y + tile_height;
                let tvs = [
                    [min_x, max_y],
                    [min_x, min_y],
                    [max_x, min_y],
                    [max_x, max_y],
                ];
                vertices.push(tvs);
            }
        }
        Charset {
            texture_vertices: vertices,
        }
    }

    pub fn push_text_vertices(
        &self,
        vertices: &mut Vec<V2T2C4>,
        text: &[u8],
        position: Vector2,
        char_size: Vector2,
        color: Color,
    ) {
        for (i, c) in text.iter().enumerate() {            
            let x = c % 16;
            let y = c / 16;
            let out_c = y * 16 + x;
            let coords = self.texture_vertices[out_c as usize];
            let min_x = position[0] + char_size[0] * i as f32;
            let min_y = position[1];
            let max_x = min_x + char_size[0];
            let max_y = min_y + char_size[1];
            vertices.push(V2T2C4([min_x, min_y], coords[0], color));
            vertices.push(V2T2C4([max_x, max_y], coords[2], color));
            vertices.push(V2T2C4([min_x, max_y], coords[1], color));
            vertices.push(V2T2C4([min_x, min_y], coords[0], color));
            vertices.push(V2T2C4([max_x, max_y], coords[2], color));
            vertices.push(V2T2C4([max_x, min_y], coords[3], color));
        }
    }
}

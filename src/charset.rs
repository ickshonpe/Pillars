use graphics::{Color, TCVertex2, Vertex2};

pub struct Charset {
    texture_vertices: Vec<[Vertex2; 4]>,
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
        vertices: &mut Vec<TCVertex2>,
        text: &[u8],
        position: Vertex2,
        char_size: Vertex2,
        color: Color,
    ) {
        //let mut vertices = Vec::with_capacity(text.len() * 6);
        for i in 0..text.len() {
            let char = text[i];
            let x = char % 16;
            let y = char / 16;
            //let y = 15 - y;
            let char = y * 16 + x;
            let coords = self.texture_vertices[char as usize];
            let min_x = position[0] + char_size[0] * i as f32;
            let min_y = position[1];
            let max_x = min_x + char_size[0];
            let max_y = min_y + char_size[1];
            vertices.push(TCVertex2([min_x, min_y], coords[0], color));
            vertices.push(TCVertex2([max_x, max_y], coords[2], color));
            vertices.push(TCVertex2([min_x, max_y], coords[1], color));
            vertices.push(TCVertex2([min_x, min_y], coords[0], color));
            vertices.push(TCVertex2([max_x, max_y], coords[2], color));
            vertices.push(TCVertex2([max_x, min_y], coords[3], color));
        }
    }
}

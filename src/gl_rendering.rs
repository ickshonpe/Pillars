use board::Board;
use columns::Jewel;
use graphics::Vertex2;
use graphics::Color;
use graphics::TCVertex2;
use point2::Size2;

pub fn push_quad_vertices(vertex_buffer: &mut Vec<TCVertex2>, position: Vertex2, size: Vertex2, color: Color) {
    let min_x = position[0];
    let min_y = position[1];
    let max_x = position[0] + size[0];
    let max_y = position[1] + size[1];
    vertex_buffer.push(TCVertex2([min_x, min_y], [0.0, 0.0], color));
    vertex_buffer.push(TCVertex2([max_x, max_y], [1.0, 1.0], color));
    vertex_buffer.push(TCVertex2([min_x, max_y], [0.0, 1.0], color));
    vertex_buffer.push(TCVertex2([min_x, min_y], [0.0, 0.0], color));
    vertex_buffer.push(TCVertex2([max_x, max_y], [1.0, 1.0], color));
    vertex_buffer.push(TCVertex2([max_x, min_y], [1.0, 0.0], color));
}


pub fn draw_board(
    mut vertex_buffer: &mut Vec<TCVertex2>,
    board: &Board,
    column: ::columns::Column,
    target: Vertex2,
    tile_size: Vertex2,
    tile_padding: Vertex2) {
    
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32 + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32 + target[1];
                draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
            }
        }
    }

    let mut p = column.position;
    for i in 0..3 {
        let dest_x = (p.x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32 + target[0];
        let dest_y = (p.y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32 + target[1];
        draw_jewel(&mut vertex_buffer, [dest_x, dest_y],tile_size, column.jewels[i]);
        p.up();
    }
}

pub fn draw_column(
    mut vertex_buffer: &mut Vec<TCVertex2>,
    column: ::columns::Column,
    target: Vertex2,
    tile_size: Vertex2,
    tile_padding: Vertex2,
    alpha: f32) {
    let mut p = column.position;
    for i in 0..3 {
        let x = target[0] + p.x as f32 * tile_size[0];
        let y =  target[1] + p.y as f32 * tile_size[1];
        let mut color = column.jewels[i].color_gl();
        color[3] = alpha;
        let position = [x, y];
        push_quad_vertices(vertex_buffer, position, tile_size, color);
        p.up();
    }

}


fn draw_jewel(vertex_buffer: &mut Vec<TCVertex2>, target: Vertex2, size: Vertex2, jewel: Jewel) {
    let color = jewel.color_gl();    
    push_quad_vertices(vertex_buffer, target, size, color)
}


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

use std;
use point2::P2;
use graphics;
pub fn draw_board_highlight_matches(
    vertex_buffer: &mut Vec<TCVertex2>,
    board: &Board,
    matches: &std::collections::HashSet<P2>,
    target: Vertex2,
    tile_size: Vertex2,
    tile_padding: Vertex2) {
    
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32 + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32 + target[1];
                let p = P2::new(x, y);
                if matches.contains(&p) {
                    let color = graphics::WHITE;    
                    push_quad_vertices(vertex_buffer, [dest_x, dest_y], tile_size, color)
                } else {                
                    draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
                }
            }
        }
    }
}

pub fn draw_board_fade_matches(
    vertex_buffer: &mut Vec<TCVertex2>,
    board: &Board,
    matches: &std::collections::HashSet<P2>,
    alpha: f32,
    target: Vertex2,
    tile_size: Vertex2,
    tile_padding: Vertex2) {
    
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32 + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32 + target[1];
                let p = P2::new(x, y);
                if matches.contains(&p) {
                    let mut color = jewel.color_gl();
                    color[3] = alpha;
                    push_quad_vertices(vertex_buffer, [dest_x, dest_y], tile_size, color)
                } else {                
                    draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
                }
            }
        }
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

use gl_util::draw_textured_colored_quads;
use charset::Charset;
pub fn get_scores_display_strings(score: u64, high_score: u64, window_rect: graphics::Rectangle, char_size: Vertex2) -> Vec<(Vec<u8>, Vertex2)> {
    vec![
        (format!("{:06}", high_score).into_bytes(), [window_rect.left() + char_size[0] * 13., window_rect.top() - char_size[1] * 1.5]),
        (format!("{:06}", score).into_bytes(), [window_rect.left() + char_size[0] * 3., window_rect.top() - char_size[1] * 1.5])
    ]
}


pub fn draw_board_all_fading(
    vertex_buffer: &mut Vec<TCVertex2>,
    board: &Board,
    fading: &Vec<(P2, f32)>,    
    target: Vertex2,
    tile_size: Vertex2,
    tile_padding: Vertex2) {
    
    
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as f32 * (tile_size[0] + tile_padding[0]) + tile_padding[0]) as f32 + target[0];
                let dest_y = (y as f32 * (tile_size[1] + tile_padding[1]) + tile_padding[1]) as f32 + target[1];
                let p = P2::new(x, y);
                let mut done = false;
                for fader in fading.iter() {
                    if p == fader.0 {
                         let color = jewel.color_gl();                        
                        let fade = {
                            if fader.1 < 0. {
                                0.
                            } else {
                                fader.1
                            }
                        };
                        let y = fade;
                        let x = 1. - fade;

                        let faded_color = 
                            [
                                0.8 * x + color[0] * y,
                                0.8 * x + color[1] * y,
                                0.8 * x + color[2] * y,
                                1.0
                            ];
                        push_quad_vertices(vertex_buffer, [dest_x, dest_y], tile_size, faded_color);
                        done = true;
                        break;
                    }
                }

                if !done {
                     draw_jewel(vertex_buffer, [dest_x, dest_y], tile_size, jewel);
                }
            }
        }
    }
}

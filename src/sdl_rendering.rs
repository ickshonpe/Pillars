use board::*;
use columns::*;
use point2::*;
use sdl2::rect::Rect;
use columns::Jewel;
use sdl2::pixels::Color;
use graphics;
use sdl2;
use sdl2::render::*;

fn float_color_to_rgb(color: graphics::Color) -> Color {
    let r: u8 = (255f32 * color[0]) as u8;
    let g: u8 = (255f32 * color[1]) as u8;
    let b: u8 = (255f32 * color[2]) as u8;
    Color::RGB(r, g, b)
}

pub fn draw_board<T: RenderTarget>(
    mut canvas: &mut Canvas<T>,
    board: &Board,
    column: ::columns::Column,
    target: [i32; 2],
    tile_size: [u32; 2],
    tile_padding: [u32; 2]) {
    let viewport_height = canvas.viewport().height() as i32;
    for x in 0..board.width() {
        for y in 0..board.height() {
            if let Some(jewel) = board[x][y] {
                let dest_x = (x as u32 * tile_size[0] + tile_padding[0]) as i32 + target[0];
                let dest_y = viewport_height - (y as u32 * tile_size[1] + tile_padding[1]) as i32 - target[1] - 1;
                draw_jewel(&mut canvas,[dest_x, dest_y], tile_size, jewel);
            }
        }
    }

    let mut p = column.position;
    for i in 0..3 {
        let dest_x = (p.x as u32 * tile_size[0] + tile_padding[0]) as i32 + target[0];
        let dest_y = viewport_height - (p.y as u32 * tile_size[1] + tile_padding[1]) as i32 - target[1] - 1;
        draw_jewel(&mut canvas, [dest_x, dest_y],tile_size, column.jewels[i]);
        p.up();
    }
}

fn draw_jewel<T: RenderTarget>(canvas: &mut Canvas<T>, target: [i32; 2], size: [u32; 2], jewel: Jewel) {
    let color = float_color_to_rgb(jewel.color_gl());
    let rect = Rect::new(target[0], target[1], size[0], size[1]);
    canvas.set_draw_color(color);
    let _ = canvas.fill_rect(rect);
}


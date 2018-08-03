use gl;
use gl_rendering;
use gl_util;
use graphics;
use board::Board;
use columns::Column;
use std::collections::HashSet;
use point2::P2;

pub enum BoardDrawMode<'a> {
    Normal(&'a Board),
    Highlight(&'a Board, &'a HashSet<P2>),
    Fading(&'a Board, &'a HashSet<P2>, f32),
    GameOver(&'a Board, &'a [(P2, f32)])
}

pub fn draw_game(
    board: BoardDrawMode,
    falling_column: Option<Column>,
    next_column: Option<(Column, f32)>,
    current_score: u64,
    high_score: u64,
    ctx: &graphics::GraphicsContext
) {
    let mut board_vertices = Vec::new();
    
    if let Some((next_column, alpha)) = next_column {
        gl_rendering::draw_column(
            &mut board_vertices,
            next_column,
            ctx.target,
            ctx.cell_size,
            ctx.cell_padding,
            alpha
        );
    }

    if let BoardDrawMode::GameOver(board, fading) = board {
        gl_rendering::draw_board_all_fading(
            &mut board_vertices, 
            board, 
            fading, 
            ctx.target,
            ctx.cell_size, 
            ctx.cell_padding
        );
    }

    if let BoardDrawMode::Normal(board) = board {
        gl_rendering::draw_board(
            &mut board_vertices,
            board,
            falling_column,
            ctx.target,
            ctx.cell_size,
            ctx.cell_padding,
        );
    } 
    if let BoardDrawMode::Highlight(board, matches) = board {
        gl_rendering::draw_board_highlight_matches(
            &mut board_vertices,
            board,
            matches,
            ctx.target,
            ctx.cell_size,
            ctx.cell_padding,
        );
    } 
    if let BoardDrawMode::Fading(board, matches, alpha) = board {
        gl_rendering::draw_board_fade_matches(            
            &mut board_vertices,
            board,
            matches,
            alpha,
            ctx.target,
            ctx.cell_size,
            ctx.cell_padding,
        );
    } 
    


    let display_strings = 
        gl_rendering::get_scores_display_strings(
            current_score,
            high_score,
            ctx.window_rect,
            ctx.char_size,
        );

    let mut charset_vertices = Vec::new();
    for message in &display_strings {
        ctx.charset.push_text_vertices(
            &mut charset_vertices,
            &message.0,
            message.1,
            ctx.char_size,
            graphics::WHITE,
        );
    }
    // draw all pillars
    gl_util::draw_textured_colored_quads(
        &board_vertices,
        &ctx.shader_program,
        &ctx.pillar_texture,
        ctx.vertex_buffer,
        ctx.vertex_attributes_array,
    );

    gl_util::draw_textured_colored_quads(
        &ctx.border_vertices,
        &ctx.shader_program,
        &ctx.block_texture,
        ctx.vertex_buffer,
        ctx.vertex_attributes_array,
    );

    gl_util::draw_textured_colored_quads(
        &charset_vertices,
        &ctx.shader_program,
        &ctx.charset_texture,
        ctx.vertex_buffer,
        ctx.vertex_attributes_array,
    );
}

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
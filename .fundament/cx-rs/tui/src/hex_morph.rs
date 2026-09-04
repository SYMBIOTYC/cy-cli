//! SYMBIOTYC hexagon morph animation for the top-right corner.
//!
//! Smoothly morphs between a square and a hexagon shape, rendered in black/pink.
//! The animation loops continuously with a gentle easing curve.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;

/// Animation duration for one full morph cycle (square → hex → square).
const MORPH_CYCLE_MS: u64 = 4000;

/// Get the current morph progress (0.0 = square, 1.0 = hex).
fn morph_progress() -> f32 {
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let t_raw = (ms % MORPH_CYCLE_MS) as f32 / MORPH_CYCLE_MS as f32;
    // Smooth easing: sin wave mapped to 0..1
    (t_raw * std::f32::consts::PI).sin() * 0.5 + 0.5
}

/// Build a single frame of the morph animation as a 5×3 char grid.
fn build_morph_frame(t: f32) -> [[char; 5]; 3] {
    let mut grid = [[' '; 5]; 3];

    let top_indent = t;
    let bot_indent = t;

    // Row 0: top edge
    let tl = (top_indent * 1.0) as usize;
    let tr = 4 - (top_indent * 1.0) as usize;
    for x in tl..=tr {
        if x == tl || x == tr {
            grid[0][x] = '█';
        } else {
            grid[0][x] = '▄';
        }
    }

    // Row 1: sides
    grid[1][0] = '█';
    grid[1][4] = '█';

    // Row 2: bottom edge
    let bl = (bot_indent * 1.0) as usize;
    let br = 4 - (bot_indent * 1.0) as usize;
    for x in bl..=br {
        if x == bl || x == br {
            grid[2][x] = '█';
        } else {
            grid[2][x] = '▀';
        }
    }

    grid
}

/// Render the SYMBIOTYC hexagon morph at the given position in a buffer.
pub(crate) fn render_hex_morph(area: Rect, buf: &mut Buffer) {
    if area.width < 5 || area.height < 3 {
        return;
    }

    let t = morph_progress();
    let pink = Color::Rgb(255, 0, 128);
    let style = Style::default().fg(pink);

    let rows = build_morph_frame(t);
    for (y, row) in rows.iter().enumerate() {
        if y as u16 >= area.height {
            break;
        }
        for (x, ch) in row.iter().enumerate() {
            if x as u16 >= area.width {
                break;
            }
            if *ch != ' ' {
                buf.set_span(
                    area.x + x as u16,
                    area.y + y as u16,
                    &Span::styled(ch.to_string(), style),
                    1,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morph_frame_square() {
        let grid = build_morph_frame(0.0);
        assert_eq!(grid[0], ['█', '▄', '▄', '▄', '█']);
        assert_eq!(grid[1], ['█', ' ', ' ', ' ', '█']);
        assert_eq!(grid[2], ['█', '▀', '▀', '▀', '█']);
    }

    #[test]
    fn morph_frame_hex() {
        let grid = build_morph_frame(1.0);
        assert_eq!(grid[0], [' ', '█', '▄', '█', ' ']);
        assert_eq!(grid[1], ['█', ' ', ' ', ' ', '█']);
        assert_eq!(grid[2], [' ', '█', '▀', '█', ' ']);
    }
}

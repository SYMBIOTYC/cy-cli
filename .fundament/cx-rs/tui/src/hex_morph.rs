//! SYMBIOTYC hexagon indicator for the top-right corner.
//!
//! - Dimmed quietly when the model is idle.
//! - Leans left (row shear in a 3×3 grid) every 3-10 seconds while thinking.
//! - Occasional random green flash.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;

const GRID_W: usize = 3;
const GRID_H: usize = 3;

const ROT_MIN_MS: u64 = 3000;
const ROT_MAX_MS: u64 = 10000;

fn time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn xorshift32(seed: u64) -> u32 {
    let mut s = seed;
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    (s as u32).wrapping_mul(0x9E3779B9)
}

const THEME_PINK: u8 = 0xFF;
const THEME_RED: u8 = 0x3D;
const THEME_MAGENTA: u8 = 0xA8;

fn build_frame(thinking: bool) -> ([[char; GRID_W]; GRID_H], bool, bool) {
    let ms = time_ms();

    // Stable rotation interval: changes once per second, varies 3-10s per phase.
    let sec = ms / 1000;
    let rot_interval = ROT_MIN_MS + ((sec.wrapping_mul(0x9E3779B9u64)) % (ROT_MAX_MS - ROT_MIN_MS));
    let rot_phase = (ms / rot_interval) % 2; // 0=center, 1=shifted

    // Random pink flash: rare (1/60 frames ≈ 0.5s at 30fps).
    let flash = xorshift32(ms / 33) % 60 == 0;

    let dimmed = !thinking;

    let mut grid: [[char; GRID_W]; GRID_H] = [[' '; GRID_W]; GRID_H];

    // Simulate 30° lean by shifting rows: left lean = all cols shift -1,
    // right lean = all cols shift +1. Out-of-bounds cells are omitted.
    let col_offset: isize = match rot_phase {
        1 => -1,
        _ => 0,
    };

    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let shifted = x as isize + col_offset;
            if shifted < 0 || shifted >= GRID_W as isize {
                continue;
            }
            grid[y][shifted as usize] = base_hex_char(y, x);
        }
    }

    (grid, dimmed, flash)
}

fn base_hex_char(y: usize, x: usize) -> char {
    match (y, x) {
        (0, 1) => '▄',
        (1, 0) | (1, 2) => '█',
        (2, 1) => '▀',
        _ => ' ',
    }
}

const BG_DARK: ratatui::style::Color = ratatui::style::Color::Rgb(0, 0, 0);
const FG_LIGHT: ratatui::style::Color = ratatui::style::Color::Rgb(234, 232, 230);
const ACCENT_MUTED: ratatui::style::Color = ratatui::style::Color::Rgb(138, 106, 120);
const ACCENT_GREEN: ratatui::style::Color = ratatui::style::Color::Rgb(122, 154, 136);
const DIM_GRAY: ratatui::style::Color = ratatui::style::Color::Rgb(108, 108, 114);

pub(crate) fn render_hex_morph(area: Rect, buf: &mut Buffer, thinking: bool) {
    if area.width < 1 || area.height < 1 {
        return;
    }

    let (grid, dimmed, flash) = build_frame(thinking);

    let color = if flash {
        ACCENT_GREEN
    } else if dimmed {
        DIM_GRAY
    } else {
        ACCENT_MUTED
    };

    let style = if dimmed && !flash {
        Style::default().fg(color)
    } else {
        Style::default().fg(color)
    };

    for (y, row) in grid.iter().enumerate() {
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
    fn base_hex_shape() {
        assert_eq!(base_hex_char(0, 0), ' ');
        assert_eq!(base_hex_char(0, 1), '▄');
        assert_eq!(base_hex_char(1, 0), '█');
        assert_eq!(base_hex_char(1, 1), ' ');
        assert_eq!(base_hex_char(1, 2), '█');
        assert_eq!(base_hex_char(2, 0), ' ');
        assert_eq!(base_hex_char(2, 1), '▀');
        assert_eq!(base_hex_char(2, 2), ' ');
    }

    #[test]
    fn build_frame_always_valid() {
        for thinking in [false, true] {
            let (grid, _, _) = build_frame(thinking);
            assert_eq!(grid.len(), GRID_H);
            for row in &grid {
                assert_eq!(row.len(), GRID_W);
            }
        }
    }
}

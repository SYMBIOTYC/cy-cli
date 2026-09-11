//! SYMBIOTYC hexagon indicator for the top-right corner.
//!
//! - Idle: static white hexagon ⬡.
//! - Thinking: alternates between ⬡ and ⬢ to simulate 30° rotation.
//! - Occasional subtle flash on thinking state.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;

const FG_LIGHT: Color = Color::Rgb(234, 232, 230);
const DIM_GRAY: Color = Color::Rgb(108, 108, 114);
const ACCENT_MUTED: Color = Color::Rgb(138, 106, 120);
const ACCENT_GREEN: Color = Color::Rgb(122, 154, 136);

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

pub(crate) fn render_hex_morph(area: Rect, buf: &mut Buffer, thinking: bool) {
    if area.width < 1 || area.height < 1 {
        return;
    }

    let ms = time_ms();
    let tick = ms / 120;

    let (symbol, color) = if !thinking {
        ("⬡", FG_LIGHT)
    } else {
        let flash = xorshift32(ms / 33) % 60 == 0;
        let idx = (tick / 8) % 2;
        let ch = if idx == 0 { '⬡' } else { '⬢' };
        let color = if flash {
            ACCENT_GREEN
        } else {
            ACCENT_MUTED
        };
        (ch.to_string(), color)
    };

    let style = Style::default().fg(color);

    for y in 0..area.height {
        for x in 0..area.width {
            if x as usize >= symbol.chars().count() {
                break;
            }
            let ch = symbol.chars().next().unwrap_or(' ');
            if ch != ' ' {
                buf.set_span(
                    area.x + x,
                    area.y + y,
                    &Span::styled(ch.to_string(), style),
                    1,
                );
            }
        }
        if y == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_hex_morph_does_not_panic_when_area_is_empty() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 0, 0));
        render_hex_morph(Rect::new(0, 0, 0, 0), &mut buf, false);
    }

    #[test]
    fn render_hex_morph_renders_idle_symbol() {
        let area = Rect::new(0, 0, 1, 1);
        let mut buf = Buffer::empty(area);
        render_hex_morph(area, &mut buf, false);
        let cell = &buf[(0, 0)];
        assert_eq!(cell.symbol(), "⬡");
    }
}

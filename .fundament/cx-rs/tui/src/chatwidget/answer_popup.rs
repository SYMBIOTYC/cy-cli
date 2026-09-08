//! Transient popup that surfaces a CY answer computed while the main turn is running.
//!
//! The popup is deliberately lightweight: a single bordered box rendered in the chat layout
//! slot directly above the composer. It never interrupts the active turn, never steals focus
//! from bottom-pane views, and disappears on `Esc` or after [`ANSWER_POPUP_TTL`].
//!
//! The state lives in a module-owned singleton (mirrored `Option` semantics) instead of a
//! `ChatWidget` field because the popup is a single global surface: at most one popup is
//! visible at a time, regardless of which thread widget is attached, and background subagent
//! tasks update it through `AppEvent::ShowAnswerPopup` without touching turn state.

use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use ratatui::widgets::Wrap;

use crate::render::renderable::Renderable;

/// The popup dismisses itself once this TTL elapses without an explicit close.
///
/// Showing the popup schedules a delayed frame at exactly this deadline (see
/// `ChatWidget::show_answer_popup`) so expiry repaints even when nothing else animates.
pub(crate) const ANSWER_POPUP_TTL: Duration = Duration::from_secs(/*secs*/ 15);

const ANSWER_POPUP_TITLE: &str = " CY answer ";
const ANSWER_POPUP_MAX_WIDTH: u16 = 72;
const ANSWER_POPUP_MAX_HEIGHT: u16 = 12;
const ANSWER_POPUP_MIN_HEIGHT: u16 = 3;
/// Horizontal breathing room subtracted from the chat width before clamping to the max width.
const ANSWER_POPUP_WIDTH_MARGIN: u16 = 4;
/// Left/right border columns inside the popup box.
const ANSWER_POPUP_BORDER_COLUMNS: u16 = 2;
/// Top/bottom border rows inside the popup box.
const ANSWER_POPUP_BORDER_ROWS: u16 = 2;

/// One popup snapshot: the answer text plus when it appeared (for TTL expiry).
pub(crate) struct AnswerPopupState {
    pub(crate) text: String,
    pub(crate) created: Instant,
}

static ANSWER_POPUP: Mutex<Option<AnswerPopupState>> = Mutex::new(None);

/// Show (or replace) the popup with `text` and reset its TTL clock.
pub(crate) fn show(text: String) {
    with_popup(|slot| {
        *slot = Some(AnswerPopupState {
            text,
            created: Instant::now(),
        });
    });
}

/// Close the popup immediately.
pub(crate) fn dismiss() {
    with_popup(|slot| *slot = None);
}

/// True while a non-expired popup is visible; sweeps an expired popup as a side effect.
pub(crate) fn popup_is_open() -> bool {
    with_popup(|slot| {
        expire_if_stale(slot);
        slot.is_some()
    })
    .unwrap_or(false)
}

/// Build the renderable for the popup slot above the composer, or `None` when closed/expired.
pub(crate) fn answer_popup_renderable() -> Option<Box<dyn Renderable>> {
    with_popup(|slot| {
        expire_if_stale(slot);
        slot.as_ref().map(|state| {
            Box::new(AnswerPopupRenderable {
                text: state.text.clone(),
            }) as Box<dyn Renderable>
        })
    })
    .flatten()
}

/// Insert `text` into the popup with an explicit creation time (TTL tests).
#[cfg(test)]
fn show_with_created(text: String, created: Instant) {
    with_popup(|slot| *slot = Some(AnswerPopupState { text, created }));
}

fn with_popup<R>(f: impl FnOnce(&mut Option<AnswerPopupState>) -> R) -> Option<R> {
    // A poisoned lock only means a previous holder panicked mid-update; fall back to "closed"
    // instead of taking the whole render path down with it.
    ANSWER_POPUP.lock().ok().map(|mut slot| f(&mut slot))
}

fn expire_if_stale(slot: &mut Option<AnswerPopupState>) {
    if let Some(state) = slot
        && state.created.elapsed() >= ANSWER_POPUP_TTL
    {
        *slot = None;
    }
}

struct AnswerPopupRenderable {
    text: String,
}

impl AnswerPopupRenderable {
    fn popup_width(width: u16) -> u16 {
        width
            .saturating_sub(ANSWER_POPUP_WIDTH_MARGIN)
            .clamp(1, ANSWER_POPUP_MAX_WIDTH)
    }
}

impl Renderable for AnswerPopupRenderable {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let width = Self::popup_width(area.width).min(area.width);
        let height = self.desired_height(area.width).min(area.height);
        if width == 0 || height == 0 {
            return;
        }
        let x = area.x + area.width.saturating_sub(width) / 2;
        let popup_area = Rect::new(x, area.y, width, height);
        Clear.render(popup_area, buf);
        let block = Block::bordered().title(ANSWER_POPUP_TITLE);
        let inner = block.inner(popup_area);
        block.render(popup_area, buf);
        Paragraph::new(self.text.as_str())
            .wrap(Wrap { trim: false })
            .render(inner, buf);
    }

    fn desired_height(&self, width: u16) -> u16 {
        let inner_width = Self::popup_width(width)
            .saturating_sub(ANSWER_POPUP_BORDER_COLUMNS)
            .max(1);
        let wrapped_lines = Paragraph::new(self.text.as_str())
            .wrap(Wrap { trim: false })
            .line_count(inner_width);
        let height = u16::try_from(wrapped_lines)
            .unwrap_or(u16::MAX)
            .saturating_add(ANSWER_POPUP_BORDER_ROWS);
        height.clamp(ANSWER_POPUP_MIN_HEIGHT, ANSWER_POPUP_MAX_HEIGHT)
    }
}

#[cfg(test)]
#[path = "answer_popup_tests.rs"]
mod tests;

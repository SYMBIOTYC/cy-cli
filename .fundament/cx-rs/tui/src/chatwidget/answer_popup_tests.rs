use super::*;
use pretty_assertions::assert_eq;

fn buffer_rows(buffer: &Buffer) -> Vec<String> {
    buffer
        .content
        .chunks(usize::from(buffer.area.width))
        .map(|row| {
            row.iter()
                .map(ratatui::buffer::Cell::symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

#[test]
fn popup_height_is_clamped_between_min_and_max() {
    let short = AnswerPopupRenderable {
        text: "hi".to_string(),
    };
    assert_eq!(short.desired_height(/*width*/ 100), 3);

    let long = AnswerPopupRenderable {
        text: "word ".repeat(/*n*/ 500),
    };
    assert_eq!(long.desired_height(/*width*/ 100), 12);

    // Width clamps to min(72, width - 4), so a narrow terminal shrinks the box.
    assert_eq!(long.desired_height(/*width*/ 20), 12);
}

#[test]
fn popup_lifecycle_show_render_expire_dismiss() {
    // Single sequential test: the popup state is a process-wide singleton.
    dismiss();
    assert!(!popup_is_open());
    assert!(answer_popup_renderable().is_none());

    show("Hello from CY".to_string());
    assert!(popup_is_open());
    let renderable = answer_popup_renderable().expect("open popup must render");
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 12,
    );
    let mut buffer = Buffer::empty(area);
    renderable.render(area, &mut buffer);
    let rows = buffer_rows(&buffer);
    assert!(
        rows.iter().any(|row| row.contains("CY answer")),
        "missing popup title in {rows:?}"
    );
    assert!(
        rows.iter().any(|row| row.contains("Hello from CY")),
        "missing popup body in {rows:?}"
    );

    // The popup box is centered at width min(72, 100 - 4) = 72 → x = 14.
    let border_row = rows.first().expect("at least one row");
    assert_eq!(border_row.find('┌'), Some(14));

    dismiss();
    assert!(!popup_is_open());
    assert!(answer_popup_renderable().is_none());

    // A popup older than the TTL sweeps itself on the next access.
    let expired = Instant::now()
        .checked_sub(ANSWER_POPUP_TTL)
        .expect("TTL must stay representable");
    show_with_created("stale answer".to_string(), expired);
    assert!(!popup_is_open());
    assert!(answer_popup_renderable().is_none());
}

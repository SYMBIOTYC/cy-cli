//! Question detection and the lightweight read-only subagent behind the CY answer popup.
//!
//! When the user sends a question while the main turn is still running, the main input track
//! can hand the text to [`ChatWidget::maybe_intercept_inflight_question`] (single call, no
//! turn interruption). The helper acknowledges the question in the popup and spawns a
//! throwaway subagent that makes one direct `chat/completions` call against the configured
//! provider — no tool loop, no shell, no browser — then delivers the answer through
//! `AppEvent::ShowAnswerPopup`.

use std::time::Duration;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;

use super::answer_popup;
use crate::app_event::AppEvent;
use crate::chatwidget::ChatWidget;
use crate::chatwidget::DEFAULT_OPENAI_BASE_URL;
use crate::text_formatting::truncate_text;

/// Shown immediately after a question is intercepted, until the subagent answer lands.
const QUESTION_ACKNOWLEDGED_MESSAGE: &str = "(question received, computing…)";
/// Short read-only system prompt: the subagent must stay cheap and side-effect free.
const ANSWER_POPUP_SYSTEM_PROMPT: &str = "You are CY, briefly answering a side question shown in a small terminal popup while a main task keeps running. You are strictly read-only and have no tools: never run commands and never modify anything. Answer directly, in at most 6 sentences of plain text (no markdown, no code blocks). If you are not confident, say so in one short sentence.";
/// One-shot request budget for the subagent call.
const ANSWER_POPUP_REQUEST_TIMEOUT: Duration = Duration::from_secs(/*secs*/ 45);
/// Questions are clipped before they travel: the popup flow is for short side questions.
const QUESTION_MAX_GRAPHEMES: usize = 1200;
/// Answers are clipped so the popup stays within its small box.
const ANSWER_MAX_GRAPHEMES: usize = 800;

/// English yes/no and wh- question openers.
const QUESTION_OPENERS_EN: &[&str] = &[
    "who", "whom", "whose", "what", "where", "when", "why", "how", "which", "can", "could",
    "should", "would", "will", "shall", "may", "might", "must", "is", "are", "am", "do", "does",
    "did", "has", "have", "had", "was", "were",
];

/// Russian wh- question openers.
const QUESTION_OPENERS_RU: &[&str] = &[
    "кто",
    "что",
    "где",
    "когда",
    "куда",
    "откуда",
    "почему",
    "зачем",
    "как",
    "какой",
    "какая",
    "какое",
    "какие",
    "каким",
    "какими",
    "сколько",
    "чей",
    "чья",
    "чьё",
    "чье",
    "чьи",
];

/// Russian yes/no openers that only count as questions in the "<word> ли …" form.
const QUESTION_OPENERS_RU_LI: &[&str] = &[
    "можно",
    "нужно",
    "надо",
    "стоит",
    "есть",
    "верно",
    "правда",
    "так",
];

/// True when `text` reads as a question: it ends with `?`/`؟` or starts with an
/// English/Russian question word.
pub(crate) fn is_question(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.ends_with('?') || trimmed.ends_with('؟') {
        return true;
    }

    let lowered = trimmed.to_lowercase();
    let first_word = lowered
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|c: char| !c.is_alphanumeric());
    if QUESTION_OPENERS_EN.contains(&first_word) || QUESTION_OPENERS_RU.contains(&first_word) {
        return true;
    }

    QUESTION_OPENERS_RU_LI
        .iter()
        .any(|opener| starts_with_li_form(&lowered, opener))
}

/// Matches the Russian "<opener> ли …" yes/no question form (e.g. "можно ли …").
fn starts_with_li_form(lowered: &str, opener: &str) -> bool {
    let Some(rest) = lowered.strip_prefix(opener) else {
        return false;
    };
    let mut words = rest.split_whitespace();
    words
        .next()
        .is_some_and(|word| word.trim_matches(|c: char| !c.is_alphanumeric()) == "ли")
}

impl ChatWidget {
    /// One-line hook for the main input track: intercept a question submitted while a turn is
    /// running, acknowledge it in the popup, and compute the answer with a lightweight
    /// read-only subagent. Returns `true` when `text` was a question and the flow started.
    ///
    /// The active turn is never interrupted: the acknowledgement and the eventual answer only
    // travel through `AppEvent::ShowAnswerPopup`. The main input track calls this once per
    // in-flight plain-text message (see `submit_user_message_with_history_and_shell_escape_policy`).
    pub(crate) fn maybe_intercept_inflight_question(&self, text: &str) -> bool {
        if !is_question(text) {
            return false;
        }
        self.show_answer_popup(QUESTION_ACKNOWLEDGED_MESSAGE.to_string());
        self.spawn_question_subagent(truncate_text(text.trim(), QUESTION_MAX_GRAPHEMES));
        true
    }

    /// Show the popup and guarantee a repaint at TTL expiry even when nothing else animates.
    pub(crate) fn show_answer_popup(&self, text: String) {
        answer_popup::show(text);
        self.frame_requester.schedule_frame();
        self.frame_requester
            .schedule_frame_in(answer_popup::ANSWER_POPUP_TTL);
    }

    /// Close the popup and repaint.
    pub(crate) fn dismiss_answer_popup(&self) {
        answer_popup::dismiss();
        self.frame_requester.schedule_frame();
    }

    /// `Esc` closes the popup before the bottom-pane view stack sees the key.
    /// Returns `true` when the key event was consumed. Dismissal travels through
    /// `AppEvent::DismissAnswerPopup` so every popup transition is event-sourced.
    pub(crate) fn handle_answer_popup_key_event(&self, key_event: KeyEvent) -> bool {
        if key_event.code == KeyCode::Esc
            && key_event.kind == KeyEventKind::Press
            && answer_popup::popup_is_open()
        {
            self.app_event_tx.send(AppEvent::DismissAnswerPopup);
            return true;
        }
        false
    }

    /// Spawn the throwaway subagent: one direct `chat/completions` call, no tool loop.
    fn spawn_question_subagent(&self, question: String) {
        let http_client = self.pet_http_client.clone();
        let app_event_tx = self.app_event_tx.clone();
        let base_url = self
            .runtime_model_provider_base_url
            .clone()
            .or_else(|| self.config.model_provider.base_url.clone())
            .unwrap_or_else(|| DEFAULT_OPENAI_BASE_URL.to_string());
        let api_key = self.config.model_provider.api_key().ok().flatten();
        let model = {
            let model = self.current_model().trim();
            (!model.is_empty()).then(|| model.to_string())
        };

        let request = async move {
            compute_question_answer(
                &http_client,
                &base_url,
                api_key.as_deref(),
                model.as_deref(),
                &question,
            )
            .await
        };

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            std::mem::drop(handle.spawn(async move {
                app_event_tx.send(answer_popup_event_for(request.await));
            }));
        } else {
            let _ = std::thread::spawn(move || {
                let result = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime.block_on(request),
                    Err(err) => Err(format!("failed to start answer popup runtime: {err}")),
                };
                app_event_tx.send(answer_popup_event_for(result));
            });
        }
    }
}

fn answer_popup_event_for(result: Result<String, String>) -> AppEvent {
    let text = match result {
        Ok(answer) => answer,
        Err(err) => format!("(answer unavailable: {err})"),
    };
    AppEvent::ShowAnswerPopup { text }
}

async fn compute_question_answer(
    http_client: &cx_http_client::RouteAwareClientPool,
    base_url: &str,
    api_key: Option<&str>,
    model: Option<&str>,
    question: &str,
) -> Result<String, String> {
    let mut body = serde_json::json!({
        "messages": [
            { "role": "system", "content": ANSWER_POPUP_SYSTEM_PROMPT },
            { "role": "user", "content": question },
        ],
        "stream": false,
    });
    if let Some(model) = model {
        body["model"] = serde_json::Value::String(model.to_string());
    }

    let mut request = http_client
        .post(chat_completions_url(base_url))
        .json(&body)
        .timeout(ANSWER_POPUP_REQUEST_TIMEOUT);
    if let Some(api_key) = api_key {
        request = request.header("Authorization", format!("Bearer {api_key}"));
    }

    let response = request.send().await.map_err(|err| err.to_string())?;
    let status = response.status();
    let payload = response.text().await.map_err(|err| err.to_string())?;
    if !status.is_success() {
        return Err(format!("provider returned {status}"));
    }
    let value: serde_json::Value =
        serde_json::from_str(&payload).map_err(|err| format!("bad provider response: {err}"))?;
    let answer = value
        .pointer("/choices/0/message/content")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    if answer.is_empty() {
        return Err("provider returned an empty answer".to_string());
    }
    Ok(truncate_text(answer, ANSWER_MAX_GRAPHEMES))
}

/// Provider base URLs point at the model API root (`…/v1`) or, for the Responses wire API,
/// at `…/responses`; the subagent always speaks plain chat completions.
fn chat_completions_url(base_url: &str) -> String {
    let trimmed = base_url.trim_end_matches('/');
    match trimmed.strip_suffix("/responses") {
        Some(root) => format!("{root}/chat/completions"),
        None => format!("{trimmed}/chat/completions"),
    }
}

#[cfg(test)]
#[path = "answer_popup_backend_tests.rs"]
mod tests;

use super::*;
use pretty_assertions::assert_eq;

#[test]
fn detects_english_and_russian_questions() {
    assert!(is_question("How do I reset the session?"));
    assert!(is_question("what does this error mean"));
    assert!(is_question("Can you summarize the diff"));
    assert!(is_question("Как отменить последний коммит"));
    assert!(is_question("почему терн завис?"));
    assert!(is_question("можно ли прервать задачу"));
    assert!(is_question("Есть ли способ вернуть черновик"));
    assert!(is_question("plain statement turned question ?"));
    assert!(is_question("что делать؟"));
}

#[test]
fn ignores_non_questions() {
    assert!(!is_question(""));
    assert!(!is_question("   "));
    assert!(!is_question("run the tests and fix failures"));
    assert!(!is_question("почини билд"));
    assert!(!is_question("можно настроить тему"));
    assert!(!is_question("Howe street address update"));
    assert!(!is_question("whoa, that worked"));
}

#[test]
fn chat_completions_url_handles_provider_shapes() {
    assert_eq!(
        chat_completions_url("https://api.cy.example/v1"),
        "https://api.cy.example/v1/chat/completions"
    );
    assert_eq!(
        chat_completions_url("https://api.cy.example/v1/"),
        "https://api.cy.example/v1/chat/completions"
    );
    assert_eq!(
        chat_completions_url("https://api.cy.example/v1/responses"),
        "https://api.cy.example/v1/chat/completions"
    );
    assert_eq!(
        chat_completions_url("http://127.0.0.1:8790/v1/responses/"),
        "http://127.0.0.1:8790/v1/chat/completions"
    );
}

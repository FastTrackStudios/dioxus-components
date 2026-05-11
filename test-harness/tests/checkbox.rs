//! Tests for `dioxus_primitives::checkbox::Checkbox` driven by the
//! `dioxus-test` library from https://github.com/DioxusLabs/dioxus/pull/5323.

use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator, CheckboxState};
use dioxus_test::{Key, attr, by_testid, contains_string, eq, render, text};

fn checkbox(initial: CheckboxState) -> impl Fn() -> Element + Clone + 'static {
    move || {
        rsx! {
            Checkbox {
                default_checked: initial,
                "data-testid": "cb",
                CheckboxIndicator {
                    "data-testid": "indicator",
                    "X"
                }
            }
        }
    }
}

#[tokio::test]
async fn renders_with_unchecked_state() {
    let mut tester = render(checkbox(CheckboxState::Unchecked)).build();
    tester
        .query(by_testid("cb"))
        .expect(attr("data-state", eq("unchecked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn renders_with_checked_state() {
    let mut tester = render(checkbox(CheckboxState::Checked)).build();
    tester
        .query(by_testid("cb"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_flips_state_from_unchecked_to_checked() {
    let mut tester = render(checkbox(CheckboxState::Unchecked)).build();

    tester.query(by_testid("cb")).click().await.unwrap();

    tester
        .query(by_testid("cb"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_flips_state_from_checked_to_unchecked() {
    let mut tester = render(checkbox(CheckboxState::Checked)).build();

    tester.query(by_testid("cb")).click().await.unwrap();

    tester
        .query(by_testid("cb"))
        .expect(attr("data-state", eq("unchecked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn key_down_event_delivery() {
    // Custom component with an explicit `onkeydown` handler so we can verify that
    // `dioxus-test`'s synthetic keyboard event reaches the Dioxus runtime.
    //
    // Note: the primitives' Checkbox doesn't currently wire Space to toggle (it relies on
    // the browser's `role="checkbox"` keystroke → click translation, which we don't get
    // in the headless renderer). So we test the event-delivery path, not Checkbox a11y.
    #[component]
    fn KeyLog() -> Element {
        let mut last = use_signal(String::new);
        rsx! {
            div {
                "data-testid": "kl",
                tabindex: 0,
                onkeydown: move |e| {
                    last.set(format!("{:?}", e.key()));
                },
                "{last}"
            }
        }
    }

    let mut tester = render(KeyLog).build();

    tester
        .query(by_testid("kl"))
        .key_down(Key::Character(" ".into()))
        .await
        .unwrap();

    tester
        .query(by_testid("kl"))
        .expect(text(contains_string("Character")))
        .await
        .unwrap();
}

#[tokio::test]
async fn indicator_renders_children_only_when_checked() {
    let mut tester = render(checkbox(CheckboxState::Unchecked)).build();

    tester
        .query(by_testid("indicator"))
        .expect(text(eq("")))
        .await
        .unwrap();

    tester.query(by_testid("cb")).click().await.unwrap();

    tester
        .query(by_testid("indicator"))
        .expect(text(contains_string("X")))
        .await
        .unwrap();
}

#[tokio::test]
async fn disabled_checkbox_reports_data_disabled() {
    let mut tester = render(|| {
        rsx! {
            Checkbox {
                disabled: true,
                "data-testid": "cb",
                CheckboxIndicator { "X" }
            }
        }
    })
    .build();
    tester
        .query(by_testid("cb"))
        .expect(attr("data-disabled", eq("true")))
        .await
        .unwrap();
}

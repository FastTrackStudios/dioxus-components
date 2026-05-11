//! Port of `playwright/select.spec.ts` (browser-only Tab and pointer-driven option selection
//! are out of scope; the rest of the keyboard surface is covered).

use dioxus::prelude::*;
use dioxus_primitives::select::{
    Select, SelectList, SelectOption, SelectTrigger, SelectValue,
};
use dioxus_test::{Key, attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Select::<String> {
                "data-testid": "root",
                SelectTrigger {
                    "data-testid": "trigger",
                    aria_label: "Select Trigger",
                    SelectValue { placeholder: "Select a fruit..." }
                }
                SelectList {
                    "data-testid": "list",
                    aria_label: "Select Demo",
                    SelectOption::<String> {
                        index: 0usize,
                        value: "apple",
                        "data-testid": "option-apple",
                        "Apple"
                    }
                    SelectOption::<String> {
                        index: 1usize,
                        value: "banana",
                        "data-testid": "option-banana",
                        "Banana"
                    }
                    SelectOption::<String> {
                        index: 2usize,
                        value: "orange",
                        disabled: true,
                        "data-testid": "option-orange",
                        "Orange"
                    }
                    SelectOption::<String> {
                        index: 3usize,
                        value: "orangeade",
                        "data-testid": "option-orangeade",
                        "Orangeade"
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn click_trigger_opens_list() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();

    tester
        .query(by_testid("list"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_down_focuses_first_option() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::ArrowDown).await.unwrap();

    // The focused option has tabindex="0"; everything else stays at "-1".
    tester
        .query(by_testid("option-apple"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_down_twice_focuses_second_option() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::ArrowDown).await.unwrap();
    list.key_down(Key::ArrowDown).await.unwrap();

    tester
        .query(by_testid("option-banana"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_up_from_top_wraps_to_last_enabled() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::ArrowUp).await.unwrap();

    // The matching playwright test asserts "Orangeade" focuses, since "Orange" is disabled.
    tester
        .query(by_testid("option-orangeade"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_down_skips_disabled_options() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::ArrowDown).await.unwrap();
    list.key_down(Key::ArrowDown).await.unwrap();
    list.key_down(Key::ArrowDown).await.unwrap();

    // Apple → Banana → (skip disabled Orange) → Orangeade
    tester
        .query(by_testid("option-orangeade"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
    tester
        .query(by_testid("option-orange"))
        .expect(attr("aria-disabled", eq("true")))
        .await
        .unwrap();
}

#[tokio::test]
async fn enter_selects_focused_option_and_closes_list() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::ArrowDown).await.unwrap();
    list.key_down(Key::Enter).await.unwrap();

    // Listbox is removed from DOM once closed (use_animated_open's `render()` returns false).
    assert!(
        tester.query(by_testid("list")).immediately().is_err(),
        "list should be gone after Enter selects"
    );
}

#[tokio::test]
async fn escape_closes_list() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::Escape).await.unwrap();

    assert!(
        tester.query(by_testid("list")).immediately().is_err(),
        "list should be gone after Escape"
    );
}

#[tokio::test]
async fn typeahead_focuses_matching_option() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    // Type "Ban" — Banana should focus.
    list.key_down(Key::Character("B".into())).await.unwrap();
    list.key_down(Key::Character("a".into())).await.unwrap();
    list.key_down(Key::Character("n".into())).await.unwrap();

    tester
        .query(by_testid("option-banana"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

#[tokio::test]
async fn typeahead_skips_disabled_options() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    let mut list = tester.query(by_testid("list"));
    list.key_down(Key::Character("O".into())).await.unwrap();
    list.key_down(Key::Character("r".into())).await.unwrap();
    list.key_down(Key::Character("a".into())).await.unwrap();

    // "Ora" matches both Orange (disabled) and Orangeade; the typeahead should land on Orangeade.
    tester
        .query(by_testid("option-orangeade"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

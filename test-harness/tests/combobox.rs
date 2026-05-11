//! Port of `playwright/combobox.spec.ts` (interaction parts that don't depend on browser-driven
//! focus / paint / pointer-down semantics).

use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    Combobox, ComboboxEmpty, ComboboxInput, ComboboxList, ComboboxOption,
};
use dioxus_test::{Key, attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        let frameworks = vec![
            ("nextjs", "Next.js"),
            ("sveltekit", "SvelteKit"),
            ("nuxt", "Nuxt"),
            ("remix", "Remix"),
            ("solidstart", "SolidStart"),
        ];
        rsx! {
            Combobox::<String> {
                "data-testid": "root",
                ComboboxInput { "data-testid": "input", placeholder: "Select framework" }
                ComboboxList {
                    "data-testid": "list",
                    for (idx, (value, label)) in frameworks.iter().cloned().enumerate() {
                        ComboboxOption::<String> {
                            value: value.to_string(),
                            text_value: Some(label.to_string()),
                            index: idx,
                            "data-testid": "option-{value}",
                            "{label}"
                        }
                    }
                    ComboboxEmpty { "data-testid": "empty", "No framework found." }
                }
            }
        }
    }
}

#[tokio::test]
async fn list_is_closed_initially() {
    let mut tester = render(demo()).build();
    tester
        .query(by_testid("input"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_down_from_focused_input_opens_list() {
    let mut tester = render(demo()).build();

    let mut input = tester.query(by_testid("input"));
    input.focus().await.unwrap();
    input.key_down(Key::ArrowDown).await.unwrap();

    tester
        .query(by_testid("input"))
        .expect(attr("aria-expanded", eq("true")))
        .await
        .unwrap();
    tester
        .query(by_testid("input"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_input_opens_list() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("input")).click().await.unwrap();

    tester
        .query(by_testid("input"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();
}

#[tokio::test]
async fn typing_filters_options() {
    let mut tester = render(demo()).build();

    let mut input = tester.query(by_testid("input"));
    input.click().await.unwrap();
    input.input("sve").await.unwrap();

    // The SvelteKit option should still be visible.
    tester.query(by_testid("option-sveltekit")).await.unwrap();
    // The Next.js option should be filtered out.
    assert!(
        tester
            .query(by_testid("option-nextjs"))
            .immediately()
            .is_err()
    );
}

#[tokio::test]
async fn no_match_shows_empty_state() {
    let mut tester = render(demo()).build();

    let mut input = tester.query(by_testid("input"));
    input.click().await.unwrap();
    input.input("zzzz").await.unwrap();

    tester.query(by_testid("empty")).await.unwrap();
    // No options are rendered when nothing matches.
    assert!(
        tester
            .query(by_testid("option-sveltekit"))
            .immediately()
            .is_err()
    );
}

#[tokio::test]
async fn escape_closes_the_list() {
    let mut tester = render(demo()).build();

    let mut input = tester.query(by_testid("input"));
    input.click().await.unwrap();
    tester
        .query(by_testid("input"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();

    let mut input = tester.query(by_testid("input"));
    input.key_down(Key::Escape).await.unwrap();

    tester
        .query(by_testid("input"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
}

#[tokio::test]
async fn keyboard_selection_closes_list_and_sets_value() {
    // Option selection in the primitive runs on pointerup/pointerdown rather than click, and
    // the testing library doesn't expose pointer events. Drive selection via the keyboard
    // path on the input instead: ArrowDown to focus the first option, type to narrow to
    // SvelteKit, then Enter to commit.
    let mut tester = render(demo()).build();

    let mut input = tester.query(by_testid("input"));
    input.click().await.unwrap();
    input.input("sve").await.unwrap();
    input.key_down(Key::ArrowDown).await.unwrap();
    input.key_down(Key::Enter).await.unwrap();

    tester
        .query(by_testid("input"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
    tester
        .query(by_testid("input"))
        .expect(attr("value", eq("SvelteKit")))
        .await
        .unwrap();
}

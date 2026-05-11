//! Tests for `dioxus_primitives::radio_group::RadioGroup` — exercises real keyboard
//! navigation via the new `key_down` event surface in `dioxus-test`.

use dioxus::prelude::*;
use dioxus_primitives::radio_group::{RadioGroup, RadioItem};
use dioxus_test::{Key, attr, by_testid, eq, render};

fn demo_group(default: &'static str) -> impl Fn() -> Element + Clone + 'static {
    move || {
        rsx! {
            RadioGroup {
                default_value: default.to_string(),
                RadioItem {
                    value: "blue".to_string(),
                    index: 0usize,
                    "data-testid": "blue",
                    "Blue"
                }
                RadioItem {
                    value: "red".to_string(),
                    index: 1usize,
                    "data-testid": "red",
                    "Red"
                }
                RadioItem {
                    value: "green".to_string(),
                    index: 2usize,
                    "data-testid": "green",
                    "Green"
                }
            }
        }
    }
}

#[tokio::test]
async fn renders_default_selection() {
    let mut tester = render(demo_group("blue")).build();

    tester
        .query(by_testid("blue"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
    tester
        .query(by_testid("red"))
        .expect(attr("data-state", eq("unchecked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_selects_item() {
    let mut tester = render(demo_group("blue")).build();

    tester.query(by_testid("red")).click().await.unwrap();

    tester
        .query(by_testid("red"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
    tester
        .query(by_testid("blue"))
        .expect(attr("data-state", eq("unchecked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn focus_event_delivery_diagnostic() {
    // Sanity check: confirm that dispatching focus() actually invokes the radio's onfocus handler.
    // If it didn't, the arrow-key tests would fail because the focus state never updates.
    use dioxus_test::contains_string;
    #[component]
    fn FocusProbe() -> Element {
        let mut log = use_signal(String::new);
        rsx! {
            div {
                "data-testid": "log",
                "{log}"
            }
            button {
                "data-testid": "btn",
                onfocus: move |_| log.set("got-focus".into()),
                "Btn"
            }
        }
    }

    let mut tester = render(FocusProbe).build();
    tester.query(by_testid("btn")).focus().await.unwrap();

    tester
        .query(by_testid("log"))
        .expect(dioxus_test::text(contains_string("got-focus")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_down_moves_selection_to_next_item() {
    let mut tester = render(demo_group("blue")).build();

    // The roving-focus state needs an initial focus on the selected item before arrow keys move
    // it. Focus the first radio, then press ArrowDown to advance to the second.
    let mut blue = tester.query(by_testid("blue"));
    blue.focus().await.unwrap();
    blue.key_down(Key::ArrowDown).await.unwrap();

    tester
        .query(by_testid("red"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_down_twice_lands_on_third_item() {
    let mut tester = render(demo_group("blue")).build();

    tester.query(by_testid("blue")).focus().await.unwrap();
    tester
        .query(by_testid("blue"))
        .key_down(Key::ArrowDown)
        .await
        .unwrap();
    // Wait for the first arrow to settle before sending the next.
    tester
        .query(by_testid("red"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
    tester
        .query(by_testid("red"))
        .key_down(Key::ArrowDown)
        .await
        .unwrap();

    tester
        .query(by_testid("green"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
}


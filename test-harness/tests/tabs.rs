//! Tests for `dioxus_primitives::tabs::Tabs`.

use dioxus::prelude::*;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};
use dioxus_test::{Key, attr, by_testid, eq, render, text};

fn demo_tabs(default: &'static str) -> impl Fn() -> Element + Clone + 'static {
    move || {
        rsx! {
            Tabs {
                default_value: default.to_string(),
                horizontal: true,
                TabList {
                    TabTrigger {
                        value: "one".to_string(),
                        index: 0usize,
                        "data-testid": "trigger-one",
                        "One"
                    }
                    TabTrigger {
                        value: "two".to_string(),
                        index: 1usize,
                        "data-testid": "trigger-two",
                        "Two"
                    }
                }
                TabContent {
                    index: 0usize,
                    value: "one".to_string(),
                    "data-testid": "content-one",
                    "Panel one"
                }
                TabContent {
                    index: 1usize,
                    value: "two".to_string(),
                    "data-testid": "content-two",
                    "Panel two"
                }
            }
        }
    }
}

#[tokio::test]
async fn renders_default_active_tab() {
    let mut tester = render(demo_tabs("one")).build();

    tester
        .query(by_testid("trigger-one"))
        .expect(attr("data-state", eq("active")))
        .await
        .unwrap();
    tester
        .query(by_testid("trigger-two"))
        .expect(attr("data-state", eq("inactive")))
        .await
        .unwrap();
    tester
        .query(by_testid("content-one"))
        .expect(text(eq("Panel one")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_switches_active_tab() {
    let mut tester = render(demo_tabs("one")).build();

    tester
        .query(by_testid("trigger-two"))
        .click()
        .await
        .unwrap();

    tester
        .query(by_testid("trigger-two"))
        .expect(attr("data-state", eq("active")))
        .await
        .unwrap();
    tester
        .query(by_testid("trigger-one"))
        .expect(attr("data-state", eq("inactive")))
        .await
        .unwrap();
    tester
        .query(by_testid("content-two"))
        .expect(text(eq("Panel two")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_right_moves_roving_focus_to_next_tab() {
    let mut tester = render(demo_tabs("one")).build();

    let mut t1 = tester.query(by_testid("trigger-one"));
    t1.focus().await.unwrap();
    t1.key_down(Key::ArrowRight).await.unwrap();

    // Roving tabindex: the focused tab (and the selected one) both get tabindex="0";
    // ArrowRight here lights up trigger-two.
    tester
        .query(by_testid("trigger-two"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

#[tokio::test]
async fn aria_selected_reflects_active_tab() {
    let mut tester = render(demo_tabs("two")).build();

    tester
        .query(by_testid("trigger-two"))
        .expect(attr("aria-selected", eq("true")))
        .await
        .unwrap();
    tester
        .query(by_testid("trigger-one"))
        .expect(attr("aria-selected", eq("false")))
        .await
        .unwrap();
}

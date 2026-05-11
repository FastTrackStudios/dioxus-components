//! Tests for `dioxus_primitives::toggle::Toggle` driven by the `dioxus-test`
//! library from https://github.com/DioxusLabs/dioxus/pull/5323.

use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;
use dioxus_test::{attr, by_testid, eq, render};

fn demo_toggle() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Toggle {
                default_pressed: false,
                "data-testid": "toggle",
                "B"
            }
        }
    }
}

#[tokio::test]
async fn renders_in_off_state() {
    let mut tester = render(demo_toggle()).build();
    tester
        .query(by_testid("toggle"))
        .expect(attr("data-state", eq("off")))
        .await
        .unwrap();
    tester
        .query(by_testid("toggle"))
        .expect(attr("aria-pressed", eq("false")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_flips_pressed_state() {
    let mut tester = render(demo_toggle()).build();

    tester.query(by_testid("toggle")).click().await.unwrap();

    tester
        .query(by_testid("toggle"))
        .expect(attr("data-state", eq("on")))
        .await
        .unwrap();
    tester
        .query(by_testid("toggle"))
        .expect(attr("aria-pressed", eq("true")))
        .await
        .unwrap();
}

#[tokio::test]
async fn double_click_returns_to_off() {
    let mut tester = render(demo_toggle()).build();

    tester.query(by_testid("toggle")).click().await.unwrap();
    tester
        .query(by_testid("toggle"))
        .expect(attr("data-state", eq("on")))
        .await
        .unwrap();

    tester.query(by_testid("toggle")).click().await.unwrap();
    tester
        .query(by_testid("toggle"))
        .expect(attr("data-state", eq("off")))
        .await
        .unwrap();
}

//! Tests for `dioxus_primitives::switch::Switch` driven by the `dioxus-test`
//! library from https://github.com/DioxusLabs/dioxus/pull/5323.

use dioxus::prelude::*;
use dioxus_primitives::switch::{Switch, SwitchThumb};
use dioxus_test::{attr, by_testid, eq, render};

fn demo_switch() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Switch {
                default_checked: false,
                "data-testid": "switch",
                SwitchThumb {}
            }
        }
    }
}

#[tokio::test]
async fn renders_in_unchecked_state() {
    let mut tester = render(demo_switch()).build();
    tester
        .query(by_testid("switch"))
        .expect(attr("data-state", eq("unchecked")))
        .await
        .unwrap();
    tester
        .query(by_testid("switch"))
        .expect(attr("aria-checked", eq("false")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_toggles_to_checked() {
    let mut tester = render(demo_switch()).build();

    tester.query(by_testid("switch")).click().await.unwrap();

    tester
        .query(by_testid("switch"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();
    tester
        .query(by_testid("switch"))
        .expect(attr("aria-checked", eq("true")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_twice_returns_to_unchecked() {
    let mut tester = render(demo_switch()).build();

    tester.query(by_testid("switch")).click().await.unwrap();
    tester
        .query(by_testid("switch"))
        .expect(attr("data-state", eq("checked")))
        .await
        .unwrap();

    tester.query(by_testid("switch")).click().await.unwrap();
    tester
        .query(by_testid("switch"))
        .expect(attr("data-state", eq("unchecked")))
        .await
        .unwrap();
}

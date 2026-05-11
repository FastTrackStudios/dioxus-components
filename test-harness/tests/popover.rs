//! Port of `playwright/popover.spec.ts` (focus-trap Tab behavior is browser-driven).

use dioxus::prelude::*;
use dioxus_primitives::popover::{PopoverContent, PopoverRoot, PopoverTrigger};
use dioxus_test::{attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            PopoverRoot {
                "data-testid": "root",
                PopoverTrigger { "data-testid": "trigger", "Show Popover" }
                PopoverContent {
                    "data-testid": "content",
                    "Inside the popover"
                }
            }
        }
    }
}

#[tokio::test]
async fn popover_is_closed_initially() {
    let mut tester = render(demo()).build();
    tester
        .query(by_testid("root"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_trigger_opens_popover() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();

    tester
        .query(by_testid("root"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();
    tester.query(by_testid("content")).await.unwrap();
}

#[tokio::test]
async fn click_trigger_twice_closes_popover() {
    let mut tester = render(demo()).build();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.click().await.unwrap();
    tester
        .query(by_testid("root"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.click().await.unwrap();
    tester
        .query(by_testid("root"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
}

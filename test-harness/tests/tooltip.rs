//! Port of `playwright/tooltip.spec.ts`. Uses the new `hover`/`unhover` and `focus`/`blur`
//! helpers — the tooltip primitive opens on either pointer-enter or focus.

use dioxus::prelude::*;
use dioxus_primitives::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use dioxus_test::{by_testid, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Tooltip {
                TooltipTrigger { "data-testid": "trigger", "Rich content" }
                TooltipContent { "data-testid": "content", "Tooltip body" }
            }
        }
    }
}

#[tokio::test]
async fn tooltip_hidden_initially() {
    let mut tester = render(demo()).build();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

#[tokio::test]
async fn hover_shows_tooltip() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).hover().await.unwrap();

    tester.query(by_testid("content")).await.unwrap();
}

#[tokio::test]
async fn unhover_hides_tooltip() {
    let mut tester = render(demo()).build();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.hover().await.unwrap();
    tester.query(by_testid("content")).await.unwrap();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.unhover().await.unwrap();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

#[tokio::test]
async fn focus_shows_tooltip() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).focus().await.unwrap();

    tester.query(by_testid("content")).await.unwrap();
}

#[tokio::test]
async fn blur_hides_tooltip() {
    let mut tester = render(demo()).build();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.focus().await.unwrap();
    tester.query(by_testid("content")).await.unwrap();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.blur().await.unwrap();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

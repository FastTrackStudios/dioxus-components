//! Port of `playwright/hover-card.spec.ts`.

use dioxus::prelude::*;
use dioxus_primitives::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};
use dioxus_test::{by_testid, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            HoverCard {
                HoverCardTrigger { "data-testid": "trigger", "Dioxus" }
                HoverCardContent { "data-testid": "content", "Hover card body" }
            }
        }
    }
}

#[tokio::test]
async fn hover_card_hidden_initially() {
    let mut tester = render(demo()).build();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

#[tokio::test]
async fn focus_shows_hover_card() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).focus().await.unwrap();

    tester.query(by_testid("content")).await.unwrap();
}

#[tokio::test]
async fn blur_hides_hover_card() {
    let mut tester = render(demo()).build();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.focus().await.unwrap();
    tester.query(by_testid("content")).await.unwrap();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.blur().await.unwrap();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

#[tokio::test]
async fn hover_shows_hover_card() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).hover().await.unwrap();

    tester.query(by_testid("content")).await.unwrap();
}

#[tokio::test]
async fn unhover_hides_hover_card() {
    let mut tester = render(demo()).build();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.hover().await.unwrap();
    tester.query(by_testid("content")).await.unwrap();

    let mut trigger = tester.query(by_testid("trigger"));
    trigger.unhover().await.unwrap();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

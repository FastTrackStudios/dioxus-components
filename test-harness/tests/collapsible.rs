//! Tests for `dioxus_primitives::collapsible::Collapsible`.

use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use dioxus_test::{attr, by_testid, eq, render, text};

fn demo_collapsible(default_open: bool) -> impl Fn() -> Element + Clone + 'static {
    move || {
        rsx! {
            Collapsible {
                default_open,
                CollapsibleTrigger { "data-testid": "trigger", "Show details" }
                CollapsibleContent {
                    "data-testid": "content",
                    "Hidden content goes here"
                }
            }
        }
    }
}

#[tokio::test]
async fn closed_by_default_renders_no_content() {
    let mut tester = render(demo_collapsible(false)).build();

    tester
        .query(by_testid("trigger"))
        .expect(attr("aria-expanded", eq("false")))
        .await
        .unwrap();
    tester
        .query(by_testid("content"))
        .expect(text(eq("")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_trigger_reveals_content() {
    let mut tester = render(demo_collapsible(false)).build();

    tester.query(by_testid("trigger")).click().await.unwrap();

    tester
        .query(by_testid("content"))
        .expect(text(eq("Hidden content goes here")))
        .await
        .unwrap();
    tester
        .query(by_testid("trigger"))
        .expect(attr("aria-expanded", eq("true")))
        .await
        .unwrap();
}

#[tokio::test]
async fn click_trigger_twice_hides_content_again() {
    let mut tester = render(demo_collapsible(false)).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester
        .query(by_testid("content"))
        .expect(text(eq("Hidden content goes here")))
        .await
        .unwrap();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester
        .query(by_testid("content"))
        .expect(text(eq("")))
        .await
        .unwrap();
}

#[tokio::test]
async fn opens_with_default_open_true() {
    let mut tester = render(demo_collapsible(true)).build();

    tester
        .query(by_testid("trigger"))
        .expect(attr("aria-expanded", eq("true")))
        .await
        .unwrap();
    tester
        .query(by_testid("content"))
        .expect(text(eq("Hidden content goes here")))
        .await
        .unwrap();
}

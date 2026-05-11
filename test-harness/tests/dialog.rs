//! Port of `playwright/dialog.spec.ts` (modulo browser-driven Tab/Escape focus behavior
//! that the headless renderer can't simulate).

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use dioxus_test::{attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        let mut open = use_signal(|| false);
        rsx! {
            button {
                "data-testid": "trigger",
                onclick: move |_| open.set(true),
                "Show Dialog"
            }
            DialogRoot {
                "data-testid": "root",
                open: open(),
                on_open_change: move |v| open.set(v),
                DialogContent {
                    "data-testid": "content",
                    button {
                        "data-testid": "close",
                        onclick: move |_| open.set(false),
                        "Close"
                    }
                    "Hello"
                }
            }
        }
    }
}

#[tokio::test]
async fn dialog_is_closed_initially() {
    let mut tester = render(demo()).build();
    // The root element only renders when `render()` is true. When closed, content is absent.
    let resolved = tester.query(by_testid("content"));
    assert!(
        resolved.immediately().is_err(),
        "content should be absent when dialog is closed"
    );
}

#[tokio::test]
async fn click_trigger_opens_dialog() {
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
async fn click_close_button_closes_dialog() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester
        .query(by_testid("root"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();

    tester.query(by_testid("close")).click().await.unwrap();

    let resolved = tester.query(by_testid("content"));
    assert!(
        resolved.immediately().is_err(),
        "content should be gone after close"
    );
}

#[tokio::test]
async fn reopen_after_closing() {
    let mut tester = render(demo()).build();

    for _ in 0..2 {
        tester.query(by_testid("trigger")).click().await.unwrap();
        tester
            .query(by_testid("root"))
            .expect(attr("data-state", eq("open")))
            .await
            .unwrap();
        tester.query(by_testid("close")).click().await.unwrap();
    }
}

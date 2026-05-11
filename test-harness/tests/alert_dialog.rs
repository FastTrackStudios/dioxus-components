//! Port of `playwright/alert-dialog.spec.ts` (Tab focus-trap and Escape-close are browser-driven
//! and aren't reproduced in the headless renderer).

use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent, AlertDialogRoot,
};
use dioxus_test::{attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        let mut open = use_signal(|| false);
        rsx! {
            button {
                "data-testid": "trigger",
                onclick: move |_| open.set(true),
                "Show Alert Dialog"
            }
            AlertDialogRoot {
                "data-testid": "root",
                open: open(),
                on_open_change: move |v| open.set(v),
                AlertDialogContent {
                    "data-testid": "content",
                    "Are you sure?"
                    AlertDialogActions {
                        AlertDialogCancel { "data-testid": "cancel", "Cancel" }
                        AlertDialogAction { "data-testid": "delete", "Delete" }
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn dialog_is_closed_initially() {
    let mut tester = render(demo()).build();
    assert!(
        tester.query(by_testid("content")).immediately().is_err(),
        "content should be absent when closed"
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
async fn cancel_button_closes_dialog() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester.query(by_testid("content")).await.unwrap();

    tester.query(by_testid("cancel")).click().await.unwrap();
    assert!(
        tester.query(by_testid("content")).immediately().is_err(),
        "content should disappear after cancel"
    );
}

#[tokio::test]
async fn confirm_button_closes_dialog() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester.query(by_testid("content")).await.unwrap();

    tester.query(by_testid("delete")).click().await.unwrap();
    assert!(
        tester.query(by_testid("content")).immediately().is_err(),
        "content should disappear after confirm"
    );
}

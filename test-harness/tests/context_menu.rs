//! Port of `playwright/context-menu.spec.ts`. Uses the new `context_click` helper to fire
//! `oncontextmenu` on the trigger.

use dioxus::prelude::*;
use dioxus_primitives::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use dioxus_test::{Key, attr, by_testid, eq, render, text};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        let mut selected = use_signal(String::new);
        rsx! {
            div {
                "data-testid": "selected",
                "{selected}"
            }
            ContextMenu {
                ContextMenuTrigger {
                    "data-testid": "trigger",
                    "right click here"
                }
                ContextMenuContent {
                    "data-testid": "content",
                    ContextMenuItem {
                        index: 0usize,
                        value: "Edit".to_string(),
                        "data-testid": "edit",
                        on_select: move |v: String| selected.set(format!("Selected: {v}")),
                        "Edit"
                    }
                    ContextMenuItem {
                        index: 1usize,
                        value: "Undo".to_string(),
                        disabled: true,
                        "data-testid": "undo",
                        on_select: move |v: String| selected.set(format!("Selected: {v}")),
                        "Undo"
                    }
                    ContextMenuItem {
                        index: 2usize,
                        value: "Duplicate".to_string(),
                        "data-testid": "duplicate",
                        on_select: move |v: String| selected.set(format!("Selected: {v}")),
                        "Duplicate"
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn right_click_opens_the_menu() {
    let mut tester = render(demo()).build();

    tester
        .query(by_testid("trigger"))
        .context_click()
        .await
        .unwrap();

    tester
        .query(by_testid("content"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();
}

#[tokio::test]
async fn enter_on_item_selects_and_closes_menu() {
    // ContextMenuItem selection runs on `onpointerdown`, not `onclick`; the keyboard path
    // through `onkeydown` is what we can drive here.
    let mut tester = render(demo()).build();

    tester
        .query(by_testid("trigger"))
        .context_click()
        .await
        .unwrap();
    tester
        .query(by_testid("edit"))
        .key_down(Key::Enter)
        .await
        .unwrap();

    tester
        .query(by_testid("selected"))
        .expect(text(eq("Selected: Edit")))
        .await
        .unwrap();
}

#[tokio::test]
async fn escape_closes_the_menu() {
    let mut tester = render(demo()).build();

    tester
        .query(by_testid("trigger"))
        .context_click()
        .await
        .unwrap();
    tester
        .query(by_testid("content"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();

    tester
        .query(by_testid("content"))
        .key_down(Key::Escape)
        .await
        .unwrap();

    assert!(
        tester.query(by_testid("content")).immediately().is_err(),
        "menu should close on Escape"
    );
}

#[tokio::test]
async fn disabled_item_is_marked() {
    let mut tester = render(demo()).build();

    tester
        .query(by_testid("trigger"))
        .context_click()
        .await
        .unwrap();
    tester
        .query(by_testid("undo"))
        .expect(attr("data-disabled", eq("true")))
        .await
        .unwrap();
}

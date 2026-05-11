//! Port of `playwright/dropdown-menu.spec.ts`. The Tab/Escape close paths in the original
//! spec rely on browser-driven Tab and document-level Escape — those aren't reproduced here.

use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use dioxus_test::{Key, attr, by_testid, contains_string, eq, render, text};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        let mut selected = use_signal(String::new);
        rsx! {
            div {
                "data-testid": "selected",
                "{selected}"
            }
            DropdownMenu {
                "data-testid": "root",
                DropdownMenuTrigger { "data-testid": "trigger", "Open Menu" }
                DropdownMenuContent {
                    "data-testid": "content",
                    DropdownMenuItem::<String> {
                        value: "edit".to_string(),
                        index: 0usize,
                        "data-testid": "edit",
                        on_select: move |v: String| selected.set(format!("Selected: {v}")),
                        "Edit"
                    }
                    DropdownMenuItem::<String> {
                        value: "undo".to_string(),
                        index: 1usize,
                        disabled: true,
                        "data-testid": "undo",
                        on_select: move |v: String| selected.set(format!("Selected: {v}")),
                        "Undo"
                    }
                    DropdownMenuItem::<String> {
                        value: "duplicate".to_string(),
                        index: 2usize,
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
async fn menu_is_closed_initially() {
    let mut tester = render(demo()).build();
    tester
        .query(by_testid("trigger"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
    assert!(tester.query(by_testid("content")).immediately().is_err());
}

#[tokio::test]
async fn click_trigger_opens_menu() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();

    tester
        .query(by_testid("trigger"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();
    tester.query(by_testid("content")).await.unwrap();
}

#[tokio::test]
async fn click_item_selects_and_closes() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester.query(by_testid("duplicate")).click().await.unwrap();

    tester
        .query(by_testid("selected"))
        .expect(text(eq("Selected: duplicate")))
        .await
        .unwrap();
    tester
        .query(by_testid("trigger"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
}

#[tokio::test]
async fn disabled_item_is_marked() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester
        .query(by_testid("undo"))
        .expect(attr("data-disabled", eq("true")))
        .await
        .unwrap();
}

#[tokio::test]
async fn escape_closes_the_menu() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester
        .query(by_testid("trigger"))
        .expect(attr("data-state", eq("open")))
        .await
        .unwrap();

    tester
        .query(by_testid("root"))
        .key_down(Key::Escape)
        .await
        .unwrap();

    tester
        .query(by_testid("trigger"))
        .expect(attr("data-state", eq("closed")))
        .await
        .unwrap();
}

#[tokio::test]
async fn enter_on_item_selects_it() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger")).click().await.unwrap();
    tester
        .query(by_testid("edit"))
        .key_down(Key::Enter)
        .await
        .unwrap();

    tester
        .query(by_testid("selected"))
        .expect(text(contains_string("Selected: edit")))
        .await
        .unwrap();
}

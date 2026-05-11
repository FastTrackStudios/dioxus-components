//! Port of `playwright/toolbar.spec.ts`. Uses the new `tab()` helper to walk into the toolbar,
//! ArrowRight to traverse buttons, and `focused()` to assert which one currently holds focus.

use dioxus::prelude::*;
use dioxus_primitives::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
use dioxus_test::{Key, by_testid, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Toolbar { aria_label: "Text formatting",
                ToolbarButton { index: 0usize, "data-testid": "bold", "Bold" }
                ToolbarButton { index: 1usize, "data-testid": "italic", "Italic" }
                ToolbarButton { index: 2usize, "data-testid": "underline", "Underline" }
                ToolbarSeparator {}
                ToolbarButton { index: 3usize, "data-testid": "align-left", "Align Left" }
                ToolbarButton { index: 4usize, "data-testid": "align-center", "Align Center" }
                ToolbarButton { index: 5usize, "data-testid": "align-right", "Align Right" }
            }
        }
    }
}

fn focused_testid(tester: &dioxus_test::DocumentTester) -> Option<String> {
    tester.focused()?.attr("data-testid")
}

#[tokio::test]
async fn tab_focuses_first_toolbar_button() {
    let mut tester = render(demo()).build();
    tester.tab().await.unwrap();

    assert_eq!(focused_testid(&tester).as_deref(), Some("bold"));
}

#[tokio::test]
async fn arrow_right_walks_through_buttons_in_order() {
    let mut tester = render(demo()).build();
    tester.tab().await.unwrap();

    let order = ["bold", "italic", "underline", "align-left", "align-center", "align-right"];
    for window in order.windows(2) {
        let (current, next) = (window[0], window[1]);
        assert_eq!(focused_testid(&tester).as_deref(), Some(current));
        tester
            .query(by_testid(current))
            .key_down(Key::ArrowRight)
            .await
            .unwrap();
        assert_eq!(focused_testid(&tester).as_deref(), Some(next));
    }
}

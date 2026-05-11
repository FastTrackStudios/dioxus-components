//! Port of `playwright/toast.spec.ts`. Toast notifications are dispatched from a button and
//! closed via the close button inside each toast.

use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
use dioxus_test::{by_testid, render};
use std::time::Duration;

#[component]
fn ToastButton() -> Element {
    let toast_api = use_toast();
    rsx! {
        button {
            "data-testid": "create",
            onclick: move |_| {
                toast_api.info(
                    "Custom Toast".to_string(),
                    ToastOptions::new()
                        .description("Some info you need")
                        .duration(Duration::from_secs(60))
                        .permanent(false),
                );
            },
            "Info (60s)"
        }
    }
}

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            ToastProvider {
                ToastButton {}
            }
        }
    }
}

#[tokio::test]
async fn create_and_close_two_toasts() {
    let mut tester = render(demo()).build();

    // No toast close buttons initially.
    assert!(
        tester.query_all("button[aria-label='close']").immediately().is_empty(),
        "no toasts should exist at startup"
    );

    // Dispatch two toasts.
    tester.query(by_testid("create")).click().await.unwrap();
    tester.query(by_testid("create")).click().await.unwrap();

    // Both close buttons should be present.
    tester
        .query_all("button[aria-label='close']")
        .expect(min_count(2))
        .await
        .unwrap();

    // Close the first toast.
    tester
        .query("button[aria-label='close']")
        .click()
        .await
        .unwrap();
    tester
        .query_all("button[aria-label='close']")
        .expect(exact_count(1))
        .await
        .unwrap();

    // Close the last toast.
    tester
        .query("button[aria-label='close']")
        .click()
        .await
        .unwrap();
    tester
        .query_all("button[aria-label='close']")
        .expect(exact_count(0))
        .await
        .unwrap();
}

// --- helpers ---

fn min_count(min: usize) -> impl for<'a> dioxus_test::Matcher<Vec<dioxus_test::ResolvedElement<'a>>>
{
    struct M(usize);
    impl<'a> dioxus_test::Matcher<Vec<dioxus_test::ResolvedElement<'a>>> for M {
        fn matches(&self, els: Vec<dioxus_test::ResolvedElement<'a>>) -> std::ops::ControlFlow<()> {
            if els.len() >= self.0 {
                std::ops::ControlFlow::Break(())
            } else {
                std::ops::ControlFlow::Continue(())
            }
        }
        fn describe(&self) -> String {
            format!("at least {} matching elements", self.0)
        }
    }
    M(min)
}

fn exact_count(
    n: usize,
) -> impl for<'a> dioxus_test::Matcher<Vec<dioxus_test::ResolvedElement<'a>>> {
    struct M(usize);
    impl<'a> dioxus_test::Matcher<Vec<dioxus_test::ResolvedElement<'a>>> for M {
        fn matches(&self, els: Vec<dioxus_test::ResolvedElement<'a>>) -> std::ops::ControlFlow<()> {
            if els.len() == self.0 {
                std::ops::ControlFlow::Break(())
            } else {
                std::ops::ControlFlow::Continue(())
            }
        }
        fn describe(&self) -> String {
            format!("exactly {} matching elements", self.0)
        }
    }
    M(n)
}

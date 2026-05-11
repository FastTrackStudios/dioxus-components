//! Tests for `dioxus_primitives::progress::Progress`.

use dioxus::prelude::*;
use dioxus_primitives::progress::{Progress, ProgressIndicator};
use dioxus_test::{attr, by_testid, eq, render};

fn demo_progress(value: f64, max: f64) -> impl Fn() -> Element + Clone + 'static {
    move || {
        rsx! {
            Progress {
                "data-testid": "pb",
                value,
                max,
                ProgressIndicator {}
            }
        }
    }
}

#[tokio::test]
async fn reports_value_and_max_via_aria() {
    let mut tester = render(demo_progress(42.0, 100.0)).build();

    tester
        .query(by_testid("pb"))
        .expect(attr("aria-valuenow", eq("42")))
        .await
        .unwrap();
    tester
        .query(by_testid("pb"))
        .expect(attr("aria-valuemax", eq("100")))
        .await
        .unwrap();
    tester
        .query(by_testid("pb"))
        .expect(attr("data-state", eq("loading")))
        .await
        .unwrap();
}

#[tokio::test]
async fn custom_max_value() {
    let mut tester = render(demo_progress(7.0, 10.0)).build();

    tester
        .query(by_testid("pb"))
        .expect(attr("aria-valuenow", eq("7")))
        .await
        .unwrap();
    tester
        .query(by_testid("pb"))
        .expect(attr("aria-valuemax", eq("10")))
        .await
        .unwrap();
    tester
        .query(by_testid("pb"))
        .expect(attr("data-value", eq("7")))
        .await
        .unwrap();
}

#[tokio::test]
async fn indeterminate_progress_has_no_value() {
    let mut tester = render(|| {
        rsx! {
            Progress {
                "data-testid": "pb",
                ProgressIndicator {}
            }
        }
    })
    .build();

    tester
        .query(by_testid("pb"))
        .expect(attr("data-state", eq("indeterminate")))
        .await
        .unwrap();
    // aria-valuenow is omitted entirely when value is None — confirm it's absent.
    let pb = tester.query(by_testid("pb")).await.unwrap();
    assert_eq!(pb.attr("aria-valuenow"), None);
}

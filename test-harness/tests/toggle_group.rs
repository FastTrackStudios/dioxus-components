//! Port of `playwright/toggle_group.spec.ts`.

use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};
use dioxus_test::{Key, attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            ToggleGroup { horizontal: true, allow_multiple_pressed: true,
                ToggleItem { index: 0usize, "data-testid": "b", em { "B" } }
                ToggleItem { index: 1usize, "data-testid": "i", i { "I" } }
                ToggleItem { index: 2usize, "data-testid": "u", u { "U" } }
            }
        }
    }
}

#[tokio::test]
async fn items_start_unpressed() {
    let mut tester = render(demo()).build();
    for id in ["b", "i", "u"] {
        tester
            .query(by_testid(id))
            .expect(attr("data-state", eq("off")))
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn click_toggles_item_on() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("b")).click().await.unwrap();

    tester
        .query(by_testid("b"))
        .expect(attr("data-state", eq("on")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_right_updates_tabindex_for_next_item() {
    // Pure focus-state assertion: roving tabindex moves from `b` to `i` on ArrowRight.
    let mut tester = render(demo()).build();

    let mut b = tester.query(by_testid("b"));
    b.focus().await.unwrap();
    b.key_down(Key::ArrowRight).await.unwrap();

    tester
        .query(by_testid("i"))
        .expect(attr("tabindex", eq("0")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_right_moves_roving_focus() {
    // The playwright equivalent presses ArrowRight then Enter — Enter activates because the
    // browser auto-translates Enter on a focused button into a click. The headless renderer
    // doesn't do that translation, so we dispatch the click directly after the arrow moves
    // roving focus to the next item.
    let mut tester = render(demo()).build();

    let mut b = tester.query(by_testid("b"));
    b.click().await.unwrap();
    b.key_down(Key::ArrowRight).await.unwrap();

    tester.query(by_testid("i")).click().await.unwrap();

    tester
        .query(by_testid("i"))
        .expect(attr("data-state", eq("on")))
        .await
        .unwrap();
    // B remains on because allow_multiple_pressed is true.
    tester
        .query(by_testid("b"))
        .expect(attr("data-state", eq("on")))
        .await
        .unwrap();
}

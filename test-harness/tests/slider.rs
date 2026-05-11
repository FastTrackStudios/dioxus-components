//! Port of `playwright/slider.spec.ts` (keyboard navigation portion). Drag/pointer interactions
//! require pointer events that the testing library doesn't yet expose, so they're omitted.

use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
use dioxus_test::{Key, Modifiers, attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Slider {
                label: "Demo Slider",
                horizontal: true,
                default_value: 50.0,
                SliderTrack {
                    SliderRange {}
                    SliderThumb { "data-testid": "thumb" }
                }
            }
        }
    }
}

#[tokio::test]
async fn initial_value_is_50() {
    let mut tester = render(demo()).build();
    tester
        .query(by_testid("thumb"))
        .expect(attr("aria-valuenow", eq("50")))
        .await
        .unwrap();
}

#[tokio::test]
async fn shift_arrow_right_jumps_by_ten() {
    let mut tester = render(demo()).build();

    let mut thumb = tester.query(by_testid("thumb"));
    thumb.focus().await.unwrap();
    thumb
        .key_down_with_modifiers(Key::ArrowRight, Modifiers::SHIFT)
        .await
        .unwrap();

    tester
        .query(by_testid("thumb"))
        .expect(attr("aria-valuenow", eq("60")))
        .await
        .unwrap();

    let mut thumb = tester.query(by_testid("thumb"));
    thumb
        .key_down_with_modifiers(Key::ArrowRight, Modifiers::SHIFT)
        .await
        .unwrap();
    tester
        .query(by_testid("thumb"))
        .expect(attr("aria-valuenow", eq("70")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_left_decreases_by_one() {
    let mut tester = render(demo()).build();

    let mut thumb = tester.query(by_testid("thumb"));
    thumb.focus().await.unwrap();
    thumb.key_down(Key::ArrowLeft).await.unwrap();

    tester
        .query(by_testid("thumb"))
        .expect(attr("aria-valuenow", eq("49")))
        .await
        .unwrap();
}

#[tokio::test]
async fn arrow_right_increases_by_one() {
    let mut tester = render(demo()).build();

    let mut thumb = tester.query(by_testid("thumb"));
    thumb.focus().await.unwrap();
    thumb.key_down(Key::ArrowRight).await.unwrap();

    tester
        .query(by_testid("thumb"))
        .expect(attr("aria-valuenow", eq("51")))
        .await
        .unwrap();
}

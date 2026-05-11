//! Port of `playwright/accordion.spec.ts`.

use dioxus::prelude::*;
use dioxus_primitives::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use dioxus_test::{attr, by_testid, eq, render};

fn demo() -> impl Fn() -> Element + Clone + 'static {
    || {
        rsx! {
            Accordion {
                allow_multiple_open: false,
                horizontal: false,
                AccordionItem { index: 0usize, "data-testid": "item-0",
                    AccordionTrigger { "data-testid": "trigger-0", "Item 0" }
                    AccordionContent { "Content 0" }
                }
                AccordionItem { index: 1usize, "data-testid": "item-1",
                    AccordionTrigger { "data-testid": "trigger-1", "Item 1" }
                    AccordionContent { "Content 1" }
                }
                AccordionItem { index: 2usize, "data-testid": "item-2",
                    AccordionTrigger { "data-testid": "trigger-2", "Item 2" }
                    AccordionContent { "Content 2" }
                }
            }
        }
    }
}

#[tokio::test]
async fn click_opens_an_item() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger-0")).click().await.unwrap();

    tester
        .query(by_testid("item-0"))
        .expect(attr("data-open", eq("true")))
        .await
        .unwrap();
}

#[tokio::test]
async fn opening_a_second_item_closes_the_first() {
    let mut tester = render(demo()).build();

    tester.query(by_testid("trigger-0")).click().await.unwrap();
    tester
        .query(by_testid("item-0"))
        .expect(attr("data-open", eq("true")))
        .await
        .unwrap();

    tester.query(by_testid("trigger-1")).click().await.unwrap();
    tester
        .query(by_testid("item-1"))
        .expect(attr("data-open", eq("true")))
        .await
        .unwrap();
    tester
        .query(by_testid("item-0"))
        .expect(attr("data-open", eq("false")))
        .await
        .unwrap();
}

#[tokio::test]
async fn items_start_closed() {
    let mut tester = render(demo()).build();

    for id in ["item-0", "item-1", "item-2"] {
        tester
            .query(by_testid(id))
            .expect(attr("data-open", eq("false")))
            .await
            .unwrap();
    }
}

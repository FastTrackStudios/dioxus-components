use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use dioxus_icons::lucide::{BadgeCheck, ChevronRight};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.75rem",
            width: "100%",
            max_width: "22rem",

            Item { variant: ItemVariant::Outline,
                ItemContent {
                    ItemTitle { "Basic item" }
                    ItemDescription { "Title and description." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, "Action" }
                }
            }

            Item {
                variant: ItemVariant::Outline,
                size: ItemSize::Sm,
                as: move |attrs: Vec<Attribute>| rsx! {
                    a { href: "#", ..attrs,
                        ItemMedia { variant: ItemMediaVariant::Icon,
                            BadgeCheckIcon {}
                        }
                        ItemContent {
                            ItemTitle { "Profile verified" }
                        }
                        ItemActions {
                            ChevronRightIcon {}
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn BadgeCheckIcon() -> Element {
    rsx! {
        BadgeCheck { size: "20" }
    }
}

#[component]
fn ChevronRightIcon() -> Element {
    rsx! {
        ChevronRight { size: "16" }
    }
}

use dioxus::prelude::*;

use super::super::component::*;

#[css_module("/src/components/badge/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            class: Styles::dx_badge_example,
            display: "flex",
            flex_wrap: "wrap",
            justify_content: "center",
            gap: "0.5rem",
            max_width: "16rem",
            Badge { "Primary" }
            Badge { variant: BadgeVariant::Secondary, "Secondary" }
            Badge { variant: BadgeVariant::Destructive, "Destructive" }
            Badge { variant: BadgeVariant::Outline, "Outline" }
        }
    }
}

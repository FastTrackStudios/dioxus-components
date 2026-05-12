use dioxus::prelude::*;
use dioxus_primitives::progress::{self};

#[css_module("/src/components/progress/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value, between 0 and max.
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to 100.
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,

    /// Additional attributes to apply to the progress element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        progress::Progress {
            class: Styles::dx_progress,
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            progress::ProgressIndicator { class: Styles::dx_progress_indicator }
        }
    }
}

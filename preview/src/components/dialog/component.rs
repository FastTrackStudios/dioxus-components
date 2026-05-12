use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogDescriptionProps, DialogTitleProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/dialog/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// The id of the dialog root element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the dialog.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the dialog if uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes applied to the dialog content (the panel inside the backdrop).
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children rendered inside the dialog content.
    pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let content_base = attributes!(div {
        class: Styles::dx_dialog,
    });
    let content_merged = merge_attributes(vec![content_base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            class: Styles::dx_dialog_backdrop,
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: content_merged,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_dialog_title,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_dialog_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

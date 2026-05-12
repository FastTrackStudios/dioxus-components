use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronsUpDown};
use dioxus_primitives::combobox::{self, ComboboxEmptyProps, ComboboxOptionProps, ComboboxProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/combobox/style.css")]
struct Styles;

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Combobox {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: merged,
            div { class: Styles::dx_combobox_input_wrapper,
                combobox::ComboboxInput {
                    class: Styles::dx_combobox_input,
                }
                ChevronsUpDown {
                    class: Styles::dx_combobox_expand_icon,
                    size: "16px",
                }
            }
            combobox::ComboboxList {
                class: Styles::dx_combobox_list,
                {props.children}
            }
        }
    }
}

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox_empty });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxEmpty {
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox_option });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            combobox::ComboboxItemIndicator {
                Check {
                    class: Styles::dx_combobox_check_icon,
                    size: "16px",
                }
            }
        }
    }
}

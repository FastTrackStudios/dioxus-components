use std::time::Duration;

use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps,
    SelectTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/select/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the select.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The default uncontrolled value of the select.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the select is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Time without input before typeahead matching resets.
    #[props(default = Duration::from_millis(500))]
    pub typeahead_timeout: Duration,

    /// Placeholder shown in the trigger when no value is selected.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Optional id for the trigger element.
    #[props(default)]
    pub trigger_id: Option<String>,

    /// Optional id for the popup list element.
    #[props(default)]
    pub list_id: ReadSignal<Option<String>>,

    /// Accessible label for the trigger button.
    #[props(default)]
    pub trigger_aria_label: Option<String>,

    /// Accessible label for the popup list.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional attributes applied to the select root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The grouped options (and optionally [`SelectGroupLabel`]s) rendered in the popup.
    pub children: Element,
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_select });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                id: props.trigger_id,
                aria_label: props.trigger_aria_label,
                select::SelectValue { placeholder: props.placeholder }
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                id: props.list_id,
                aria_label: props.aria_label,
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectMultiProps<T: Clone + PartialEq + 'static = String> {
    pub values: ReadSignal<Option<Vec<T>>>,

    #[props(default)]
    pub default_values: Vec<T>,

    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub name: ReadSignal<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Duration::from_millis(500))]
    pub typeahead_timeout: Duration,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub trigger_id: Option<String>,

    #[props(default)]
    pub list_id: ReadSignal<Option<String>>,

    #[props(default)]
    pub trigger_aria_label: Option<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_select });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                id: props.trigger_id,
                aria_label: props.trigger_aria_label,
                select::SelectValue { placeholder: props.placeholder }
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                id: props.list_id,
                aria_label: props.aria_label,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let base = attributes!(button { class: Styles::dx_select_trigger });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectTrigger { attributes: merged,
            {props.children}
            ChevronDown {
                class: "dx-select-expand-icon",
                size: "20px",
                stroke: "var(--primary-color-7)",
            }
        }
    }
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let base = attributes!(div { class: Styles::dx_select_list });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectList {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div { class: Styles::dx_select_group_label });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_select_option });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check {
                    size: "1rem",
                    stroke: "var(--secondary-color-5)",
                }
            }
        }
    }
}

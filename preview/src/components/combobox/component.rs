use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronsUpDown};
use dioxus_primitives::combobox::{self, ComboboxEmptyProps, ComboboxOptionProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/combobox/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The default uncontrolled value.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the combobox is disabled.
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

    /// The controlled text query used to filter options.
    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    /// The initial text query when uncontrolled.
    #[props(default)]
    pub default_query: ReadSignal<String>,

    /// Callback fired when the text query changes.
    #[props(default)]
    pub on_query_change: Callback<String>,

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Custom filter callback. Receives `(query, option_text_value)`.
    #[props(default = Callback::new(|(q, t): (String, String)| dioxus_primitives::combobox::default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    /// Placeholder text for the input.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Optional id for the input element.
    #[props(default)]
    pub input_id: ReadSignal<Option<String>>,

    /// Optional id for the popup list element.
    #[props(default)]
    pub list_id: ReadSignal<Option<String>>,

    /// Accessible label for the popup list.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional attributes for the combobox root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Options (and an optional [`ComboboxEmpty`]) rendered inside the popup.
    pub children: Element,
}

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let base = attributes!(div { class: Styles::dx_combobox });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Combobox::<T> {
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
                    placeholder: props.placeholder,
                    id: props.input_id,
                }
                ChevronsUpDown {
                    class: Styles::dx_combobox_expand_icon,
                    size: "16px",
                }
            }
            combobox::ComboboxList {
                class: Styles::dx_combobox_list,
                id: props.list_id,
                aria_label: props.aria_label,
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

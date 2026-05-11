use dioxus::prelude::*;
use dioxus_primitives::otp::{
    self, OneTimePasswordGroupProps, OneTimePasswordInputProps, OneTimePasswordSeparatorProps,
    OneTimePasswordSlotProps,
};

#[css_module("/src/components/otp/style.css")]
struct Styles;

#[component]
pub fn OneTimePasswordInput(props: OneTimePasswordInputProps) -> Element {
    rsx! {
        otp::OneTimePasswordInput {
            class: Styles::dx_otp,
            value: props.value,
            default_value: props.default_value,
            maxlength: props.maxlength,
            inputmode: props.inputmode,
            autocomplete: props.autocomplete,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            id: props.id,
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            validate: props.validate,
            on_value_change: props.on_value_change,
            on_complete: props.on_complete,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordGroup(props: OneTimePasswordGroupProps) -> Element {
    rsx! {
        otp::OneTimePasswordGroup {
            class: Styles::dx_otp_group,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    rsx! {
        otp::OneTimePasswordSlot {
            class: Styles::dx_otp_slot,
            index: props.index,
            attributes: props.attributes,
            span { class: Styles::dx_otp_caret, aria_hidden: "true" }
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordSeparator(props: OneTimePasswordSeparatorProps) -> Element {
    rsx! {
        otp::OneTimePasswordSeparator {
            class: Styles::dx_otp_separator,
            attributes: props.attributes,
            svg {
                width: "10",
                height: "10",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                line { x1: "5", y1: "12", x2: "19", y2: "12" }
            }
            {props.children}
        }
    }
}

use super::super::context::OtpCtx;
use dioxus::prelude::*;

/// The props for the [`OneTimePasswordSlot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordSlotProps {
    /// The position of this slot in the value (zero-based).
    pub index: ReadSignal<usize>,

    /// Additional attributes applied to the slot element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional children rendered after the character (for example, a custom caret element).
    /// The current character is exposed via the `data-char` attribute.
    pub children: Element,
}

/// # OneTimePasswordSlot
///
/// A single slot within a [`super::OneTimePasswordInput`]. Renders the character at `index`
/// from the shared value. Must be used inside a [`super::OneTimePasswordInput`].
///
/// ## Styling
///
/// The slot element exposes:
/// - `data-active`: `true` when this slot is the next one to receive input.
/// - `data-empty`: `true` when no character has been entered at this position.
/// - `data-disabled`: mirrors the parent's disabled state.
/// - `data-char`: the current character at this position (empty when none).
#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    let ctx: OtpCtx = use_context();
    let index = props.index;

    let char_at = use_memo(move || {
        ctx.value
            .read()
            .chars()
            .nth(index())
            .map(|c| c.to_string())
            .unwrap_or_default()
    });
    let is_active = use_memo(move || ctx.active_index.cloned() == Some(index()));
    let is_empty = use_memo(move || char_at.read().is_empty());

    rsx! {
        div {
            role: "presentation",
            aria_hidden: "true",
            "data-active": is_active,
            "data-empty": is_empty,
            "data-disabled": ctx.disabled,
            "data-char": char_at,
            ..props.attributes,

            {char_at}
            {props.children}
        }
    }
}

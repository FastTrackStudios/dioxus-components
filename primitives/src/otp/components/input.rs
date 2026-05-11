use super::super::context::OtpCtx;
use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;

/// The props for the [`OneTimePasswordInput`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordInputProps {
    /// The controlled value of the OTP input.
    pub value: ReadSignal<Option<String>>,

    /// The default value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// The maximum number of characters the input accepts (the total number of slots).
    pub maxlength: ReadSignal<usize>,

    /// Hint for the on-screen keyboard. Defaults to `"numeric"`.
    #[props(default = ReadSignal::new(Signal::new(String::from("numeric"))))]
    pub inputmode: ReadSignal<String>,

    /// Autocomplete hint applied to the underlying input. Defaults to `"one-time-code"`.
    #[props(default = ReadSignal::new(Signal::new(String::from("one-time-code"))))]
    pub autocomplete: ReadSignal<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the input is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,

    /// The name attribute used for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Optional id for the inner `<input>`. When omitted, a stable id is generated.
    /// Use this to associate an external `<label for=...>`.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Accessible name for the input. Forwarded as `aria-label` on the underlying `<input>`.
    #[props(default)]
    pub aria_label: ReadSignal<Option<String>>,

    /// ID of an element labelling the input. Forwarded as `aria-labelledby` on the underlying `<input>`.
    #[props(default)]
    pub aria_labelledby: ReadSignal<Option<String>>,

    /// Optional validator. Called with the prospective new value when inserting characters
    /// (keystrokes and paste); return `false` to reject the change. `Backspace` and `Delete`
    /// bypass the validator and always shrink the value.
    #[props(default)]
    pub validate: Option<Callback<String, bool>>,

    /// Callback fired whenever the value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Callback fired when the value reaches `maxlength`.
    #[props(default)]
    pub on_complete: Callback<String>,

    /// Additional attributes applied to the wrapper element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the input — typically [`super::OneTimePasswordGroup`],
    /// [`super::OneTimePasswordSlot`], and [`super::OneTimePasswordSeparator`] components.
    pub children: Element,
}

/// # OneTimePasswordInput
///
/// The `OneTimePasswordInput` is the root of an OTP entry. It renders a single, accessible
/// `<input>` element overlaid on top of its children so paste, autofill (`autocomplete="one-time-code"`),
/// IME composition, and screen readers continue to work, while child [`super::OneTimePasswordSlot`]s
/// render the visual representation of each character.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::otp::{
///     OneTimePasswordInput, OneTimePasswordGroup, OneTimePasswordSlot, OneTimePasswordSeparator,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         OneTimePasswordInput { maxlength: 6usize,
///             OneTimePasswordGroup {
///                 OneTimePasswordSlot { index: 0usize }
///                 OneTimePasswordSlot { index: 1usize }
///                 OneTimePasswordSlot { index: 2usize }
///             }
///             OneTimePasswordSeparator {}
///             OneTimePasswordGroup {
///                 OneTimePasswordSlot { index: 3usize }
///                 OneTimePasswordSlot { index: 4usize }
///                 OneTimePasswordSlot { index: 5usize }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The wrapper sets the following data attributes:
/// - `data-disabled`: `true` or `false` depending on the `disabled` prop.
#[component]
pub fn OneTimePasswordInput(props: OneTimePasswordInputProps) -> Element {
    let maxlength = props.maxlength;
    let on_complete = props.on_complete;
    let validate = props.validate;

    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let generated_id = use_unique_id();
    let input_id = use_id_or(generated_id, props.id);
    let mut is_focused = use_signal(|| false);
    let mut cursor = use_signal(|| 0usize);

    let active_index = use_memo(move || {
        if !is_focused() {
            return None;
        }
        let max = maxlength();
        if max == 0 {
            return None;
        }
        let c = cursor();
        if c >= max {
            return None;
        }
        let len = value.read().chars().count();
        Some(c.min(len))
    });

    use_context_provider(|| OtpCtx {
        value,
        disabled: props.disabled,
        active_index,
    });

    rsx! {
        div {
            role: "group",
            position: "relative",
            "data-disabled": props.disabled,
            ..props.attributes,

            {props.children}

            input {
                id: input_id,
                r#type: "text",
                inputmode: props.inputmode,
                autocomplete: props.autocomplete,
                name: props.name,
                disabled: props.disabled,
                required: props.required,
                aria_label: props.aria_label,
                aria_labelledby: props.aria_labelledby,
                maxlength: maxlength() as i64,
                value,

                style: "position:absolute;top:0;left:0;right:0;bottom:0;width:100%;height:100%;opacity:1;color:transparent;background:transparent;caret-color:transparent;outline:none;border:none;padding:0;margin:0;text-align:center;font-family:inherit;font-size:inherit;cursor:text;user-select:none;",

                onkeydown: move |e: Event<KeyboardData>| {
                    if (props.disabled)() {
                        return;
                    }
                    let key = e.key();
                    let max = maxlength();
                    if max == 0 {
                        return;
                    }
                    let mods = e.modifiers();
                    let mut chars: Vec<char> = value.read().chars().collect();
                    let old_len = chars.len();
                    let mut new_cursor = cursor();
                    let mut value_changed = false;

                    match key {
                        Key::ArrowLeft => {
                            new_cursor = new_cursor.saturating_sub(1);
                            e.prevent_default();
                        }
                        Key::ArrowRight => {
                            if new_cursor < chars.len() {
                                new_cursor += 1;
                            }
                            e.prevent_default();
                        }
                        Key::Home => {
                            new_cursor = 0;
                            e.prevent_default();
                        }
                        Key::End => {
                            new_cursor = chars.len();
                            e.prevent_default();
                        }
                        Key::Backspace => {
                            e.prevent_default();
                            if mods.ctrl() || mods.meta() {
                                if !chars.is_empty() {
                                    chars.clear();
                                    new_cursor = 0;
                                    value_changed = true;
                                }
                            } else {
                                let effective = new_cursor.min(chars.len());
                                if effective > 0 {
                                    chars.remove(effective - 1);
                                    new_cursor = effective - 1;
                                    value_changed = true;
                                }
                            }
                        }
                        Key::Delete => {
                            e.prevent_default();
                            if new_cursor < chars.len() {
                                chars.remove(new_cursor);
                                value_changed = true;
                            }
                        }
                        Key::Character(ref s)
                            if s.chars().count() == 1
                                && !mods.ctrl()
                                && !mods.meta()
                                && !mods.alt() =>
                        {
                            e.prevent_default();
                            let insert_at = new_cursor.min(chars.len());
                            if insert_at < max {
                                if let Some(c) = s.chars().next() {
                                    let mut next_chars = chars.clone();
                                    next_chars.insert(insert_at, c);
                                    next_chars.truncate(max);
                                    let next_value: String =
                                        next_chars.iter().copied().collect();
                                    if let Some(validate) = validate {
                                        if !validate.call(next_value.clone()) {
                                            return;
                                        }
                                    }
                                    chars = next_chars;
                                    new_cursor = (insert_at + 1).min(max);
                                    value_changed = true;
                                }
                            }
                        }
                        _ => {}
                    }

                    if value_changed {
                        let new_value: String = chars.into_iter().collect();
                        let new_len = new_value.chars().count();
                        set_value.call(new_value.clone());
                        if old_len < max && new_len == max {
                            on_complete.call(new_value);
                        }
                    }
                    if new_cursor != cursor() {
                        cursor.set(new_cursor);
                    }
                },

                oninput: move |e| {
                    let raw = e.value();
                    let max = maxlength();
                    let filtered: String = raw.chars().take(max).collect();
                    if let Some(validate) = validate {
                        if !validate.call(filtered.clone()) {
                            return;
                        }
                    }
                    let len = filtered.chars().count();
                    let old_len = value.read().chars().count();
                    if filtered != *value.read() {
                        set_value.call(filtered.clone());
                        if max > 0 && old_len < max && len == max {
                            on_complete.call(filtered);
                        }
                    }
                    cursor.set(len);
                },

                onfocus: move |_| {
                    is_focused.set(true);
                    cursor.set(value.read().chars().count());
                },
                onblur: move |_| is_focused.set(false),
            }
        }
    }
}

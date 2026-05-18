use super::super::context::{OtpCtx, SelectionRange, SlotBounds};
use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;
use std::rc::Rc;

fn input_value_and_cursor(
    current_value: &str,
    raw_value: &str,
    max: usize,
    cursor: usize,
    selected_range: Option<SelectionRange>,
) -> (String, usize, bool) {
    if max == 0 {
        return (String::new(), 0, false);
    }

    let current_chars: Vec<char> = current_value.chars().collect();
    let next_chars: Vec<char> = raw_value.chars().take(max).collect();
    let current_len = current_chars.len();
    let next_len = next_chars.len();
    let cursor = cursor.min(current_len);
    let (start, end) = selected_range
        .map(|range| (range.start.min(current_len), range.end.min(current_len)))
        .unwrap_or((cursor, cursor));
    let replaced_len = end.saturating_sub(start);
    let base_len = current_len.saturating_sub(replaced_len);
    let next_cursor = if next_len >= base_len {
        start + (next_len - base_len)
    } else {
        current_chars
            .iter()
            .zip(&next_chars)
            .take_while(|(a, b)| a == b)
            .count()
    }
    .min(next_len);
    let next: String = next_chars.into_iter().collect();
    let changed = next != current_value;

    (next, next_cursor, changed)
}

fn delete_backward(
    current_value: &str,
    cursor: usize,
    selected_range: Option<SelectionRange>,
) -> (String, usize) {
    let mut chars: Vec<char> = current_value.chars().collect();

    if let Some(range) = selected_range {
        chars.drain(range.start..range.end);
        return (chars.into_iter().collect(), range.start);
    }

    let cursor = cursor.min(chars.len());
    if cursor == 0 {
        return (current_value.to_string(), 0);
    }

    chars.remove(cursor - 1);
    (chars.into_iter().collect(), cursor - 1)
}

fn delete_forward(
    current_value: &str,
    cursor: usize,
    selected_range: Option<SelectionRange>,
) -> (String, usize) {
    let mut chars: Vec<char> = current_value.chars().collect();

    if let Some(range) = selected_range {
        chars.drain(range.start..range.end);
        return (chars.into_iter().collect(), range.start);
    }

    let cursor = cursor.min(chars.len());
    if cursor >= chars.len() {
        return (current_value.to_string(), cursor);
    }

    chars.remove(cursor);
    (chars.into_iter().collect(), cursor)
}

fn active_slot_index(cursor: usize, len: usize, max: usize) -> Option<usize> {
    if max == 0 {
        return None;
    }

    Some(cursor.min(len).min(max - 1))
}

fn cursor_from_slot_bounds(client_x: f64, len: usize, slot_bounds: &[Option<SlotBounds>]) -> usize {
    for index in 0..len {
        let Some(bounds) = slot_bounds.get(index).copied().flatten() else {
            continue;
        };

        if client_x < bounds.center() {
            return index;
        }
    }

    len
}

fn cursor_from_input_bounds(client_x: f64, len: usize, input_bounds: Option<SlotBounds>) -> usize {
    let Some(bounds) = input_bounds else {
        return len;
    };
    let width = bounds.right - bounds.left;
    if width <= 0.0 || len == 0 {
        return 0;
    }

    let offset = (client_x - bounds.left).clamp(0.0, width);
    ((offset / width * len as f64) + 0.5).floor() as usize
}

fn cursor_from_bounds(
    client_x: f64,
    len: usize,
    slot_bounds: &[Option<SlotBounds>],
    input_bounds: Option<SlotBounds>,
) -> usize {
    if slot_bounds.iter().take(len).any(Option::is_some) {
        cursor_from_slot_bounds(client_x, len, slot_bounds)
    } else {
        cursor_from_input_bounds(client_x, len, input_bounds).min(len)
    }
}

fn selection_focus_from_slot_bounds(
    client_x: f64,
    anchor: usize,
    len: usize,
    slot_bounds: &[Option<SlotBounds>],
) -> usize {
    for index in 0..len {
        let Some(bounds) = slot_bounds.get(index).copied().flatten() else {
            continue;
        };

        if bounds.left <= client_x && client_x <= bounds.right {
            return if index < anchor {
                index
            } else {
                (index + 1).min(len)
            };
        }
    }

    cursor_from_slot_bounds(client_x, len, slot_bounds)
}

fn selection_focus_from_bounds(
    client_x: f64,
    anchor: usize,
    len: usize,
    slot_bounds: &[Option<SlotBounds>],
    input_bounds: Option<SlotBounds>,
) -> usize {
    if slot_bounds.iter().take(len).any(Option::is_some) {
        selection_focus_from_slot_bounds(client_x, anchor, len, slot_bounds)
    } else {
        cursor_from_input_bounds(client_x, len, input_bounds).min(len)
    }
}

fn apply_value_change(
    current_value: &str,
    next_value: String,
    max: usize,
    set_value: Callback<String>,
    on_complete: Callback<String>,
) {
    let old_len = current_value.chars().count();
    let len = next_value.chars().count();

    if next_value != current_value {
        set_value.call(next_value.clone());
        if old_len < max && len == max {
            on_complete.call(next_value);
        }
    }
}

fn focus_input(input_id: String) {
    spawn(async move {
        let eval = document::eval(
            r#"
            const id = await dioxus.recv();
            const input = document.getElementById(id);

            if (input && input.focus) {
                input.focus({ preventScroll: true });
            }
            "#,
        );
        let _ = eval.send(input_id);
    });
}

async fn mounted_bounds(mounted: Rc<MountedData>) -> Option<SlotBounds> {
    let Ok(rect) = mounted.get_client_rect().await else {
        return None;
    };

    Some(SlotBounds {
        left: rect.origin.x,
        right: rect.origin.x + rect.size.width,
    })
}

async fn sync_input_bounds(mut input_bounds: Signal<Option<SlotBounds>>, mounted: Rc<MountedData>) {
    if let Some(bounds) = mounted_bounds(mounted).await {
        input_bounds.set(Some(bounds));
    }
}

async fn refresh_input_bounds(
    mut input_bounds: Signal<Option<SlotBounds>>,
    input_ref: Signal<Option<Rc<MountedData>>>,
) -> Option<SlotBounds> {
    let Some(mounted) = input_ref() else {
        return input_bounds();
    };

    let Some(bounds) = mounted_bounds(mounted).await else {
        return input_bounds();
    };

    input_bounds.set(Some(bounds));
    Some(bounds)
}

async fn refresh_slot_bounds(
    mut slot_bounds: Signal<Vec<Option<SlotBounds>>>,
    slot_refs: Signal<Vec<Option<Rc<MountedData>>>>,
    len: usize,
) -> Vec<Option<SlotBounds>> {
    let refs = {
        let refs = slot_refs.read();
        (0..len)
            .map(|index| refs.get(index).cloned().flatten())
            .collect::<Vec<_>>()
    };
    let previous = slot_bounds.read().clone();
    let mut next = previous.clone();
    if next.len() < len {
        next.resize(len, None);
    }

    for (index, mounted) in refs.into_iter().enumerate() {
        let Some(mounted) = mounted else {
            continue;
        };
        if let Some(bounds) = mounted_bounds(mounted).await {
            next[index] = Some(bounds);
        }
    }

    if next != previous {
        slot_bounds.set(next.clone());
    }

    next
}

async fn sync_input_selection(input_id: String, start: usize, end: usize) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const start = await dioxus.recv();
        const end = await dioxus.recv();
        const input = document.getElementById(id);

        if (input && document.activeElement === input && input.setSelectionRange) {
            input.setSelectionRange(start, end);
        }
        "#,
    );
    let _ = eval.send(input_id);
    let _ = eval.send(start);
    let _ = eval.send(end);
}

async fn sync_input_value_and_selection(input_id: String, value: String, start: usize, end: usize) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const value = await dioxus.recv();
        const start = await dioxus.recv();
        const end = await dioxus.recv();
        const input = document.getElementById(id);

        if (input) {
            if (input.value !== value) {
                input.value = value;
            }
            if (document.activeElement === input && input.setSelectionRange) {
                input.setSelectionRange(start, end);
            }
        }
        "#,
    );
    let _ = eval.send(input_id);
    let _ = eval.send(value);
    let _ = eval.send(start);
    let _ = eval.send(end);
}

#[cfg(test)]
mod tests {
    use super::{active_slot_index, delete_backward, delete_forward, input_value_and_cursor};
    use crate::otp::context::SelectionRange;

    #[test]
    fn input_change_uses_raw_value() {
        assert_eq!(
            input_value_and_cursor("12", "129", 6, 2, None),
            ("129".to_string(), 3, true)
        );
    }

    #[test]
    fn input_change_uses_visible_cursor_for_middle_insert() {
        assert_eq!(
            input_value_and_cursor("12", "192", 6, 1, None),
            ("192".to_string(), 2, true)
        );
    }

    #[test]
    fn consecutive_input_change_keeps_cursor_after_inserted_text() {
        assert_eq!(
            input_value_and_cursor("192", "1982", 6, 2, None),
            ("1982".to_string(), 3, true)
        );
    }

    #[test]
    fn unchanged_input_is_not_reported_as_changed() {
        assert_eq!(
            input_value_and_cursor("192", "192", 6, 3, None),
            ("192".to_string(), 3, false)
        );
    }

    #[test]
    fn input_change_truncates_to_maxlength() {
        assert_eq!(
            input_value_and_cursor("123456", "1234569", 6, 6, None),
            ("123456".to_string(), 6, false)
        );
    }

    #[test]
    fn input_deletion_uses_raw_value() {
        assert_eq!(
            input_value_and_cursor("12", "1", 6, 2, None),
            ("1".to_string(), 1, true)
        );
    }

    #[test]
    fn full_input_replacement_uses_raw_value() {
        assert_eq!(
            input_value_and_cursor("12", "987654", 6, 0, SelectionRange::new(0, 2, 2)),
            ("987654".to_string(), 6, true)
        );
    }

    #[test]
    fn full_input_replacement_with_shared_prefix_uses_raw_value() {
        assert_eq!(
            input_value_and_cursor("123456", "123999", 6, 0, SelectionRange::new(0, 6, 6)),
            ("123999".to_string(), 6, true)
        );
    }

    #[test]
    fn shorter_full_input_replacement_with_shared_prefix_uses_raw_value() {
        assert_eq!(
            input_value_and_cursor("123456", "1239", 6, 0, SelectionRange::new(0, 6, 6)),
            ("1239".to_string(), 4, true)
        );
    }

    #[test]
    fn full_input_replacement_truncates_to_maxlength() {
        assert_eq!(
            input_value_and_cursor("123456", "9876543", 6, 0, SelectionRange::new(0, 6, 6)),
            ("987654".to_string(), 6, true)
        );
    }

    #[test]
    fn selected_range_replacement_with_shared_digits_uses_raw_value() {
        assert_eq!(
            input_value_and_cursor("123456", "123996", 6, 3, SelectionRange::new(3, 5, 6)),
            ("123996".to_string(), 5, true)
        );
    }

    #[test]
    fn delete_backward_removes_selected_range() {
        let selected_range = SelectionRange::new(1, 4, 6);

        assert_eq!(
            delete_backward("123456", 4, selected_range),
            ("156".to_string(), 1)
        );
    }

    #[test]
    fn delete_forward_removes_after_cursor() {
        assert_eq!(delete_forward("123456", 2, None), ("12456".to_string(), 2));
    }

    #[test]
    fn active_slot_stays_visible_at_end() {
        assert_eq!(active_slot_index(6, 6, 6), Some(5));
    }

    #[test]
    fn active_slot_tracks_next_empty_slot() {
        assert_eq!(active_slot_index(2, 2, 6), Some(2));
    }

    #[test]
    fn backward_pointer_selection_includes_slot_under_pointer() {
        use super::selection_focus_from_slot_bounds;
        use crate::otp::context::SlotBounds;

        let bounds = (0..6)
            .map(|index| {
                Some(SlotBounds {
                    left: index as f64 * 10.0,
                    right: index as f64 * 10.0 + 10.0,
                })
            })
            .collect::<Vec<_>>();

        assert_eq!(selection_focus_from_slot_bounds(15.0, 5, 6, &bounds), 1);
    }

    #[test]
    fn pointer_selection_falls_back_to_input_bounds() {
        use super::selection_focus_from_bounds;
        use crate::otp::context::SlotBounds;

        let input_bounds = Some(SlotBounds {
            left: 0.0,
            right: 60.0,
        });

        assert_eq!(
            selection_focus_from_bounds(15.0, 5, 6, &[], input_bounds),
            2
        );
    }
}

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
    let mut selection_anchor = use_signal(|| None::<usize>);
    let mut selection_range = use_signal(|| None::<SelectionRange>);
    let mut selecting_with_pointer = use_signal(|| false);
    let mut pointer_focus_x = use_signal(|| None::<f64>);
    let slot_bounds = use_signal(Vec::<Option<SlotBounds>>::new);
    let slot_refs = use_signal(Vec::<Option<Rc<MountedData>>>::new);
    let input_bounds = use_signal(|| None::<SlotBounds>);
    let mut input_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut pointer_focus_pending = use_hook(|| CopyValue::new(false));

    let native_selection = use_memo(move || {
        let len = value.read().chars().count();
        if let Some(range) =
            selection_range().and_then(|range| SelectionRange::new(range.start, range.end, len))
        {
            (range.start, range.end)
        } else {
            let c = cursor().min(len);
            (c, c)
        }
    });

    let selected_range = use_memo(move || {
        if !is_focused() {
            return None;
        }

        let len = value.read().chars().count();
        selection_range().and_then(|range| SelectionRange::new(range.start, range.end, len))
    });

    let active_index = use_memo(move || {
        if !is_focused() {
            return None;
        }
        if selected_range().is_some() {
            return None;
        }
        let len = value.read().chars().count();
        active_slot_index(cursor(), len, maxlength())
    });

    use_context_provider(|| OtpCtx {
        value,
        disabled: props.disabled,
        active_index,
        selected_range,
        slot_bounds,
        slot_refs,
    });

    use_effect(move || {
        if !is_focused() {
            return;
        }

        let id = input_id();
        let (start, end) = native_selection();

        spawn(async move {
            sync_input_selection(id, start, end).await;
        });
    });

    use_effect(move || {
        let Some(focus_x) = pointer_focus_x() else {
            return;
        };
        let Some(anchor) = selection_anchor() else {
            return;
        };

        let len = value.read().chars().count();
        let next_cursor =
            selection_focus_from_bounds(focus_x, anchor, len, &slot_bounds.read(), input_bounds());
        let next_selection = SelectionRange::new(anchor, next_cursor, len);

        if cursor() != next_cursor {
            cursor.set(next_cursor);
        }
        if selection_range() != next_selection {
            selection_range.set(next_selection);
        }
        if !selecting_with_pointer() && next_selection.is_some() {
            pointer_focus_x.set(None);
        }
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
                maxlength,
                name: props.name,
                disabled: props.disabled,
                required: props.required,
                aria_label: props.aria_label,
                aria_labelledby: props.aria_labelledby,
                value,

                style: "position:absolute;z-index:20;top:0;left:0;right:0;bottom:0;width:100%;height:100%;opacity:0;color:transparent;background:transparent;caret-color:transparent;outline:none;border:none;padding:0;margin:0;text-align:center;font-family:inherit;font-size:inherit;cursor:text;user-select:none;",

                onmounted: move |event: Event<MountedData>| {
                    let mounted = event.data();
                    input_ref.set(Some(mounted.clone()));
                    async move {
                        sync_input_bounds(input_bounds, mounted).await;
                    }
                },

                onresize: move |_| async move {
                    let Some(mounted) = input_ref() else {
                        return;
                    };
                    sync_input_bounds(input_bounds, mounted).await;
                },

                onpointerdown: move |e: Event<PointerData>| async move {
                    if (props.disabled)() {
                        return;
                    }

                    e.prevent_default();
                    let focus_x = e.client_coordinates().x;
                    is_focused.set(true);

                    let len = value.read().chars().count();
                    let next_cursor = cursor_from_bounds(
                        focus_x,
                        len,
                        &slot_bounds.read(),
                        input_bounds(),
                    );

                    cursor.set(next_cursor);
                    selection_anchor.set(Some(next_cursor));
                    selection_range.set(None);
                    pointer_focus_x.set(None);
                    selecting_with_pointer.set(true);
                    pointer_focus_pending.set(true);
                    focus_input(input_id());

                    let fresh_input_bounds =
                        refresh_input_bounds(input_bounds, input_ref).await;
                    let fresh_slot_bounds =
                        refresh_slot_bounds(slot_bounds, slot_refs, len).await;
                    let next_anchor = cursor_from_bounds(
                        focus_x,
                        len,
                        &fresh_slot_bounds,
                        fresh_input_bounds,
                    );

                    selection_anchor.set(Some(next_anchor));
                    if let Some(current_focus_x) = pointer_focus_x() {
                        let next_cursor = selection_focus_from_bounds(
                            current_focus_x,
                            next_anchor,
                            len,
                            &fresh_slot_bounds,
                            fresh_input_bounds,
                        );

                        cursor.set(next_cursor);
                        selection_range.set(SelectionRange::new(next_anchor, next_cursor, len));
                    } else {
                        cursor.set(next_anchor);
                    }
                },

                onpointermove: move |e: Event<PointerData>| {
                    if !selecting_with_pointer() || (props.disabled)() {
                        return;
                    }

                    e.prevent_default();
                    let focus_x = e.client_coordinates().x;
                    let len = value.read().chars().count();
                    let fallback_cursor = cursor_from_bounds(
                        focus_x,
                        len,
                        &slot_bounds.read(),
                        input_bounds(),
                    );
                    let anchor = selection_anchor().unwrap_or(fallback_cursor);
                    let next_cursor = selection_focus_from_bounds(
                        focus_x,
                        anchor,
                        len,
                        &slot_bounds.read(),
                        input_bounds(),
                    );

                    pointer_focus_x.set(Some(focus_x));
                    cursor.set(next_cursor);
                    selection_range.set(SelectionRange::new(anchor, next_cursor, len));
                },

                onpointerup: move |e: Event<PointerData>| {
                    if selecting_with_pointer() {
                        e.prevent_default();
                    }
                    selecting_with_pointer.set(false);
                },

                onpointercancel: move |_| {
                    selecting_with_pointer.set(false);
                    pointer_focus_x.set(None);
                    pointer_focus_pending.set(false);
                },

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
                    let chars: Vec<char> = value.read().chars().collect();
                    let len = chars.len();
                    let selected = selection_range();
                    let mut new_cursor = cursor().min(len);
                    let mut next_selection = selected;
                    let mut next_anchor = selection_anchor();

                    match key {
                        Key::ArrowLeft => {
                            e.prevent_default();
                            if mods.shift() {
                                let anchor = next_anchor.unwrap_or(new_cursor);
                                new_cursor = new_cursor.saturating_sub(1);
                                next_anchor = Some(anchor);
                                next_selection = SelectionRange::new(anchor, new_cursor, len);
                            } else {
                                new_cursor = selected
                                    .map(|range| range.start)
                                    .unwrap_or_else(|| new_cursor.saturating_sub(1));
                                next_anchor = None;
                                next_selection = None;
                            }
                        }
                        Key::ArrowRight => {
                            e.prevent_default();
                            if mods.shift() {
                                let anchor = next_anchor.unwrap_or(new_cursor);
                                new_cursor = (new_cursor + 1).min(len);
                                next_anchor = Some(anchor);
                                next_selection = SelectionRange::new(anchor, new_cursor, len);
                            } else {
                                new_cursor = selected
                                    .map(|range| range.end)
                                    .unwrap_or_else(|| (new_cursor + 1).min(len));
                                next_anchor = None;
                                next_selection = None;
                            }
                        }
                        Key::Home => {
                            e.prevent_default();
                            if mods.shift() {
                                let anchor = next_anchor.unwrap_or(new_cursor);
                                new_cursor = 0;
                                next_anchor = Some(anchor);
                                next_selection = SelectionRange::new(anchor, new_cursor, len);
                            } else {
                                new_cursor = 0;
                                next_anchor = None;
                                next_selection = None;
                            }
                        }
                        Key::End => {
                            e.prevent_default();
                            if mods.shift() {
                                let anchor = next_anchor.unwrap_or(new_cursor);
                                new_cursor = len;
                                next_anchor = Some(anchor);
                                next_selection = SelectionRange::new(anchor, new_cursor, len);
                            } else {
                                new_cursor = len;
                                next_anchor = None;
                                next_selection = None;
                            }
                        }
                        Key::Backspace => {
                            e.prevent_default();
                            if mods.ctrl() || mods.meta() {
                                if !chars.is_empty() {
                                    new_cursor = 0;
                                    set_value.call(String::new());
                                }
                            } else {
                                let current_value = value.read().clone();
                                let (next_value, next_cursor) =
                                    delete_backward(&current_value, new_cursor, selected);
                                apply_value_change(
                                    &current_value,
                                    next_value,
                                    max,
                                    set_value,
                                    on_complete,
                                );
                                new_cursor = next_cursor;
                            }
                            next_anchor = None;
                            next_selection = None;
                        }
                        Key::Delete => {
                            e.prevent_default();
                            let current_value = value.read().clone();
                            let (next_value, next_cursor) =
                                delete_forward(&current_value, new_cursor, selected);
                            apply_value_change(
                                &current_value,
                                next_value,
                                max,
                                set_value,
                                on_complete,
                            );
                            new_cursor = next_cursor;
                            next_anchor = None;
                            next_selection = None;
                        }
                        Key::Character(ref s)
                            if (mods.ctrl() || mods.meta()) && s.eq_ignore_ascii_case("a") =>
                        {
                            e.prevent_default();
                            new_cursor = 0;
                            next_anchor = Some(0);
                            next_selection = SelectionRange::new(0, len, len);
                        }
                        Key::Character(ref s)
                            if s.chars().count() == 1
                                && !mods.ctrl()
                                && !mods.meta()
                                && !mods.alt() =>
                        {
                            e.prevent_default();
                            let (start, end) = selected
                                .map(|range| (range.start, range.end))
                                .unwrap_or((new_cursor, new_cursor));
                            if let Some(c) = s.chars().next() {
                                let mut next_chars = chars.clone();
                                next_chars.splice(start..end, [c]);
                                next_chars.truncate(max);
                                let next_value: String =
                                    next_chars.iter().copied().collect();
                                if let Some(validate) = validate {
                                    if !validate.call(next_value.clone()) {
                                        return;
                                    }
                                }
                                let current_value: String = chars.iter().copied().collect();
                                apply_value_change(
                                    &current_value,
                                    next_value,
                                    max,
                                    set_value,
                                    on_complete,
                                );
                                new_cursor = (start + 1).min(next_chars.len());
                                next_anchor = None;
                                next_selection = None;
                            }
                        }
                        _ => {}
                    }

                    if cursor() != new_cursor {
                        cursor.set(new_cursor);
                    }
                    if selection_anchor() != next_anchor {
                        selection_anchor.set(next_anchor);
                    }
                    if selection_range() != next_selection {
                        selection_range.set(next_selection);
                    }
                    pointer_focus_x.set(None);
                },

                oninput: move |e: FormEvent| {
                    let raw = e.value();
                    let max = maxlength();
                    let current_value = value.read().clone();
                    let selected = selection_range();
                    let current_cursor = cursor().min(current_value.chars().count());
                    let (next_value, next_cursor, inserted) =
                        input_value_and_cursor(&current_value, &raw, max, current_cursor, selected);
                    if inserted {
                        if let Some(validate) = validate {
                            if !validate.call(next_value.clone()) {
                                let id = input_id();
                                let (start, end) = selected
                                    .map(|range| (range.start, range.end))
                                    .unwrap_or((current_cursor, current_cursor));
                                spawn(async move {
                                    sync_input_value_and_selection(
                                        id,
                                        current_value,
                                        start,
                                        end,
                                    )
                                    .await;
                                });
                                return;
                            }
                        }
                    }
                    if raw != next_value {
                        let id = input_id();
                        let value_to_sync = next_value.clone();
                        spawn(async move {
                            sync_input_value_and_selection(
                                id,
                                value_to_sync,
                                next_cursor,
                                next_cursor,
                            )
                            .await;
                        });
                    } else {
                        let id = input_id();
                        spawn(async move {
                            sync_input_selection(id, next_cursor, next_cursor).await;
                        });
                    }
                    apply_value_change(
                        &current_value,
                        next_value,
                        max,
                        set_value,
                        on_complete,
                    );
                    cursor.set(next_cursor);
                    selection_anchor.set(None);
                    selection_range.set(None);
                    pointer_focus_x.set(None);
                },

                onfocus: move |_| {
                    is_focused.set(true);
                    if pointer_focus_pending() {
                        pointer_focus_pending.set(false);
                        return;
                    }
                    if !selecting_with_pointer()
                        && selection_anchor().is_none()
                        && selection_range().is_none()
                        && pointer_focus_x().is_none()
                    {
                        cursor.set(value.read().chars().count());
                        selection_anchor.set(None);
                        selection_range.set(None);
                        pointer_focus_x.set(None);
                    }
                },
                onblur: move |_| {
                    is_focused.set(false);
                    selecting_with_pointer.set(false);
                    selection_anchor.set(None);
                    selection_range.set(None);
                    pointer_focus_x.set(None);
                    pointer_focus_pending.set(false);
                },
            }
        }
    }
}

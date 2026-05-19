use super::super::context::OtpCtx;
use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;
use std::ops::Range;

fn selection_range_for(anchor: usize, focus: usize, len: usize) -> Option<Range<usize>> {
    let start = anchor.min(focus).min(len);
    let end = anchor.max(focus).min(len);

    (start < end).then_some(start..end)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InputValueChange {
    value: String,
    cursor: usize,
    changed: bool,
}

impl InputValueChange {
    fn from_raw_input(
        current_value: &str,
        raw_value: &str,
        max: usize,
        cursor: usize,
        selected_range: Option<Range<usize>>,
    ) -> Self {
        if max == 0 {
            return Self {
                value: String::new(),
                cursor: 0,
                changed: false,
            };
        }

        let current_chars: Vec<char> = current_value.chars().collect();
        let raw_chars: Vec<char> = raw_value.chars().take(max).collect();
        let current_len = current_chars.len();
        let cursor = cursor.min(current_len);
        let range = edit_range_for_entry(cursor, current_len, max, selected_range.clone());
        let (next_chars, next_cursor) = if selected_range.is_none() && !range.is_empty() {
            if let Some(inserted_chars) =
                inserted_chars_at_cursor(&current_chars, &raw_chars, cursor)
            {
                let mut replaced_chars = current_chars.clone();
                replaced_chars.splice(range.start..range.end, inserted_chars.iter().copied());
                replaced_chars.truncate(max);
                let next_cursor = (range.start + inserted_chars.len()).min(replaced_chars.len());
                (replaced_chars, next_cursor)
            } else {
                let next_cursor = raw_value_cursor(&current_chars, &raw_chars, range, current_len);
                (raw_chars, next_cursor)
            }
        } else {
            let next_cursor = raw_value_cursor(&current_chars, &raw_chars, range, current_len);
            (raw_chars, next_cursor)
        };
        let next: String = next_chars.into_iter().collect();
        let changed = next != current_value;

        Self {
            value: next,
            cursor: next_cursor,
            changed,
        }
    }
}

fn edit_range_for_entry(
    cursor: usize,
    len: usize,
    max: usize,
    selected_range: Option<Range<usize>>,
) -> Range<usize> {
    if let Some(range) = selected_range {
        return range.start.min(len)..range.end.min(len);
    }

    let cursor = cursor.min(len);
    if len == 0 || cursor == len && len < max {
        cursor..cursor
    } else {
        let start = cursor.min(len.saturating_sub(1));
        start..(start + 1).min(len)
    }
}

fn native_selection_for(
    value: &str,
    cursor: usize,
    max: usize,
    selected_range: Option<Range<usize>>,
) -> Range<usize> {
    let len = value.chars().count();
    let range = edit_range_for_entry(cursor, len, max, selected_range);

    utf16_offset_for_char(value, range.start)..utf16_offset_for_char(value, range.end)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ValueCursorChange {
    value: String,
    cursor: usize,
}

impl ValueCursorChange {
    fn delete_backward(
        current_value: &str,
        cursor: usize,
        max: usize,
        selected_range: Option<Range<usize>>,
    ) -> Self {
        let mut chars: Vec<char> = current_value.chars().collect();

        if let Some(range) = selected_range {
            chars.drain(range.start..range.end);
            return Self {
                value: chars.into_iter().collect(),
                cursor: range.start,
            };
        }

        let cursor = cursor.min(chars.len());
        if chars.is_empty() {
            return Self {
                value: current_value.to_string(),
                cursor: 0,
            };
        }
        if cursor < chars.len() || chars.len() == max {
            let start = cursor.min(chars.len().saturating_sub(1));
            chars.remove(start);
            return Self {
                value: chars.into_iter().collect(),
                cursor: start,
            };
        }
        if cursor == 0 {
            return Self {
                value: current_value.to_string(),
                cursor: 0,
            };
        }

        chars.remove(cursor - 1);
        Self {
            value: chars.into_iter().collect(),
            cursor: cursor - 1,
        }
    }

    fn delete_forward(
        current_value: &str,
        cursor: usize,
        max: usize,
        selected_range: Option<Range<usize>>,
    ) -> Self {
        let mut chars: Vec<char> = current_value.chars().collect();

        if let Some(range) = selected_range {
            chars.drain(range.start..range.end);
            return Self {
                value: chars.into_iter().collect(),
                cursor: range.start,
            };
        }

        let cursor = cursor.min(chars.len());
        if chars.is_empty() {
            return Self {
                value: current_value.to_string(),
                cursor,
            };
        }
        if cursor >= chars.len() && chars.len() < max {
            return Self {
                value: current_value.to_string(),
                cursor,
            };
        }

        let start = cursor.min(chars.len().saturating_sub(1));
        chars.remove(start);
        Self {
            value: chars.into_iter().collect(),
            cursor: start,
        }
    }
}

fn inserted_chars_at_cursor(
    current_chars: &[char],
    raw_chars: &[char],
    cursor: usize,
) -> Option<Vec<char>> {
    if raw_chars.len() <= current_chars.len() || cursor > current_chars.len() {
        return None;
    }

    let inserted_len = raw_chars.len() - current_chars.len();
    if cursor + inserted_len > raw_chars.len()
        || raw_chars[..cursor] != current_chars[..cursor]
        || raw_chars[cursor + inserted_len..] != current_chars[cursor..]
    {
        return None;
    }

    Some(raw_chars[cursor..cursor + inserted_len].to_vec())
}

fn raw_value_cursor(
    current_chars: &[char],
    next_chars: &[char],
    range: Range<usize>,
    current_len: usize,
) -> usize {
    let replaced_len = range.len();
    let base_len = current_len.saturating_sub(replaced_len);
    if next_chars.len() >= base_len {
        (range.start + next_chars.len().saturating_sub(base_len)).min(next_chars.len())
    } else {
        current_chars
            .iter()
            .zip(next_chars)
            .take_while(|(a, b)| a == b)
            .count()
    }
}

fn utf16_offset_for_char(value: &str, index: usize) -> usize {
    value
        .chars()
        .take(index)
        .map(char::len_utf16)
        .sum::<usize>()
}

fn active_slot_index(cursor: usize, len: usize, max: usize) -> Option<usize> {
    if max == 0 {
        return None;
    }

    Some(cursor.min(len).min(max - 1))
}

fn slot_selection_range(anchor_slot: usize, focus_slot: usize, len: usize) -> Option<Range<usize>> {
    if len == 0 {
        return None;
    }

    let anchor_slot = anchor_slot.min(len - 1);
    let focus_slot = focus_slot.min(len - 1);
    let start = anchor_slot.min(focus_slot);
    let end = anchor_slot.max(focus_slot) + 1;

    Some(start..end.min(len))
}

fn slot_selection_anchor_cursor(anchor_slot: usize, focus_slot: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    if focus_slot < anchor_slot {
        (anchor_slot + 1).min(len)
    } else {
        anchor_slot.min(len)
    }
}

fn slot_selection_focus_cursor(anchor_slot: usize, focus_slot: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    if focus_slot < anchor_slot {
        focus_slot.min(len)
    } else {
        (focus_slot + 1).min(len)
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

async fn sync_input_selection(input_id: String, selection: Range<usize>) {
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
    let _ = eval.send(selection.start);
    let _ = eval.send(selection.end);
}

async fn sync_input_value_and_selection(
    input_id: String,
    value: String,
    selection: Range<usize>,
) {
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
    let _ = eval.send(selection.start);
    let _ = eval.send(selection.end);
}

#[cfg(test)]
mod tests {
    use super::{
        active_slot_index, native_selection_for, selection_range_for,
        slot_selection_anchor_cursor, slot_selection_focus_cursor, slot_selection_range,
        InputValueChange, ValueCursorChange,
    };

    fn input_change(value: &str, cursor: usize, changed: bool) -> InputValueChange {
        InputValueChange {
            value: value.to_string(),
            cursor,
            changed,
        }
    }

    fn value_cursor(value: &str, cursor: usize) -> ValueCursorChange {
        ValueCursorChange {
            value: value.to_string(),
            cursor,
        }
    }

    #[test]
    fn input_change_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "129", 6, 2, None),
            input_change("129", 3, true)
        );
    }

    #[test]
    fn input_change_replaces_at_visible_cursor() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "192", 6, 1, None),
            input_change("19", 2, true)
        );
    }

    #[test]
    fn multi_character_input_change_replaces_active_slot() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "1982", 6, 1, None),
            input_change("198", 3, true)
        );
    }

    #[test]
    fn unchanged_input_is_not_reported_as_changed() {
        assert_eq!(
            InputValueChange::from_raw_input("192", "192", 6, 3, None),
            input_change("192", 3, false)
        );
    }

    #[test]
    fn input_change_truncates_to_maxlength() {
        assert_eq!(
            InputValueChange::from_raw_input("123456", "1234569", 6, 6, None),
            input_change("123456", 6, false)
        );
    }

    #[test]
    fn input_deletion_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "1", 6, 2, None),
            input_change("1", 1, true)
        );
    }

    #[test]
    fn full_input_replacement_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "987654", 6, 0, selection_range_for(0, 2, 2)),
            input_change("987654", 6, true)
        );
    }

    #[test]
    fn full_input_replacement_with_shared_prefix_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input(
                "123456",
                "123999",
                6,
                0,
                selection_range_for(0, 6, 6),
            ),
            input_change("123999", 6, true)
        );
    }

    #[test]
    fn shorter_full_input_replacement_with_shared_prefix_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("123456", "1239", 6, 0, selection_range_for(0, 6, 6),),
            input_change("1239", 4, true)
        );
    }

    #[test]
    fn full_input_replacement_truncates_to_maxlength() {
        assert_eq!(
            InputValueChange::from_raw_input(
                "123456",
                "9876543",
                6,
                0,
                selection_range_for(0, 6, 6),
            ),
            input_change("987654", 6, true)
        );
    }

    #[test]
    fn selected_range_replacement_with_shared_digits_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input(
                "123456",
                "123996",
                6,
                3,
                selection_range_for(3, 5, 6),
            ),
            input_change("123996", 5, true)
        );
    }

    #[test]
    fn delete_backward_removes_selected_range() {
        let selected_range = selection_range_for(1, 4, 6);

        assert_eq!(
            ValueCursorChange::delete_backward("123456", 4, 6, selected_range),
            value_cursor("156", 1)
        );
    }

    #[test]
    fn delete_forward_removes_after_cursor() {
        assert_eq!(
            ValueCursorChange::delete_forward("123456", 2, 6, None),
            value_cursor("12456", 2)
        );
    }

    #[test]
    fn delete_backward_removes_active_slot_in_middle() {
        assert_eq!(
            ValueCursorChange::delete_backward("123456", 2, 6, None),
            value_cursor("12456", 2)
        );
    }

    #[test]
    fn delete_forward_removes_active_slot_at_full_end() {
        assert_eq!(
            ValueCursorChange::delete_forward("123456", 6, 6, None),
            value_cursor("12345", 5)
        );
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
    fn slot_selection_range_includes_starting_slot() {
        assert_eq!(slot_selection_range(1, 4, 6), Some(1..5));
        assert_eq!(slot_selection_range(4, 1, 6), Some(1..5));
        assert_eq!(slot_selection_range(5, 5, 6), Some(5..6));
        assert_eq!(slot_selection_range(5, 2, 2), Some(1..2));
    }

    #[test]
    fn slot_selection_cursors_track_drag_direction() {
        assert_eq!(slot_selection_anchor_cursor(1, 4, 6), 1);
        assert_eq!(slot_selection_focus_cursor(1, 4, 6), 5);
        assert_eq!(slot_selection_anchor_cursor(4, 1, 6), 5);
        assert_eq!(slot_selection_focus_cursor(4, 1, 6), 1);
    }

    #[test]
    fn native_selection_uses_utf16_offsets() {
        assert_eq!(native_selection_for("😀😃😄", 3, 4, None), 6..6);
        assert_eq!(
            native_selection_for("😀😃😄", 1, 4, selection_range_for(1, 3, 3)),
            2..6
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

    /// Optional validator. Called with the prospective new value when entering characters
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
    let mut selection_range = use_signal(|| None::<Range<usize>>);
    let mut selecting_with_pointer = use_signal(|| false);
    let mut pointer_selection_anchor_slot = use_signal(|| None::<usize>);
    let mut pointer_focus_pending = use_hook(|| CopyValue::new(false));
    let disabled = props.disabled;

    let native_selection = use_memo(move || {
        let current_value = value.read();
        let range = selection_range.cloned().and_then(|range| {
            selection_range_for(range.start, range.end, current_value.chars().count())
        });
        native_selection_for(&current_value, cursor(), maxlength(), range)
    });

    let selected_range = use_memo(move || {
        if !is_focused() {
            return None;
        }

        let len = value.read().chars().count();
        selection_range
            .cloned()
            .and_then(|range| selection_range_for(range.start, range.end, len))
    });

    let active_index = use_memo(move || {
        if !is_focused() {
            return None;
        }
        if selected_range.read().is_some() {
            return None;
        }
        let len = value.read().chars().count();
        active_slot_index(cursor(), len, maxlength())
    });

    let begin_slot_selection = use_callback(move |index: usize| {
        if disabled() {
            return;
        }

        let len = value.read().chars().count();
        let next_cursor = (index + 1).min(len);

        is_focused.set(true);
        cursor.set(next_cursor);
        selection_anchor.set(None);
        selection_range.set(None);
        pointer_selection_anchor_slot.set((index < len).then_some(index));
        selecting_with_pointer.set(true);
        pointer_focus_pending.set(true);
        focus_input(input_id());
    });

    let extend_slot_selection = use_callback(move |index: usize| {
        if !selecting_with_pointer() || disabled() {
            return;
        }

        let len = value.read().chars().count();
        let Some(anchor_slot) = pointer_selection_anchor_slot() else {
            return;
        };
        let next_cursor = slot_selection_focus_cursor(anchor_slot, index, len);
        let next_anchor = slot_selection_anchor_cursor(anchor_slot, index, len);

        cursor.set(next_cursor);
        selection_anchor.set(Some(next_anchor));
        selection_range.set(slot_selection_range(anchor_slot, index, len));
    });

    let end_slot_selection = use_callback(move |index: Option<usize>| {
        if let Some(index) = index {
            if selecting_with_pointer() && selection_range.cloned().is_none() {
                let len = value.read().chars().count();
                cursor.set(index.min(len));
                selection_anchor.set(None);
            }
        }
        selecting_with_pointer.set(false);
        pointer_selection_anchor_slot.set(None);
    });

    use_context_provider(|| OtpCtx {
        value,
        disabled: props.disabled,
        active_index,
        selected_range,
        begin_slot_selection,
        extend_slot_selection,
        end_slot_selection,
    });

    use_effect(move || {
        if !is_focused() {
            return;
        }

        let id = input_id();
        let selection = native_selection();

        spawn(async move {
            sync_input_selection(id, selection).await;
        });
    });

    rsx! {
        div {
            role: "group",
            position: "relative",
            "data-disabled": props.disabled,
            onpointerdown: move |event: Event<PointerData>| {
                if disabled() {
                    return;
                }

                event.prevent_default();
                is_focused.set(true);
                cursor.set(value.read().chars().count());
                selection_anchor.set(None);
                selection_range.set(None);
                pointer_selection_anchor_slot.set(None);
                selecting_with_pointer.set(false);
                pointer_focus_pending.set(true);
                focus_input(input_id());
            },
            onpointerup: move |_| {
                selecting_with_pointer.set(false);
                pointer_selection_anchor_slot.set(None);
            },
            onpointercancel: move |_| {
                selecting_with_pointer.set(false);
                pointer_selection_anchor_slot.set(None);
                pointer_focus_pending.set(false);
            },
            onpointerleave: move |_| {
                selecting_with_pointer.set(false);
                pointer_selection_anchor_slot.set(None);
            },
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
                value,

                style: "position:absolute;z-index:20;top:0;left:0;right:0;bottom:0;width:100%;height:100%;opacity:0;color:transparent;background:transparent;caret-color:transparent;outline:none;border:none;padding:0;margin:0;text-align:center;font-family:inherit;font-size:inherit;cursor:text;user-select:none;pointer-events:none;",

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
                    let selected = selection_range.cloned();
                    let mut new_cursor = cursor().min(len);
                    let mut next_selection = selected.clone();
                    let mut next_anchor = selection_anchor();

                    match key {
                        Key::ArrowLeft => {
                            e.prevent_default();
                            if mods.shift() {
                                let anchor = next_anchor.unwrap_or(new_cursor);
                                new_cursor = new_cursor.saturating_sub(1);
                                next_anchor = Some(anchor);
                                next_selection = selection_range_for(anchor, new_cursor, len);
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
                                next_selection = selection_range_for(anchor, new_cursor, len);
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
                                next_selection = selection_range_for(anchor, new_cursor, len);
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
                                next_selection = selection_range_for(anchor, new_cursor, len);
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
                                let change = ValueCursorChange::delete_backward(
                                    &current_value,
                                    new_cursor,
                                    max,
                                    selected,
                                );
                                apply_value_change(
                                    &current_value,
                                    change.value,
                                    max,
                                    set_value,
                                    on_complete,
                                );
                                new_cursor = change.cursor;
                            }
                            next_anchor = None;
                            next_selection = None;
                        }
                        Key::Delete => {
                            e.prevent_default();
                            let current_value = value.read().clone();
                            let change = ValueCursorChange::delete_forward(
                                &current_value,
                                new_cursor,
                                max,
                                selected,
                            );
                            apply_value_change(
                                &current_value,
                                change.value,
                                max,
                                set_value,
                                on_complete,
                            );
                            new_cursor = change.cursor;
                            next_anchor = None;
                            next_selection = None;
                        }
                        Key::Character(ref s)
                            if (mods.ctrl() || mods.meta()) && s.eq_ignore_ascii_case("a") =>
                        {
                            e.prevent_default();
                            new_cursor = 0;
                            next_anchor = Some(0);
                            next_selection = selection_range_for(0, len, len);
                        }
                        Key::Character(ref s)
                            if s.chars().count() == 1
                                && !mods.ctrl()
                                && !mods.meta()
                                && !mods.alt() =>
                        {
                            e.prevent_default();
                            let range = edit_range_for_entry(new_cursor, len, max, selected);
                            if let Some(c) = s.chars().next() {
                                let mut next_chars = chars.clone();
                                next_chars.splice(range.start..range.end, [c]);
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
                                new_cursor = (range.start + 1).min(next_chars.len());
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
                    if selection_range.cloned() != next_selection {
                        selection_range.set(next_selection);
                    }
                    selecting_with_pointer.set(false);
                },

                oninput: move |e: FormEvent| {
                    let raw = e.value();
                    let max = maxlength();
                    let current_value = value.read().clone();
                    let selected = selection_range.cloned();
                    let current_cursor = cursor().min(current_value.chars().count());
                    let change = InputValueChange::from_raw_input(
                        &current_value,
                        &raw,
                        max,
                        current_cursor,
                        selected.clone(),
                    );
                    if change.changed {
                        if let Some(validate) = validate {
                            if !validate.call(change.value.clone()) {
                                let id = input_id();
                                let selection = native_selection_for(
                                    &current_value,
                                    current_cursor,
                                    max,
                                    selected,
                                );
                                spawn(async move {
                                    sync_input_value_and_selection(
                                        id,
                                        current_value,
                                        selection,
                                    )
                                    .await;
                                });
                                return;
                            }
                        }
                    }
                    let next_cursor = change.cursor;
                    let next_value = change.value;
                    if raw != next_value {
                        let id = input_id();
                        let value_to_sync = next_value.clone();
                        let selection =
                            native_selection_for(&value_to_sync, next_cursor, max, None);
                        spawn(async move {
                            sync_input_value_and_selection(id, value_to_sync, selection).await;
                        });
                    } else {
                        let id = input_id();
                        let selection =
                            native_selection_for(&next_value, next_cursor, max, None);
                        spawn(async move {
                            sync_input_selection(id, selection).await;
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
                    pointer_selection_anchor_slot.set(None);
                    selecting_with_pointer.set(false);
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
                    {
                        cursor.set(value.read().chars().count());
                        selection_anchor.set(None);
                        selection_range.set(None);
                    }
                },
                onblur: move |_| {
                    is_focused.set(false);
                    selecting_with_pointer.set(false);
                    pointer_selection_anchor_slot.set(None);
                    selection_anchor.set(None);
                    selection_range.set(None);
                    pointer_focus_pending.set(false);
                },
            }
        }
    }
}

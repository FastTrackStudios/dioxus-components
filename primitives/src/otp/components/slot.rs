use super::super::context::{OtpCtx, SlotBounds};
use dioxus::prelude::*;
use std::rc::Rc;

async fn sync_slot_bounds(
    mut slot_bounds: Signal<Vec<Option<SlotBounds>>>,
    index: usize,
    mounted: Rc<MountedData>,
) {
    let Ok(rect) = mounted.get_client_rect().await else {
        return;
    };

    let mut bounds = slot_bounds.write();
    if bounds.len() <= index {
        bounds.resize(index + 1, None);
    }
    bounds[index] = Some(SlotBounds {
        left: rect.origin.x,
        right: rect.origin.x + rect.size.width,
    });
}

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
/// - `data-selected`: `true` when this slot's character is selected.
/// - `data-selection-start`: `true` when this slot is the first selected slot.
/// - `data-selection-end`: `true` when this slot is the last selected slot.
/// - `data-empty`: `true` when no character has been entered at this position.
/// - `data-disabled`: mirrors the parent's disabled state.
/// - `data-char`: the current character at this position (empty when none).
#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    let mut ctx: OtpCtx = use_context();
    let index = props.index;
    let mut slot_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let char_at = use_memo(move || {
        ctx.value
            .read()
            .chars()
            .nth(index())
            .map(|c| c.to_string())
            .unwrap_or_default()
    });
    let is_active = use_memo(move || ctx.active_index.cloned() == Some(index()));
    let is_selected = use_memo(move || {
        ctx.selected_range
            .cloned()
            .is_some_and(|range| range.contains(index()))
    });
    let is_selection_start = use_memo(move || {
        ctx.selected_range
            .cloned()
            .is_some_and(|range| range.starts_at(index()))
    });
    let is_selection_end = use_memo(move || {
        ctx.selected_range
            .cloned()
            .is_some_and(|range| range.ends_at(index()))
    });
    let is_empty = use_memo(move || char_at.read().is_empty());

    rsx! {
        div {
            role: "presentation",
            aria_hidden: "true",
            "data-active": is_active,
            "data-selected": is_selected,
            "data-selection-start": is_selection_start,
            "data-selection-end": is_selection_end,
            "data-empty": is_empty,
            "data-disabled": ctx.disabled,
            "data-char": char_at,
            onmounted: move |event: Event<MountedData>| {
                let mounted = event.data();
                let index = index();
                slot_ref.set(Some(mounted.clone()));
                {
                    let mut slot_refs = ctx.slot_refs.write();
                    if slot_refs.len() <= index {
                        slot_refs.resize(index + 1, None);
                    }
                    slot_refs[index] = Some(mounted.clone());
                }
                async move {
                    sync_slot_bounds(ctx.slot_bounds, index, mounted).await;
                }
            },
            onresize: move |_| async move {
                let Some(mounted) = slot_ref() else {
                    return;
                };
                sync_slot_bounds(ctx.slot_bounds, index(), mounted).await;
            },
            ..props.attributes,

            {char_at}
            {props.children}
        }
    }
}

//! Shared state for the one-time-password components.

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(super) struct OtpCtx {
    pub value: Memo<String>,
    pub disabled: ReadSignal<bool>,
    pub active_index: Memo<Option<usize>>,
    pub selected_range: Memo<Option<SelectionRange>>,
    pub slot_bounds: Signal<Vec<Option<SlotBounds>>>,
    pub slot_refs: Signal<Vec<Option<std::rc::Rc<MountedData>>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SelectionRange {
    pub start: usize,
    pub end: usize,
}

impl SelectionRange {
    pub fn new(anchor: usize, focus: usize, len: usize) -> Option<Self> {
        let start = anchor.min(focus).min(len);
        let end = anchor.max(focus).min(len);

        (start < end).then_some(Self { start, end })
    }

    pub fn contains(self, index: usize) -> bool {
        self.start <= index && index < self.end
    }

    pub fn starts_at(self, index: usize) -> bool {
        self.start == index
    }

    pub fn ends_at(self, index: usize) -> bool {
        self.end == index + 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SlotBounds {
    pub left: f64,
    pub right: f64,
}

impl SlotBounds {
    pub fn center(self) -> f64 {
        self.left + (self.right - self.left) / 2.0
    }
}

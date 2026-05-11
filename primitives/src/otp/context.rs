//! Shared state for the one-time-password components.

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(super) struct OtpCtx {
    pub value: Memo<String>,
    pub disabled: ReadSignal<bool>,
    pub active_index: Memo<Option<usize>>,
}

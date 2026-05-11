//! Component definitions for the one-time-password primitive.

pub mod group;
pub mod input;
pub mod separator;
pub mod slot;

pub use group::{OneTimePasswordGroup, OneTimePasswordGroupProps};
pub use input::{OneTimePasswordInput, OneTimePasswordInputProps};
pub use separator::{OneTimePasswordSeparator, OneTimePasswordSeparatorProps};
pub use slot::{OneTimePasswordSlot, OneTimePasswordSlotProps};

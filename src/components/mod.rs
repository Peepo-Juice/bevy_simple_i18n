// mod i18n_font;
// #[cfg(feature = "numbers")]
// mod i18n_number;
mod i18n_text;
mod i18n_text_2d;
pub mod new;
pub mod utils;

// use crate::components::utils::{f64_to_fd, translate_by_key};
// pub use i18n_font::*;
// #[cfg(feature = "numbers")]
// pub use i18n_number::*;
pub use i18n_text::*;
pub use i18n_text_2d::*;

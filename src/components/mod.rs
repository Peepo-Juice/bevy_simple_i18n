// mod i18n_font;
// #[cfg(feature = "numbers")]
// mod i18n_number;

pub mod i18n_font_size;
pub mod i18n_font;
pub mod i18n_locale;
pub mod i18n_string;
pub mod i18n_text;
pub mod i18n_text_2d;
pub mod i18n_translation;
pub mod utils;

// use crate::components::utils::{f64_to_fd, translate_by_key};
// pub use i18n_font::*;
// #[cfg(feature = "numbers")]
// pub use i18n_number::*;
pub use i18n_text::*;
pub use i18n_text_2d::*;

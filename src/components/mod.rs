mod i18n_font;
#[cfg(feature = "numbers")]
mod i18n_number;
mod i18n_text;
mod i18n_text_2d;
pub mod new;
pub mod utils;

// use crate::components::utils::{f64_to_fd, translate_by_key};
pub use i18n_font::*;
#[cfg(feature = "numbers")]
pub use i18n_number::*;
pub use i18n_text::*;
pub use i18n_text_2d::*;

// pub trait I18nComponent {
//     fn locale(&self) -> Option<&String>;
//     fn set_locale(&mut self, new_locale: String);

//     fn key(&self) -> &String;

//     fn args(&self) -> &Vec<(String, InterpolationType)>;
//     fn args_mut(&mut self) -> &mut Vec<(String, InterpolationType)>;

//     /// Internal method that wraps the `rust_i18n::t!` macro
//     fn translate(&self) -> String {
//         translate_by_key(&self.get_locale(), self.key(), self.args())
//     }

//     /// If set, returns the locale of the component, otherwise the global locale
//     fn get_locale(&self) -> String {
//         if let Some(internal_locale) = self.locale() {
//             internal_locale.to_string()
//         } else {
//             rust_i18n::locale().to_string()
//         }
//     }

//     /// Set the locale for this specific translation
//     fn with_locale(mut self, locale: impl Into<String>) -> Self
//     where
//         Self: Sized,
//     {
//         self.set_locale(locale.into());
//         self
//     }

//     /// Add a standard string interpolation argument to the translation key
//     ///
//     /// This method can be called as many times as needed
//     fn with_arg(mut self, key: impl Into<String>, value: impl ToString) -> Self
//     where
//         Self: Sized,
//     {
//         self.args_mut()
//             .push((key.into(), InterpolationType::String(value.to_string())));
//         self
//     }

//     #[cfg(feature = "numbers")]
//     /// Add a number interpolation argument to the translation key
//     ///
//     /// This method can be called as many times as needed
//     fn with_num_arg(mut self, key: impl Into<String>, value: impl Into<f64>) -> Self
//     where
//         Self: Sized,
//     {
//         self.args_mut().push((
//             key.into(),
//             InterpolationType::Number(f64_to_fd(value.into())),
//         ));
//         self
//     }
// }

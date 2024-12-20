use super::{
    i18n_font::I18nFont, i18n_font_size::I18nFontSize, i18n_translation::I18nTranslation,
    InterpolationType,
};
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(I18nFont, I18nFontSize(|| I18nFontSize(12.)), I18nTranslation)]
pub struct I18nString {
    /// Translation key for i18n
    pub key: String,
    /// Interpolation arguments for the translation key
    pub args: Vec<(String, InterpolationType)>,
}
impl I18nString {
    pub fn new(str: &str) -> Self {
        Self {
            key: str.into(),
            args: vec![],
        }
    }

    pub fn with_arg(mut self, key: &str, value: impl ToString) -> Self {
        self.args
            .push((key.into(), InterpolationType::String(value.to_string())));
        self
    }

    #[cfg(feature = "numbers")]
    /// Add a number interpolation argument to the translation key
    ///
    /// This method can be called as many times as needed
    pub fn with_num_arg(mut self, key: &str, value: impl Into<f64>) -> Self {
        self.args.push((
            key.into(),
            InterpolationType::Number(super::utils::f64_to_fd(value.into())),
        ));
        self
    }
}

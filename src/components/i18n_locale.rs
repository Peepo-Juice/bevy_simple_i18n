use bevy::prelude::*;

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct I18nLocale(String);
impl I18nLocale {
    pub fn new(locale: &str) -> Self {
        Self(locale.into())
    }

    pub fn set_locale(&mut self, locale: String) {
        self.0 = locale;
    }
}
pub trait LocaleExt {
    fn locale(&self) -> String;
}

impl LocaleExt for Option<&I18nLocale> {
    fn locale(&self) -> String {
        if let Some(internal_locale) = &self {
            internal_locale.0.to_string()
        } else {
            rust_i18n::locale().to_string()
        }
    }
}

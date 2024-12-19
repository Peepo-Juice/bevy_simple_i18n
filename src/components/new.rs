use crate::prelude::utils::translate_by_key;

use super::utils::f64_to_fd;
use super::InterpolationType;
use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use fixed_decimal::FixedDecimal;

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(I18nFont, I18nTranslation)]
pub struct I18nString {
    /// Translation key for i18n
    pub key: String,
    /// Interpolation arguments for the translation key
    pub args: Vec<(String, InterpolationType)>,
}
impl I18nString {
    pub fn new(str: impl Into<String>) -> Self {
        Self {
            key: str.into(),
            args: vec![],
        }
    }

    pub fn with_arg(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.args
            .push((key.into(), InterpolationType::String(value.to_string())));
        self
    }

    #[cfg(feature = "numbers")]
    /// Add a number interpolation argument to the translation key
    ///
    /// This method can be called as many times as needed
    pub fn with_num_arg(mut self, key: impl Into<String>, value: impl Into<f64>) -> Self {
        self.args.push((
            key.into(),
            InterpolationType::Number(super::utils::f64_to_fd(value.into())),
        ));
        self
    }
}

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(I18nFont, I18nTranslation)]
pub struct I18nNumber {
    #[reflect(ignore)]
    pub(crate) fixed_decimal: FixedDecimal,
}
impl I18nNumber {
    pub fn new(number: impl Into<f64>) -> Self {
        Self {
            fixed_decimal: f64_to_fd(number.into()),
        }
    }
}

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct I18nFont {
    pub family: String,
    pub size: f32,
}

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct I18nLocale(String);
impl I18nLocale {
    pub fn new(locale: impl Into<String>) -> Self {
        Self(locale.into())
    }

    pub fn set_locale(&mut self, locale: String) {
        self.0 = locale;
    }
}
pub trait LocalExt {
    fn locale(&self) -> String;
}

impl LocalExt for Option<&I18nLocale> {
    fn locale(&self) -> String {
        if let Some(internal_locale) = &self {
            internal_locale.0.to_string()
        } else {
            // println!("global {:?} ",  rust_i18n::locale().to_string());
            rust_i18n::locale().to_string()
        }
    }
}

#[derive(Event, Reflect, Debug, Clone)]
pub struct UpdatedTranslation(pub Entity);

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(on_add = on_add_translation)]
pub struct I18nTranslation(String);
impl I18nTranslation {
    pub fn new(new: String) -> Self {
        Self(new)
    }

    pub fn set(&mut self, new: String) {
        self.0 = new;
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

fn on_add_translation(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let string = world
        .get::<I18nString>(entity)
        .expect("I18nString requires this component");
    let locale = world.get::<I18nLocale>(entity);

    let translated_txt = translate_by_key(&locale.locale(), &string.key, &string.args);

    let mut translated = world.get_mut::<I18nTranslation>(entity).unwrap();
    translated.set(translated_txt);

    world.trigger_targets(UpdatedTranslation(entity), entity);
}

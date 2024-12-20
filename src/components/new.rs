use crate::prelude::utils::translate_by_key;
use crate::prelude::FontManager;

use super::InterpolationType;
use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::reflect::Reflect;

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

#[derive(Event, Reflect, Debug, Clone)]
pub struct UpdatedFont(pub Handle<Font>);

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(on_add = on_add_font)]
pub struct I18nFont(String);
impl I18nFont {
    pub fn new(family: impl Into<String>) -> Self {
        Self(family.into())
    }

    pub fn family(&self) -> &String {
        &self.0
    }
}
fn on_add_font(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let font = world
        .get::<I18nFont>(entity)
        .expect("I18nString requires this component");
    let locale = world.get::<I18nLocale>(entity);

    let font_manager = world.resource::<FontManager>();
    let font = font_manager.get(&font.family(), locale.locale());

    world.trigger_targets(UpdatedFont(font), entity);
}

#[derive(Event, Reflect, Debug, Clone)]
pub struct UpdatedFontSize(pub f32);

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(on_add = on_add_font_size)]
pub struct I18nFontSize(pub f32);
impl I18nFontSize {
    pub fn new(size: f32) -> Self {
        Self(size.into())
    }
}
fn on_add_font_size(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let font = world
        .get::<I18nFontSize>(entity)
        .expect("I18nString requires this component");

    world.trigger_targets(UpdatedFontSize(font.0), entity);
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
pub trait LocaleExt {
    fn locale(&self) -> String;
}

impl LocaleExt for Option<&I18nLocale> {
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
pub struct UpdatedTranslation(pub String);

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(on_add = on_add_translation)]
pub struct I18nTranslation(String);
impl I18nTranslation {
    pub fn new(new: String) -> Self {
        Self(new)
    }

    pub fn set(&mut self, new: impl Into<String>) {
        self.0 = new.into();
    }

    pub fn translation(&self) -> &str {
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
    translated.set(&translated_txt);

    world.trigger_targets(UpdatedTranslation(translated_txt), entity);
}

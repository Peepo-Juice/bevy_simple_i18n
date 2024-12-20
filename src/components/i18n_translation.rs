use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use super::{
    i18n_locale::{I18nLocale, LocaleExt},
    i18n_string::I18nString,
    utils::translate_by_key,
};

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

    pub fn set(&mut self, new: &str) {
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

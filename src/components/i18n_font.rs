use super::i18n_locale::{I18nLocale, LocaleExt};
use crate::prelude::FontManager;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

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

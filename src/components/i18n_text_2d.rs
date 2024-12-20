use super::{
    i18n_font::UpdatedFont, i18n_font_size::UpdatedFontSize, i18n_string::I18nString,
    i18n_translation::UpdatedTranslation,
};
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    state::commands,
};

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(Text2d, I18nString)]
#[component(on_add = I18nText2d::on_add_hook)]
pub struct I18nText2d;
impl I18nText2d {
    pub fn new(key: &str) -> (Self, I18nString) {
        (I18nText2d, I18nString::new(key))
    }

    fn on_add_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        world
            .commands()
            .entity(entity)
            .observe(Self::update_translation);

        world
            .commands()
            .entity(entity)
            .observe(Self::updated_font_size);
        world.commands().entity(entity).observe(Self::update_font);
    }

    fn update_translation(
        trigger: Trigger<UpdatedTranslation>,
        mut commands: Commands,
        mut text_q: Query<&mut Text2d>,
    ) {
        let e = trigger.entity();

        if let Ok(mut text) = text_q.get_mut(e) {
            text.0 = trigger.0.clone();
        } /* else {
              // i dont think this ever runs, because of the required component, so might be safe to remove
              commands.entity(e).insert(Text2d::new(trigger.0.clone()));
          } */
    }

    fn updated_font_size(trigger: Trigger<UpdatedFontSize>, mut text_q: Query<&mut TextFont>) {
        let e = trigger.entity();

        if let Ok(mut bevy_font) = text_q.get_mut(e) {
            bevy_font.font_size = trigger.event().0;
        }
    }

    fn update_font(
        trigger: Trigger<UpdatedFont>,
        mut font_q: Query<&mut TextFont>,
        // a: Res<Assets<Font>>,
    ) {
        let e = trigger.entity();
        let new_font = trigger.event().0.clone();

        if let Ok(mut bevy_font) = font_q.get_mut(e) {
            bevy_font.font = new_font;
        }
    }
}
